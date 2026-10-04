// original: 0x00e67120 fanout_two_floats (proposed)
/// Copy two global floats each to six destination globals.
///
/// Loads `a` and `b` once, then stores `a` to six addresses and `b` to the
/// six words right after them (pairs at six sites). Bit-exact copy through
/// integer moves. No arguments (cdecl/0), no calls.
/// Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e67120() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        const SRC_A: u32 = 0x01050B4C;
        const SRC_B: u32 = 0x01050B50;
        const DSTS: [u32; 6] = [
            0x0103C134, 0x0103C15C, 0x0103C184, 0x0103C314, 0x0103C33C, 0x0103C364,
        ];
        let a = rd32(SRC_A);
        let b = rd32(SRC_B);
        let mut i = 0usize;
        while i < DSTS.len() {
            wr32(DSTS[i], a);
            wr32(DSTS[i].wrapping_add(4), b);
            i += 1;
        }
        0
    }
});
