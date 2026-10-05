// original: 0x00c06200 stream_cached_copy
/// Copy through the reader into a cached buffer, allocating on demand.
///
/// Prepares through the prep helper (thiscall/0), reads 4 bytes through the
/// reader (cdecl/3: `arg`, `this`+0x4c, 4) and returns 0 when the count is
/// zero. Otherwise allocates that many bytes (cdecl/1) into `this`+0x48; on
/// success reads the span through the reader again (cdecl/3: `arg`,
/// `this`+0x48, count) and returns 1, on failure queries the size helper
/// (thiscall/0), reports through the report helper (thiscall/1), clears
/// `this`+0x4c and returns 0. Thiscall: one stack word, callee cleans 4.
/// The two reader sites use separate callee ids so the first call's fill is
/// observed; the second call's copy-out is not observed.
lf_checker_rt::export!(thiscall, rw_00c06200(this: u32, arg: u32) -> u32 {
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
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        const PREP: u32 = 1;
        const READ1: u32 = 2;
        const READ2: u32 = 3;
        const ALLOC: u32 = 4;
        const SIZEQ: u32 = 5;
        const REPORT: u32 = 6;
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, this);
        let _: u32 = lf_checker_rt::callee_cdecl!(READ1, u32, arg, this + 0x4c, 4);
        let n = rd32(this + 0x4c);
        if n == 0 {
            return 0;
        }
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, n);
        wr32(this + 0x48, buf);
        if buf != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(READ2, u32, arg, rd32(this + 0x48), n);
            return 1;
        }
        let q: u32 = lf_checker_rt::callee_thiscall!(SIZEQ, u32, arg);
        let _: u32 = lf_checker_rt::callee_thiscall!(REPORT, u32, arg, q.wrapping_add(n));
        wr32(this + 0x4c, 0);
        0
    }
});
