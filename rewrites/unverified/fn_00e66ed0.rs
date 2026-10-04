// original: 0x00e66ed0 zero_strided_globals (proposed)
/// Zero 17 global words on a 0x2c stride plus one trailing global word.
///
/// Writes 0 to `[0x012BCD40 + i*0x2C]` for `i` in 0..17 (the original counts
/// ecx down from 16 and keeps looping while the decrement is non-negative,
/// which runs 17 times) and then to `0x012BD004`. No arguments (cdecl/0),
/// no calls. Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e66ed0() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        const BASE: u32 = 0x012BCD40;
        const STRIDE: u32 = 0x2C;
        const COUNT: u32 = 17;
        const TRAIL: u32 = 0x012BD004;
        let mut i = 0u32;
        while i < COUNT {
            wr32(BASE.wrapping_add(i.wrapping_mul(STRIDE)), 0);
            i += 1;
        }
        wr32(TRAIL, 0);
        0
    }
});
