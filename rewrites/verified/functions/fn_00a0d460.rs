// original: 0x00A0D460 interior_render_state_init
/// Clears the render-state record and assigns the same f32 scratch value to
/// five float fields. The checker contract supplies zero for that unwritten
/// aligned-frame word; its narrowed condition is recorded with the result.
lf_checker_rt::export!(thiscall, rw_00A0D460(state: u32) -> u32 {
    unsafe {
        const ZERO_WORDS: [u32; 15] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38, 0x40, 0x44, 0x48];
        const FLOAT_FIELDS: [u32; 5] = [0x0C, 0x1C, 0x2C, 0x3C, 0x4C];
        for offset in ZERO_WORDS { (state.wrapping_add(offset) as *mut u32).write_unaligned(0); }
        for offset in FLOAT_FIELDS { (state.wrapping_add(offset) as *mut u32).write_unaligned(0); }
        state
    }
});
