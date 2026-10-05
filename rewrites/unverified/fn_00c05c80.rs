// original: 0x00c05c80 stream_item_init_tag8d
/// Initialise a streaming request item (tag 0x8d) from one source object.
///
/// `this` receives: tag byte 0x8d at +0 and the shared streaming dword global
/// at +4. A null `arg0` only sets the validity flag at +1 to 1 and returns
/// the shared dword. Otherwise +8 takes `arg0[4][0x64]` (-1 when `arg0[4]`
/// is null), +0xc takes `arg0[8]`, +1 is cleared, and `arg0[8]` is returned.
/// Thiscall: `this` in ECX, one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c05c80(this: u32, arg0: u32) -> u32 {
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

        const TAG: u8 = 0x8d;
        const SHARED_DWORD: u32 = 0x011735A4;
        const SRC_SLOT: u32 = 0x64;
        const NONE: u32 = 0xFFFF_FFFF;
        wr8(this, TAG);
        let shared = rd32(lf_checker_rt::relocated(SHARED_DWORD));
        wr32(this + 4, shared);
        if arg0 == 0 {
            wr8(this + 1, 1);
            return shared;
        }
        let inner = rd32(arg0 + 4);
        if inner == 0 {
            wr32(this + 8, NONE);
        } else {
            wr32(this + 8, rd32(inner + SRC_SLOT));
        }
        let tail = rd32(arg0 + 8);
        wr32(this + 0x0c, tail);
        wr8(this + 1, 0);
        tail
    }
});
