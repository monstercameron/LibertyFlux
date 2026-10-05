// original: 0x00AC6440 shader_fx_set_color4 (proposed)

/// Copy four words from `src` to the colour slot at `this + 0x350`.
///
/// The original is a thiscall taking one stack pointer; it copies the words
/// in ascending order. No value is returned.
lf_checker_rt::export!(thiscall, rw_00AC6440(this: u32, src: u32) -> u32 {
    unsafe {
        const COLOR: u32 = 0x350;
        for i in 0..4u32 {
            let v = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
            (this.wrapping_add(COLOR + i * 4) as *mut u32).write_unaligned(v);
        }
        0
    }
});
