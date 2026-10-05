// original: 0x00B83EC0 bitflag_direct
/// Test the status word: bit 5 set and bit 12 clear.
///
/// Reads the word at `STATUS` past `this` and returns 1 exactly when the
/// `READY` bit is set while the `BUSY` bit is clear.
///
/// Original: 0x00B83EC0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B83EC0(this: u32) -> u32 {
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
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const STATUS: u32 = 0x20;
        const READY: u32 = 1 << 5;
        const BUSY: u32 = 1 << 12;
        let w = rd32(this + STATUS);
        if w & READY != 0 && w & BUSY == 0 { 1 } else { 0 }
    }
});
