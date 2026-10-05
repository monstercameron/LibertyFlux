// original: 0x00c05830 stream_item_init_tag7c
/// Initialise a streaming request item (tag 0x7c) from two source objects.
///
/// `this` receives: tag byte 0x7c at +0, the shared streaming dword global at
/// +4, `arg0[0x64]` at +8 (or -1 when `arg0` is null), `arg1` at +0x18, the
/// low byte of `arg2` at +1 (with +2 cleared), `arg3[0xac]` at +0xc,
/// `arg3[0xb4][0x64]` at +0x10 (or -1 when `arg3[0xb4]` is null) and
/// `arg3[0xb0]` at +0x14. Returns `arg3[0xb0]`. Thiscall: `this` in ECX,
/// four stack words, callee cleans 0x10.
lf_checker_rt::export!(thiscall, rw_00c05830(this: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
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
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const TAG: u8 = 0x7c;
        const SHARED_DWORD: u32 = 0x011735A4;
        const SRC_SLOT: u32 = 0x64;
        const NONE: u32 = 0xFFFF_FFFF;
        wr8(this, TAG);
        wr32(this + 4, rd32(lf_checker_rt::relocated(SHARED_DWORD)));
        if arg0 != 0 {
            wr32(this + 8, rd32(arg0 + SRC_SLOT));
        } else {
            wr32(this + 8, NONE);
        }
        wr32(this + 0x18, arg1);
        wr8(this + 2, 0);
        wr8(this + 1, arg2 as u8);
        wr32(this + 0x0c, rd32(arg3 + 0xac));
        let inner = rd32(arg3 + 0xb4);
        if inner != 0 {
            wr32(this + 0x10, rd32(inner + SRC_SLOT));
        } else {
            wr32(this + 0x10, NONE);
        }
        let tail = rd32(arg3 + 0xb0);
        wr32(this + 0x14, tail);
        tail
    }
});
