// original: 0x00DF3010 UIMontageText::decrease_palette_index

/// Decrease the unsigned palette index at `this + 0x218`, wrapping zero to
/// sixteen. Every nonzero value is reduced by one. The selected palette is
/// written back before the view is sent to the montage text refresh routine
/// through a tail call. The unsigned zero test preserves behavior for every
/// 32-bit input, including values whose top bit is set.
lf_checker_rt::export!(thiscall, rw_00df3010(this: u32) -> u32 {
    unsafe {
        const PALETTE_INDEX: u32 = 0x218;
        const PALETTE_COUNT: u32 = 16;
        let address = this.wrapping_add(PALETTE_INDEX) as *mut u32;
        let palette = address.read_unaligned();
        let previous_palette = if palette == 0 {
            PALETTE_COUNT
        } else {
            palette.wrapping_sub(1)
        };
        address.write_unaligned(previous_palette);
        lf_checker_rt::callee_thiscall!(1, u32, this)
    }
});
