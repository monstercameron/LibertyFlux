// original: 0x00B827B0 thread_flag_clear
/// Clear a thread's done flags, releasing two global counts.
///
/// When the outer flag `F0` is set, the inner flag `F1` (if set) releases
/// one unit of `G1` when positive and is cleared, then `G2` is released
/// the same way and `F0` is cleared. Counts are signed: zero or negative
/// is left alone.
///
/// Original: 0x00B827B0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B827B0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const F0: u32 = 0x99;
        const F1: u32 = 0x9A;
        const G1: u32 = 0x011E624C;
        const G2: u32 = 0x011E6248;
        if rd8(this + F0) != 0 {
            if rd8(this + F1) != 0 {
                let g = (lf_checker_rt::global::<u32>(G1)).read_unaligned() as i32;
                if g > 0 {
                    (lf_checker_rt::global::<u32>(G1)).write_unaligned((g - 1) as u32);
                }
                wr8(this + F1, 0);
            }
            let g = (lf_checker_rt::global::<u32>(G2)).read_unaligned() as i32;
            if g > 0 {
                (lf_checker_rt::global::<u32>(G2)).write_unaligned((g - 1) as u32);
            }
            wr8(this + F0, 0);
        }
        0
    }
});
