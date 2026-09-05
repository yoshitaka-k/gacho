//! Finder の「このアプリケーションで開く」や、アイコンへのドロップは
//! Windows / Linux と違って argv に乗らない。
//! `kAEOpenDocuments` が AppKit 経由で `application:openURLs:` に届く。
//!
//! winit のアプリデリゲートはそのメソッドを持たないため、起動直後の
//! `NSApplicationWillFinishLaunching` で既存クラスへ実装を足す。

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use objc2::ffi::class_addMethod;
use objc2::runtime::{AnyObject, Imp, Sel};
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::NSApplication;
use objc2_foundation::{NSArray, NSString, NSURL};

/// 「このアプリで開く」で届いたパス。描画ループが取り出す。
fn queue() -> &'static Mutex<Vec<PathBuf>> {
    static Q: OnceLock<Mutex<Vec<PathBuf>>> = OnceLock::new();
    Q.get_or_init(|| Mutex::new(Vec::new()))
}

/// アプリ起動後に egui のコンテキストを覚え、待機中でも再描画できるようにする。
fn ctx_slot() -> &'static OnceLock<egui::Context> {
    static C: OnceLock<egui::Context> = OnceLock::new();
    &C
}

fn deliver(paths: Vec<PathBuf>) {
    if paths.is_empty() {
        return;
    }

    if let Ok(mut q) = queue().lock() {
        q.extend(paths);
    }

    if let Some(ctx) = ctx_slot().get() {
        ctx.request_repaint();
    }
}

/// 描画ループから、未処理のパスを取り出す。
pub fn take_opened_files() -> Vec<PathBuf> {
    match queue().lock() {
        Ok(mut q) => std::mem::take(&mut *q),
        Err(_) => Vec::new(),
    }
}

/// eframe の生成時に呼ぶ。起動後の「開く」で画面を起こすために使う。
pub fn set_ctx(ctx: egui::Context) {
    let _ = ctx_slot().set(ctx);
}

/// `eframe::run_native` の前に呼ぶ。
/// winit がデリゲートを付けた直後、開くイベントが届く前にメソッドを足す。
pub fn install() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        // SAFETY: 通知名は静的 C 文字列。オブザーバはプロセス終了まで残す。
        let name = unsafe {
            CFStringCreateWithCString(
                std::ptr::null(),
                c"NSApplicationWillFinishLaunchingNotification".as_ptr(),
                K_CF_STRING_ENCODING_UTF8,
            )
        };
        if name.is_null() {
            log::error!("macOS: failed to create launch notification name");
            return;
        }

        unsafe {
            CFNotificationCenterAddObserver(
                CFNotificationCenterGetLocalCenter(),
                std::ptr::null(),
                on_will_finish_launching,
                name,
                std::ptr::null(),
                CF_NOTIFICATION_DELIVER_IMMEDIATELY,
            );
        }
    });
}

extern "C" fn on_will_finish_launching(
    _center: *mut std::ffi::c_void,
    _observer: *mut std::ffi::c_void,
    _name: *const std::ffi::c_void,
    _object: *const std::ffi::c_void,
    _user_info: *const std::ffi::c_void,
) {
    inject_open_urls_method();
}

fn inject_open_urls_method() {
    let Some(mtm) = MainThreadMarker::new() else {
        log::error!("macOS: open-handler is not on the main thread");
        return;
    };

    let app = NSApplication::sharedApplication(mtm);
    let Some(delegate) = app.delegate() else {
        log::error!("macOS: no app delegate at willFinishLaunching");
        return;
    };

    let cls = AnyObject::class(delegate.as_ref());
    // SAFETY: IMP は「catch-all」型なので、実際の application:openURLs: 署名へ変換する。
    let imp: Imp = unsafe {
        std::mem::transmute::<
            unsafe extern "C-unwind" fn(
                *mut AnyObject,
                Sel,
                *mut AnyObject,
                *mut NSArray<NSURL>,
            ),
            Imp,
        >(handle_open_urls)
    };

    // SAFETY: メインスレッドで、winit のデリゲートクラスへメソッドを足す。
    let added = unsafe {
        class_addMethod(
            std::ptr::from_ref(cls).cast_mut(),
            sel!(application:openURLs:),
            imp,
            c"v@:@@".as_ptr(),
        )
    };

    if !added.as_bool() {
        log::warn!("macOS: app delegate already implements application:openURLs:");
    }
}

unsafe extern "C-unwind" fn handle_open_urls(
    _this: *mut AnyObject,
    _cmd: Sel,
    _app: *mut AnyObject,
    urls: *mut NSArray<NSURL>,
) {
    // SAFETY: AppKit が渡す NSArray<NSURL> は、この呼び出しの間だけ有効。
    deliver(unsafe { ns_urls_to_paths(urls) });
}

/// # Safety
/// `urls` は有効な `NSArray<NSURL>` か null。
unsafe fn ns_urls_to_paths(urls: *mut NSArray<NSURL>) -> Vec<PathBuf> {
    // SAFETY: 呼び出し側が有効な NSArray か null を保証する。
    let Some(urls) = (unsafe { urls.as_ref() }) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for i in 0..urls.count() {
        let url = urls.objectAtIndex(i);
        if let Some(path) = url.path() {
            out.push(nsstring_to_path(&path));
        }
    }
    out
}

fn nsstring_to_path(s: &NSString) -> PathBuf {
    PathBuf::from(s.to_string())
}

const K_CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;
const CF_NOTIFICATION_DELIVER_IMMEDIATELY: isize = 4;

type CFNotificationCallback = extern "C" fn(
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
    *const std::ffi::c_void,
    *const std::ffi::c_void,
    *const std::ffi::c_void,
);

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFNotificationCenterGetLocalCenter() -> *mut std::ffi::c_void;
    fn CFNotificationCenterAddObserver(
        center: *mut std::ffi::c_void,
        observer: *const std::ffi::c_void,
        callback: CFNotificationCallback,
        name: *const std::ffi::c_void,
        object: *const std::ffi::c_void,
        suspension_behavior: isize,
    );
    fn CFStringCreateWithCString(
        alloc: *const std::ffi::c_void,
        c_str: *const std::ffi::c_char,
        encoding: u32,
    ) -> *const std::ffi::c_void;
}
