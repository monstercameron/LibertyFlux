// original: 0x00DE1AB0 scrolling_menu_value_is_positive

/// Read the signed 32-bit value at `+0x208` and return one only when it is strictly greater than zero. Zero and negative values return zero.
lf_checker_rt::export!(thiscall, rw_00de1ab0(this_ptr: u32) -> u32 {
    const VALUE_OFFSET: u32 = 0x208;
    let value = unsafe { ((this_ptr.wrapping_add(VALUE_OFFSET)) as *const i32).read_unaligned() };
    u32::from(value > 0)
});
