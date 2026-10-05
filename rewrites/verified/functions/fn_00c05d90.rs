// original: 0x00c05d90 stream_maybe_release_slot
/// Refresh a streaming slot and release its buffer when one is attached.
///
/// Calls the slot-refresh helper (thiscall/0) on `this`, then, only when the
/// 16-bit word at `this`+0x2e is non-zero, passes `this`+0x28 to the release
/// helper (cdecl/1). Returns nothing. Thiscall with no stack arguments; both
/// callees are intercepted and stubbed by the checker.
lf_checker_rt::export!(thiscall, rw_00c05d90(this: u32) -> u32 {
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

        const REFRESH: u32 = 1;
        const RELEASE: u32 = 2;
        const COUNT_OFF: u32 = 0x2e;
        const BUF_OFF: u32 = 0x28;
        let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, this);
        if rd16(this + COUNT_OFF) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, rd32(this + BUF_OFF));
        }
        0
    }
});
