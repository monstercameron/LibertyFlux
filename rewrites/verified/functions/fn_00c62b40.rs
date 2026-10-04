// original: 0x00c62b40 anim_player_snap_rate
/// Rate snap: snaps the two rate fields to 1.0 or 0.0 by the rate's sign.
///
/// Marks the object active, then writes 1.0 into the rate fields at `+0x4C`
/// and `+0x50` when the rate at `+0x54` is negative (or NaN), else 0.0.
/// Refreshes the active flag from the tag word. Returns 0xFF01.
export!(thiscall, rw_00c62b40(this: u32) -> u32 {
    unsafe {
        let rate = f32::from_bits(*((this + 0x54) as *const u32));
        *((this + 4) as *mut u32) |= 0x10;
        if rate >= 0.0 {
            *((this + 0x50) as *mut u32) = 0;
            *((this + 0x4C) as *mut u32) = 0;
        } else {
            *((this + 0x50) as *mut u32) = 0x3F80_0000;
            *((this + 0x4C) as *mut u32) = 0x3F80_0000;
        }
        let w = (this + 0x46) as *mut u16;
        *w &= 0xFFFD;
        if *((this + 0x44) as *const u16) == 1 {
            *w |= 1;
        }
        0xFF01
    }
});
