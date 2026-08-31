#[path = "build/mod.rs"]
pub(crate) mod build;

use std::{env, path::Path};

/// メイン関数
fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR").unwrap();

    // SVGのディレクトリを取得
    let svg_dir = manifest_path.join("assets/svg");
    println!("cargo:rerun-if-changed={}", svg_dir.display());
    build::svg::generate_svg_generated(&svg_dir, &out_dir);

    // バンドル用アイコンと Windows の実行ファイルアイコンを生成する
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    build::appicon::generate_icons(manifest_path, &out_dir);
}
