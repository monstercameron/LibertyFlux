// original: 0x00DE45B0 UIRowLayout::vf128

/// Store the one stack word in `field_1E8` at `+0x1E8`.
///
/// `thiscall`: ECX is the object; this is a leaf setter with no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00de45b0(this_ptr: u32, value: u32) -> u32 {
    const FIELD_OFFSET: u32 = 0x1E8;
    unsafe { ((this_ptr.wrapping_add(FIELD_OFFSET)) as *mut u32)
        .write_unaligned(value) };
    0
});
