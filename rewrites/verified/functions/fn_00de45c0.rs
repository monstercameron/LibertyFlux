// original: 0x00DE45C0 UILayoutManager::vf124

/// Store the low byte of the one stack word in `field_1E0` at `+0x1E0`.
///
/// `thiscall`: ECX is the object; the method changes no other memory and has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00de45c0(this_ptr: u32, value: u32) -> u32 {
    const FLAG_OFFSET: u32 = 0x1E0;
    unsafe { ((this_ptr.wrapping_add(FLAG_OFFSET)) as *mut u8)
        .write(value as u8) };
    0
});
