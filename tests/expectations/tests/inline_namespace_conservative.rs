/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod bar {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub type Ty = ::std::os::raw::c_int;
        }
        pub type Ty = ::std::os::raw::c_longlong;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct Bar {
        pub baz: root::foo::bar::Ty,
    }
    #[test]
    fn bindgen_test_layout_Bar() {
        assert_eq!(::std::mem::size_of::<Bar>() , 4usize , concat ! (
                   "Size of: " , stringify ! ( Bar ) ));
        assert_eq! (::std::mem::align_of::<Bar>() , 4usize , concat ! (
                    "Alignment of " , stringify ! ( Bar ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const Bar ) ) . baz as * const _ as usize }
                    , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( Bar ) , "::" ,
                    stringify ! ( baz ) ));
    }
    impl Clone for Bar {
        fn clone(&self) -> Self { *self }
    }
}
