// original: 0x00A4B550 vehicle_decay_u16_f48 (proposed)

/// Decays the u16 at `this + LEVEL` by the truncated global rate, zeroing a
/// companion counter when it bottoms out.
///
/// `w = u16[this + LEVEL]` (0x0F48). When `w` is 0 or `FULL` (0xFFFF) nothing
/// happens. Otherwise `step = trunc(global_float * 1000.0)` as a 64-bit
/// x87-style truncation (out-of-range and NaN yield the indefinite
/// 0x8000000000000000, whose low word is exact here), taken as an unsigned
/// 32-bit value `s`: when `w >= s` the level drops to `w - s`, else the level
/// and the dword at `this + COUNT` (0x12EC) are both zeroed. The float-to-int
/// edge emulation matches `fistp` bit for bit; the single multiply needs no
/// order pinning.
///
/// Original: 0x00A4B550 (thiscall, no stack words), leaf, x87 truncation.
lf_checker_rt::export!(thiscall, rw_00A4B550(this: u32) -> u32 {
    unsafe {
        const LEVEL: u32 = 0x0F48;
        const COUNT: u32 = 0x12EC;
        const RATE: u32 = 0x011735BC;
        const FULL: u32 = 0xFFFF;
        const SCALE: f32 = f32::from_bits(0x447A_0000); // 1000.0
        const TWO63: f32 = f32::from_bits(0x5F00_0000); // 2^63
        let g = f32::from_bits(
            lf_checker_rt::global::<u32>(RATE).read_unaligned(),
        );
        let x = g * SCALE;
        // Truncate exactly like fistp-to-qword under a truncating control
        // word: the indefinite value when the truncation would not fit.
        let t: i64 = if x.is_nan() || x >= TWO63 || x < -TWO63 {
            i64::MIN
        } else {
            x as i64
        };
        let s = t as u32;
        let w = ((this + LEVEL) as *const u16).read_unaligned() as u32;
        if w == 0 || w == FULL {
            return 0;
        }
        if w >= s {
            ((this + LEVEL) as *mut u16).write_unaligned((w - s) as u16);
        } else {
            ((this + LEVEL) as *mut u16).write_unaligned(0);
            ((this + COUNT) as *mut u32).write_unaligned(0);
        }
        0
    }
});
