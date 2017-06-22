/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Template<T> {
    pub member: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for Template<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ContainsInstantiation {
    pub not_opaque: Template<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_ContainsInstantiation() {
    assert_eq!(::std::mem::size_of::<ContainsInstantiation>() , 1usize ,
               concat ! ( "Size of: " , stringify ! ( ContainsInstantiation )
               ));
    assert_eq! (::std::mem::align_of::<ContainsInstantiation>() , 1usize ,
                concat ! (
                "Alignment of " , stringify ! ( ContainsInstantiation ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ContainsInstantiation ) ) . not_opaque as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ContainsInstantiation )
                , "::" , stringify ! ( not_opaque ) ));
}
impl Clone for ContainsInstantiation {
    fn clone(&self) -> Self { *self }
}
impl Default for ContainsInstantiation {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct ContainsOpaqueInstantiation {
    pub opaque: u32,
}
#[test]
fn bindgen_test_layout_ContainsOpaqueInstantiation() {
    assert_eq!(::std::mem::size_of::<ContainsOpaqueInstantiation>() , 4usize ,
               concat ! (
               "Size of: " , stringify ! ( ContainsOpaqueInstantiation ) ));
    assert_eq! (::std::mem::align_of::<ContainsOpaqueInstantiation>() , 4usize
                , concat ! (
                "Alignment of " , stringify ! ( ContainsOpaqueInstantiation )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ContainsOpaqueInstantiation ) ) . opaque
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                ContainsOpaqueInstantiation ) , "::" , stringify ! ( opaque )
                ));
}
impl Clone for ContainsOpaqueInstantiation {
    fn clone(&self) -> Self { *self }
}
#[test]
fn __bindgen_test_layout_Template_instantiation() {
    assert_eq!(::std::mem::size_of::<Template<::std::os::raw::c_char>>() ,
               1usize , concat ! (
               "Size of template specialization: " , stringify ! (
               Template<::std::os::raw::c_char> ) ));
    assert_eq!(::std::mem::align_of::<Template<::std::os::raw::c_char>>() ,
               1usize , concat ! (
               "Alignment of template specialization: " , stringify ! (
               Template<::std::os::raw::c_char> ) ));
}
