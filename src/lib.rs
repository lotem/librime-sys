#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! rime_struct_new {
    () => {{
        #[allow(unused_unsafe)]
        let mut s = unsafe { ::std::mem::zeroed() };
        if false {
            s
        } else {
            s.data_size = (::std::mem::size_of_val(&s) - ::std::mem::size_of_val(&s.data_size))
                as ::std::ffi::c_int;
            s
        }
    }};
    ($t:ty) => {{
        let s: $t = $crate::rime_struct_new!();
        s
    }};
}

#[macro_export]
macro_rules! rime_struct {
    ($var:ident : $t:ty) => {
        #[allow(unused_mut)]
        let mut $var: $t = $crate::rime_struct_new!();
    };
}

#[macro_export]
macro_rules! rime_call {
    ( $api_struct:expr, $api_fn:ident $(, $arg:expr)* ) => {{
        let api_fn = $api_struct.$api_fn.expect(concat!(
            "missing api function: ",
            stringify!($api_struct),
            ".",
            stringify!($api_fn)
        ));
        #[allow(unused_unsafe, clippy::macro_metavars_in_unsafe)]
        unsafe { api_fn($($arg),*) }
    }};
}

#[macro_export]
macro_rules! rime_api_call {
    ( $api_fn:ident $(, $arg:expr)* ) => {{
        #[allow(unused_unsafe)]
        let rime_api = unsafe { $crate::rime_get_api() };
        $crate::rime_call!(unsafe { *rime_api }, $api_fn $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! rime_module_call {
    ( $module:expr => $api_type:ty, $api_fn:ident $(, $arg:expr)* ) => {{
        let module = $module;
        let module_api = $crate::rime_call!(unsafe { *module }, get_api) as *const $api_type;
        $crate::rime_call!(unsafe { *module_api }, $api_fn $(, $arg)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    fn round_up(x: usize, multiple: usize) -> usize {
        let remainder = x % multiple;
        match remainder {
            0 => x,
            _ => x + multiple - remainder,
        }
    }

    #[test]
    fn test_rime_struct_macros() {
        let traits_a: RimeTraits = rime_struct_new!();
        let traits_b = rime_struct_new!(RimeTraits);
        rime_struct!(traits_c: RimeTraits);

        let expected = (mem::size_of::<RimeTraits>() - mem::size_of_val(&traits_a.data_size))
            as std::ffi::c_int;
        assert_eq!(traits_a.data_size, expected);
        assert_eq!(traits_b.data_size, expected);
        assert_eq!(traits_c.data_size, expected);
    }

    #[test]
    fn test_rime_api() {
        unsafe {
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

    #[test]
    fn test_find_module() {
        unsafe {
            let rime_api = rime_get_api();

            let setup = (*rime_api).setup;
            assert!(setup.is_some());
            rime_struct!(test_traits: RimeTraits);
            test_traits.shared_data_dir = c".".as_ptr();
            test_traits.user_data_dir = c".".as_ptr();
            test_traits.distribution_name = c"test".as_ptr();
            test_traits.distribution_code_name = c"test".as_ptr();
            test_traits.distribution_version = c"0.1".as_ptr();
            test_traits.app_name = c"test".as_ptr();
            test_traits.modules = std::ptr::null_mut();
            let setup = setup.unwrap();
            setup(&mut test_traits);

            let find_module = (*rime_api).find_module;
            assert!(find_module.is_some());
            let find_module = find_module.unwrap();
            {
                let core_module = find_module(c"core".as_ptr());
                assert!(!core_module.is_null());
            }
            {
                let dict_module = find_module(c"dict".as_ptr());
                assert!(!dict_module.is_null());
            }
            {
                let gears_module = find_module(c"gears".as_ptr());
                assert!(!gears_module.is_null());
            }
            {
                let levers_module = find_module(c"levers".as_ptr());
                assert!(!levers_module.is_null());
            }
        }
    }

    #[test]
    fn test_rime_api_call() {
        let mut test_traits: RimeTraits = rime_struct_new!();
        rime_api_call!(initialize, &mut test_traits);
        rime_api_call!(finalize);
    }

    #[test]
    fn test_rime_module_call() {
        let levers_module = rime_api_call!(find_module, c"levers".as_ptr());
        let _custom_settings = rime_module_call!(
            levers_module => RimeLeversApi,
            custom_settings_init,
            c"test".as_ptr(),
            c"test".as_ptr()
        );
    }
}
