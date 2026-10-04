// original: 0x00e67060 global_ratio_store_5 (proposed)
/// Divide two global floats and store the quotient in a third global.
///
/// Same shape as 0x00e66eb0 at different addresses. No arguments (cdecl/0),
/// no calls. Zero divisor yields inf/NaN, never a fault.
/// Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e67060() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        const SRC_A: u32 = 0x0103B918;
        const SRC_B: u32 = 0x0103B91C;
        const DST: u32 = 0x012BD1D4;
        wrf(DST, fdiv(rdf(SRC_A), rdf(SRC_B)));
        0
    }
});
