// original: 0x00c06090 stream_buffer_attach
/// Attach a grown buffer to `this` from source `src`.
///
/// Does nothing when `src` is null (returning 0) or when `src`+0x48 is null
/// (returning `src`). Otherwise prepares through the prep helper (thiscall/0),
/// allocates through the allocator (cdecl/1 with `src`+0x4c) into `this`+0x48,
/// and, unless that failed, copies through the copy helper (cdecl/3 with
/// the fresh buffer, `src`+0x48 and `src`+0x4c) and records `src`+0x4c at
/// `this`+0x4c. Returns the fresh buffer (or the early value). Thiscall: one
/// stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c06090(this: u32, src: u32) -> u32 {
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
        const ALLOC: u32 = 2;
        const COPY: u32 = 3;
        const SIZE_OFF: u32 = 0x48;
        const AUX_OFF: u32 = 0x4c;
        if src == 0 {
            return 0;
        }
        let size = rd32(src + SIZE_OFF);
        let aux = rd32(src + AUX_OFF);
        if size == 0 {
            return src;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, this);
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, aux);
        wr32(this + SIZE_OFF, buf);
        if buf == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(COPY, u32, buf, size, aux);
        wr32(this + AUX_OFF, aux);
        buf
    }
});
