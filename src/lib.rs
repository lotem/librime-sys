#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(all(feature = "dynload", target_os = "windows"))]
pub mod dynload;

#[cfg(all(feature = "dynload", target_os = "windows"))]
pub const IS_DYNAMIC_LOAD: bool = true;
#[cfg(not(all(feature = "dynload", target_os = "windows")))]
pub const IS_DYNAMIC_LOAD: bool = false;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(feature = "dynload", target_os = "windows"))]
#[no_mangle]
pub unsafe fn rime_get_api() -> *mut RimeApi {
    crate::dynload::get_rime_api()
}

#[macro_export]
macro_rules! rime_struct_new {
    () => {
        unsafe { std::mem::zeroed() }
    };
}

#[macro_export]
macro_rules! rime_call {
    ( $api_struct:expr, $api_fn:ident $(, $arg:expr)* ) => {
        {
            let api_fn = $api_struct.$api_fn.expect(
                format!("missing api function: {}.{}",
                        stringify!($api_struct),
                        stringify!($api_fn)
                ).as_str()
            );
            unsafe { api_fn($($arg),*) }
        }
    };
}

#[cfg(not(all(feature = "dynload", target_os = "windows")))]
#[macro_export]
macro_rules! rime_api_call {
    ( $api_fn:ident $(, $arg:expr)* ) => {
        {
            let rime_api = unsafe { $crate::rime_get_api() };
            $crate::rime_call!(unsafe { *rime_api }, $api_fn $(, $arg)*)
        }
    };
}

#[cfg(all(feature = "dynload", target_os = "windows"))]
#[macro_export]
macro_rules! rime_api_call {
    ( $api_fn:ident $(, $arg:expr)* ) => {
        {
            if !$crate::dynload::librime_loaded() {
                $crate::dynload::load_librime($crate::rime_lib_name!()).expect("load_librime failed");
            }
            let rime_api = unsafe { $crate::rime_get_api() };
            assert!(!rime_api.is_null(), "Rime API not loaded. Please call load_librime first.");
            $crate::rime_call!(unsafe { *rime_api }, $api_fn $(, $arg)*)
        }
    };
}

#[cfg(not(all(feature = "dynload", target_os = "windows")))]
#[macro_export]
macro_rules! rime_module_call {
    ( $module:expr => $api_type:ty, $api_fn:ident $(, $arg:expr)* ) => {
        {
            let module_api = $crate::rime_call!(unsafe { *$module }, get_api) as *const $api_type;
            $crate::rime_call!(unsafe { *module_api }, $api_fn $(, $arg)*)
        }
    };
}

#[cfg(all(feature = "dynload", target_os = "windows"))]
#[macro_export]
macro_rules! rime_module_call {
    ( $module:expr => $api_type:ty, $api_fn:ident $(, $arg:expr)* ) => {
        {
            if !$crate::dynload::librime_loaded() {
                $crate::dynload::load_librime($crate::rime_lib_name!()).expect("load_librime failed");
            }
            let module_api = $crate::rime_call!(unsafe { *$module }, get_api) as *const $api_type;
            $crate::rime_call!(unsafe { *module_api }, $api_fn $(, $arg)*)
        }
    };
}

#[cfg(all(feature = "dynload", target_os = "windows"))]
#[macro_export]
macro_rules! rime_lib_name {
    () => {
        if cfg!(target_os = "linux") {
            "librime.so".to_string()
        } else if cfg!(target_os = "macos") {
            "librime.dylib".to_string()
        } else if cfg!(target_os = "windows") {
            "rime.dll".to_string()
        } else {
            panic!("Unsupported platform")
        }
    };
}

#[cfg(all(feature = "dynload", target_os = "windows"))]
#[macro_export]
macro_rules! load_librime {
    () => {
        dynload::load_librime(rime_lib_name!()).expect("load_librime failed");
    }
}

#[cfg(all(feature = "dynload", target_os = "windows"))]
#[macro_export]
macro_rules! librime_loaded {
    () => {
        dynload::librime_loaded()
    }
}

