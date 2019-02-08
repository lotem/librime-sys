#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));



#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr};
    use std::os::raw::{c_int};

    fn round_up(x: usize, multiple: usize) -> usize {
        let remainder = x % multiple;
        match remainder {
            0 => x,
            _ => x + multiple - remainder,
        }
    }

    #[test]
    fn test_rime_api() {
        unsafe {
            let rime_api = rime_get_api();
            assert_eq!(
                std::mem::size_of::<RimeApi>(),
                round_up((*rime_api).data_size as usize,
                         std::mem::align_of::<RimeApi>()));
        }
    }

    #[test]
    fn test_find_module() {
        unsafe {
            let rime_api = rime_get_api();

            let setup = (*rime_api).setup;
            assert!(setup.is_some());
            let mut test_traits : RimeTraits = std::mem::uninitialized();
            test_traits.data_size = std::mem::size_of::<RimeTraits>() as c_int;
            test_traits.shared_data_dir = CStr::from_bytes_with_nul(b".\0").unwrap().as_ptr();
            test_traits.user_data_dir = CStr::from_bytes_with_nul(b".\0").unwrap().as_ptr();
            test_traits.distribution_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
            test_traits.distribution_code_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
            test_traits.distribution_version = CStr::from_bytes_with_nul(b"0.1\0").unwrap().as_ptr();
            test_traits.app_name = CStr::from_bytes_with_nul(b"test\0").unwrap().as_ptr();
            test_traits.modules = std::ptr::null_mut();
            let setup = setup.unwrap();
            setup(&mut test_traits);

            let find_module = (*rime_api).find_module;
            assert!(find_module.is_some());
            let find_module = find_module.unwrap();
            {
                let core_module = find_module(CStr::from_bytes_with_nul(b"core\0").unwrap().as_ptr());
                assert!(!core_module.is_null());
            }
            {
                let dict_module = find_module(CStr::from_bytes_with_nul(b"dict\0").unwrap().as_ptr());
                assert!(!dict_module.is_null());
            }
            {
                let gears_module = find_module(CStr::from_bytes_with_nul(b"gears\0").unwrap().as_ptr());
                assert!(!gears_module.is_null());
            }
            {
                let levers_module = find_module(CStr::from_bytes_with_nul(b"levers\0").unwrap().as_ptr());
                assert!(!levers_module.is_null());
            }
        }
    }
}
