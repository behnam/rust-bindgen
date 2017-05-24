/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: ::std::os::raw::c_longlong,
    pub __bindgen_padding_1: u64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(::std::mem::size_of::<max_align_t>() , 32usize , concat ! (
               "Size of: " , stringify ! ( max_align_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce1 as * const _ as usize } , 0usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce2 as * const _ as usize } , 16usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce2 ) ));
}
impl Clone for max_align_t {
    fn clone(&self) -> Self { *self }
}
