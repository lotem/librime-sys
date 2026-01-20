use std::ffi::OsStr;
use std::sync::RwLock;
use libloading::{Library, Symbol};
use std::ptr;

struct RimeApiPtr(*mut crate::RimeApi);
unsafe impl Send for RimeApiPtr {}
unsafe impl Sync for RimeApiPtr {}

lazy_static::lazy_static! {
    static ref LIB: RwLock<Option<Library>> = RwLock::new(None);
    static ref RIME_API: RwLock<RimeApiPtr> = RwLock::new(RimeApiPtr(ptr::null_mut()));
}

/// 加載 librime 動態庫，並獲取 rime_api_t 指針
#[cfg(all(feature = "dynload", target_os = "windows"))]
pub fn load_librime<P: AsRef<OsStr>>(path: P) -> Result<(), String> {
    let mut lib_guard = LIB.write().unwrap();
    let already_loaded = lib_guard.is_some();
    if already_loaded {
        return Ok(());
    }
    let lib = unsafe { Library::new(path) }.map_err(|e| e.to_string())?;
    unsafe {
        let get_api: Symbol<unsafe extern "C" fn() -> *mut crate::RimeApi> =
            lib.get(b"rime_get_api\0").map_err(|e| e.to_string())?;
        let api_ptr = get_api();
        let mut api_guard = RIME_API.write().unwrap();
        *api_guard = RimeApiPtr(api_ptr);
    }
    *lib_guard = Some(lib);
    Ok(())
}

#[cfg(all(feature = "dynload", target_os = "windows"))]
pub fn librime_loaded() -> bool {
    let lib_guard = LIB.read().unwrap();
    lib_guard.is_some()
}

/// 卸載 librime 動態庫
#[cfg(all(feature = "dynload", target_os = "windows"))]
pub fn unload_librime() {
    let mut api_guard = RIME_API.write().unwrap();
    *api_guard = RimeApiPtr(ptr::null_mut());
    drop(api_guard);
    
    let mut lib_guard = LIB.write().unwrap();
    if let Some(lib) = lib_guard.take() {
        drop(lib_guard);
        drop(lib);
    }
}

/// 取得 rime_api_t 指針
pub fn get_rime_api() -> *mut crate::RimeApi {
    RIME_API.read().unwrap().0
}
