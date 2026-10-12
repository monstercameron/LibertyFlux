// original: 0x00DF2CF0 UIMontageText::cycle_palette_forward

/// Advance the unsigned palette index stored at `this + 0x218`, wrapping
/// values at or above sixteen to zero. The updated view is then forwarded to
/// the montage text refresh routine by a tail call. The stored index uses the
/// view's fixed 32-bit object layout; wrapping is controlled by the original
/// unsigned comparison, including values with the sign bit set.
lf_checker_rt::export!(thiscall, rw_00df2cf0(this: u32) -> u32 {
    unsafe {
        const PALETTE_INDEX: u32 = 0x218;
        const PALETTE_COUNT: u32 = 16;
        let address = this.wrapping_add(PALETTE_INDEX) as *mut u32;
        let palette = address.read_unaligned();
        let next_palette = if palette >= PALETTE_COUNT {
            0
        } else {
            palette.wrapping_add(1)
        };
        address.write_unaligned(next_palette);
        lf_checker_rt::callee_thiscall!(1, u32, this)
    }
});
