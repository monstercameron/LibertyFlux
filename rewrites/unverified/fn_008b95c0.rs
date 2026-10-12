// original: 0x008B95C0 NativeImpl_IS_FRONTEND_FADING

/// Decide whether the frontend counts as fading for this menu object.
///
/// Returns 1 only when the flag byte at `+0x00` is clear, the level byte at
/// `LEVEL` is strictly below the target byte at `TARGET` (both unsigned),
/// and the mode word at `MODE` equals 1; otherwise 0. Only the low byte of
/// the return is set; the upper 24 bits keep whatever the caller left in
/// EAX. Thiscall, object in ECX.
lf_checker_rt::export!(thiscall, rw_008B95C0(this: u32) -> u32 {
    unsafe {
        const LEVEL: u32 = 0x2B;
        const TARGET: u32 = 0x2F;
        const MODE: u32 = 0x14;
        const FADE_MODE: u32 = 1;
        if (this as *const u8).read_unaligned() != 0 {
            return 0;
        }
        let level = (this.wrapping_add(LEVEL) as *const u8).read_unaligned();
        let target = (this.wrapping_add(TARGET) as *const u8).read_unaligned();
        if level >= target {
            return 0;
        }
        let mode = (this.wrapping_add(MODE) as *const u32).read_unaligned();
        u32::from(mode == FADE_MODE)
    }
});
