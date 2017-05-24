/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct foo__bindgen_ty_1 {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 8usize , concat !
               ( "Size of: " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<foo__bindgen_ty_1>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( foo__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . x as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo__bindgen_ty_1 ) ) . y as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( foo__bindgen_ty_1 ) ,
                "::" , stringify ! ( y ) ));
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 8usize , concat ! (
               "Size of: " , stringify ! ( foo ) ));
    assert_eq! (::std::mem::align_of::<foo>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( foo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . bar as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( foo ) , "::" ,
                stringify ! ( bar ) ));
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