#[cfg(feature = "dynload")]
#[macro_export]
macro_rules! unload_librime {
    () => {
        dynload::unload_librime();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::os::raw::c_int;

    #[cfg(not(all(feature = "dynload", target_os = "windows")))]
    fn round_up(x: usize, multiple: usize) -> usize {
        let remainder = x % multiple;
        match remainder {
            0 => x,
            _ => x + multiple - remainder,
        }
    }

    #[cfg(all(feature = "dynload", target_os = "windows"))]
    use super::dynload;

    fn load_and_unload<T: FnOnce()>(test: T) {
        #[cfg(all(feature = "dynload", target_os = "windows"))]
        static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        #[cfg(all(feature = "dynload", target_os = "windows"))]
        let _guard = TEST_LOCK.lock().unwrap();
        #[cfg(all(feature = "dynload", target_os = "windows"))]
        let file_name = rime_lib_name!();
        #[cfg(all(feature = "dynload", target_os = "windows"))]
        dynload::load_librime(file_name).expect("load_librime failed");

        test();

        // Note: We don't call unload_librime() here because:
        // 1. librime may have background threads or resources that are still in use
        // 2. The OS will automatically unload the library when the process exits
        // 3. Manually unloading can cause segfaults if any librime resources are still referenced
    }

    #[test]
    fn test_rime_api() {
        load_and_unload(|| {
            #[cfg(not(all(feature = "dynload", target_os = "windows")))] {
                unsafe {
                    #[cfg(all(feature = "dynload", target_os = "windows"))]
                    let rime_api = dynload::get_rime_api();
                    #[cfg(not(all(feature = "dynload", target_os = "windows")))]
                    let rime_api = rime_get_api();
                    assert_eq!(
                        std::mem::size_of::<RimeApi>(),
                        round_up(
                            (*rime_api).data_size as usize,
                            std::mem::align_of::<RimeApi>()
                        )
                    );
                }
            }
        });
    }

    #[test]
    fn test_find_module() {
        load_and_unload(|| {
            unsafe {
                #[cfg(all(feature = "dynload", target_os = "windows"))]
                let rime_api = dynload::get_rime_api();
                #[cfg(not(all(feature = "dynload", target_os = "windows")))]
                let rime_api = rime_get_api();

                assert!(!rime_api.is_null());
                let setup = (*rime_api).setup;
                assert!(setup.is_some());
                let mut test_traits: RimeTraits = std::mem::zeroed();
                test_traits.data_size = std::mem::size_of::<RimeTraits>() as c_int;
                test_traits.shared_data_dir = CStr::from_bytes_with_nul(b".\0").unwrap().as_ptr();
                test_traits.user_data_dir = CStr::from_bytes_with_nul(b".\0").unwrap().as_ptr();
                test_traits.distribution_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
                test_traits.distribution_code_name =
                    CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
                test_traits.distribution_version =
                    CStr::from_bytes_with_nul(b"0.1\0").unwrap().as_ptr();
                test_traits.app_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
                test_traits.modules = std::ptr::null_mut();
                let setup = setup.unwrap();
                setup(&mut test_traits);

                let find_module = (*rime_api).find_module;
                assert!(find_module.is_some());
                let find_module = find_module.unwrap();
                {
                    let core_module =
                        find_module(CStr::from_bytes_with_nul(b"core\0").unwrap().as_ptr());
                    assert!(!core_module.is_null());
                }
                {
                    let dict_module =
                        find_module(CStr::from_bytes_with_nul(b"dict\0").unwrap().as_ptr());
                    assert!(!dict_module.is_null());
                }
                {
                    let gears_module =
                        find_module(CStr::from_bytes_with_nul(b"gears\0").unwrap().as_ptr());
                    assert!(!gears_module.is_null());
                }
                {
                    let levers_module =
                        find_module(CStr::from_bytes_with_nul(b"levers\0").unwrap().as_ptr());
                    assert!(!levers_module.is_null());
                }
            }
        });
    }

    #[test]
    fn test_rime_api_call() {
        load_and_unload(|| {
            use std::os::raw::c_int;
            let mut test_traits: RimeTraits = rime_struct_new!();
            test_traits.data_size = std::mem::size_of::<RimeTraits>() as c_int;
            test_traits.shared_data_dir = CStr::from_bytes_with_nul(b".\0").unwrap().as_ptr();
            test_traits.user_data_dir = CStr::from_bytes_with_nul(b".\0").unwrap().as_ptr();
            test_traits.distribution_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
            test_traits.distribution_code_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
            test_traits.distribution_version = CStr::from_bytes_with_nul(b"0.1\0").unwrap().as_ptr();
            test_traits.app_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
            rime_api_call!(initialize, &mut test_traits);
            rime_api_call!(finalize);
        });
    }

    #[test]
    fn test_rime_module_call() {
        load_and_unload(|| {
            let levers_module = rime_api_call!(
                find_module,
                CStr::from_bytes_with_nul(b"levers\0").unwrap().as_ptr()
            );
            let _custom_settings = rime_module_call!(
                levers_module => RimeLeversApi,
                custom_settings_init,
                CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr(),
                CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr()
            );
        });
    }
}

