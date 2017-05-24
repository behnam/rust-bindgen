/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct C {
    pub _bitfield_1: u8,
    pub __bindgen_align: [u8; 0usize],
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 1usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
impl C {
    #[inline]
    pub fn a(&self) -> bool {
        let mask = 1usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_a(&mut self, val: bool) {
        let mask = 1usize as u8;
        let val = val as u8 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b(&self) -> bool {
        let mask = 254usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 1usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_b(&mut self, val: bool) {
        let mask = 254usize as u8;
        let val = val as u8 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 1usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(a: bool, b: bool) -> u8 {
        ({ ({ 0 } | ((a as u8 as u8) << 0usize) & (1usize as u8)) } |
             ((b as u8 as u8) << 1usize) & (254usize as u8))
    }
}
