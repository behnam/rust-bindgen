/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

#![cfg(target_os="macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub trait Foo { }
impl Foo for id { }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct FooStruct {
    pub foo: *mut id,
}
#[test]
fn bindgen_test_layout_FooStruct() {
    assert_eq!(::std::mem::size_of::<FooStruct>() , 8usize , concat ! (
               "Size of: " , stringify ! ( FooStruct ) ));
    assert_eq! (::std::mem::align_of::<FooStruct>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( FooStruct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const FooStruct ) ) . foo as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( FooStruct ) , "::" ,
                stringify ! ( foo ) ));
}
impl Clone for FooStruct {
    fn clone(&self) -> Self { *self }
}
impl Default for FooStruct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn fooFunc(foo: id);
}
extern "C" {
    #[link_name = "kFoo"]
    pub static mut kFoo: *const id;
}
