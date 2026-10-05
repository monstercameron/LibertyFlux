// original: 0x005dda30 task_vec_feed_chain

/// Feed a global vector through an indirect call, then chain two float calls.
///
/// Copies the global vector at `DFLT_VEC` to a local buffer and passes it
/// to the slot-`+0x9c` method of `b`. Then calls the first float callee
/// with (`g0`, `g1`, `[a+0x20]+0x30`, `+0x34`) and
/// the second with the first result; returns the final float's bits.
/// The callee answers are scripted; comparisons are bit-exact.
///
/// Original: 0x005dda30 (stdcall, two pointers).
lf_checker_rt::export!(stdcall, rw_005dda30(a: u32, b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const DFLT_VEC: u32 = 0x1b4b2a0;
        const FEED: u32 = 1;
        const FIRST: u32 = 2;
        const SECOND: u32 = 3;
        let g0 = rd32(lf_checker_rt::relocated(DFLT_VEC));
        let g1 = rd32(lf_checker_rt::relocated(DFLT_VEC + 4));
        let g2 = rd32(lf_checker_rt::relocated(DFLT_VEC + 8));
        let vt = rd32(b);
        let feed: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt + 0x9c) as usize) };
        let mut vec = [g0, g1, g2];
        feed(b, vec.as_ptr() as u32);
        let inner = rd32(a + 0x20);
        let w30 = rd32(inner + 0x30);
        let w34 = rd32(inner + 0x34);
        // The fourth word the original passes here is an uninitialized frame
        // slot; the contract fills uninitialized stack with zero.
        let r1: f32 = lf_checker_rt::callee_stdcall!(FIRST, f32, g0, g1, w30, w34);
        let r2: f32 = lf_checker_rt::callee_stdcall!(SECOND, f32, r1.to_bits());
        r2.to_bits()
    }
});
