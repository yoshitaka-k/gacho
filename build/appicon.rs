use std::{
    env,
    fs::{self, File},
    path::Path,
};

/// アプリアイコンを生成する
///
/// `assets/icon.png` は macOS のアイコングリッドに合わせ、
/// 1024px キャンバスの中央 824px に収めてある（四方 100px の透明余白）。
/// キャンバスいっぱいに描くと Launchpad / Dock で他アプリより大きく見える。
///
/// * `manifest_path` - CARGO_MANIFEST_DIR
/// * `out_dir` - OUT_DIR
pub(crate) fn generate_icons(manifest_path: &Path, out_dir: &str) {
    let png_path = manifest_path.join("assets/icon.png");
    println!("cargo:rerun-if-changed={}", png_path.display());

    let imaged = image::open(&png_path).expect("failed to open assets/icon.png");

    write_bundle_icons(&imaged, manifest_path);
    embed_windows_icon(&imaged, out_dir);
}

/// cargo-bundle が macOS ICNS を作れるサイズの PNG を書き出す
///
/// 1024x1024 を density=1 のまま渡すと `No matching IconType` になる。
/// 1024px は ICNS では 512@2x として扱う必要がある。
fn write_bundle_icons(imaged: &image::DynamicImage, manifest_path: &Path) {
    let out_dir = manifest_path.join("target/bundle-icons");
    fs::create_dir_all(&out_dir).expect("failed to create target/bundle-icons");

    // 論理サイズ, 密度
    let sizes = [
        (16, 1), (16, 2),
        (32, 1), (32, 2),
        (128, 1), (128, 2),
        (256, 1), (256, 2),
        (512, 1), (512, 2),
    ];

    for (size, density) in sizes {
        let pixel = size * density;
        let resized = imaged.resize_exact(pixel, pixel, image::imageops::FilterType::Lanczos3);
        let name = if density == 2 {
            format!("{size}x{size}@2x.png")
        } else {
            format!("{size}x{size}.png")
        };
        resized
            .save(out_dir.join(&name))
            .unwrap_or_else(|err| panic!("failed to write {name}: {err}"));
    }
}

/// Windows 向けに ICO を生成し、実行ファイルへ埋め込む
/// * `imaged` - ソース画像
/// * `out_dir` - OUT_DIR
fn embed_windows_icon(imaged: &image::DynamicImage, out_dir: &str) {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    let ico_path = Path::new(out_dir).join("icon.ico");
    write_ico(imaged, &ico_path);

    winresource::WindowsResource::new()
        .set_icon(ico_path.to_str().expect("invalid ico path"))
        .compile()
        .expect("failed to compile windows resources");
}

/// PNG から複数サイズの ICO を書き出す
/// * `imaged` - ソース画像
/// * `ico_path` - 出力 ICO
fn write_ico(imaged: &image::DynamicImage, ico_path: &Path) {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for size in [256_u32, 128, 64, 48, 32, 16] {
        let resized = imaged.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let icon_image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).expect("failed to encode ico entry"));
    }

    let file = File::create(ico_path).expect("failed to create icon.ico");
    icon_dir.write(file).expect("failed to write icon.ico");
}
