// original: 0x00c06f60 stream_view_init_from_parts
/// Initialise a streaming view object from eight parts.
///
/// Stores `a0` at +4 and `a1` at +8 (both float bit patterns), copies up to
/// 0xff bytes from `a2` to `this`+0xc through the string helper (cdecl/3),
/// clears the flag byte at +0x10b, stores `a4` at +0x110, `a5` at +0x114,
/// `a6` at +0, `a7` at +0x118 and `a3` at +0x10c (float bits). Returns `a7`.
/// Thiscall: `this` in ECX, eight stack words, callee cleans 0x20. Only the
/// first 64 bytes of the helper's copy are observed (16-word out cap).
lf_checker_rt::export!(thiscall, rw_00c06f60(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
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

        const COPY: u32 = 1;
        const COPY_LEN: u32 = 0xFF;
        let _: u32 = lf_checker_rt::callee_cdecl!(COPY, u32, this + 0x0c, a2, COPY_LEN);
        wr32(this + 4, a0);
        wr32(this + 8, a1);
        wr8(this + 0x10b, 0);
        wr32(this + 0x110, a4);
        wr32(this + 0x114, a5);
        wr32(this, a6);
        wr32(this + 0x118, a7);
        wr32(this + 0x10c, a3);
        a7
    }
});
