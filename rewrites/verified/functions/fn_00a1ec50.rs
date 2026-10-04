// original: 0x00a1ec50 cam_bound_track_min (proposed)

/// Tracks the lower of a stored bound and two candidates.
///
/// `this` points to a record with a bound float at `+BOUND_OFF` and a
/// state word at `+STATE_OFF`. When the bound is strictly above the first
/// argument `a0`, the state word is cleared and the bound becomes `a0`.
/// Otherwise, when the negation of the second argument (bitwise xor with
/// `0x80000000`, preserving NaN payloads) is strictly above the bound,
/// the state word is cleared and the bound becomes that negation.
/// Otherwise nothing changes. Returns nothing.
///
/// Original: 0x00a1ec50 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a1ec50(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const BOUND_OFF: u32 = 0x304;
        const STATE_OFF: u32 = 0x310;
        const SIGN_BIT: u32 = 0x8000_0000;
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        let cur = f32::from_bits(rd32(this + BOUND_OFF));
        if cur > f32::from_bits(a0) {
            ((this + STATE_OFF) as *mut u32).write_unaligned(0);
            ((this + BOUND_OFF) as *mut u32).write_unaligned(a0);
            return 0;
        }
        let neg = f32::from_bits(a1 ^ SIGN_BIT);
        if neg > cur {
            ((this + STATE_OFF) as *mut u32).write_unaligned(0);
            ((this + BOUND_OFF) as *mut u32).write_unaligned(neg.to_bits());
        }
        0
    }
});
