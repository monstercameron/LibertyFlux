// original: 0x00B83E90 bitflag_via_lookup
/// Test a looked-up status word: bit 5 set and bit 12 clear.
///
/// Resolves the handle at `HDL` through callee 1, then returns 1 exactly
/// when the `READY` bit is set while the `BUSY` bit is clear.
///
/// Original: 0x00B83E90 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B83E90(this: u32) -> u32 {
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
        const HDL: u32 = 0x18;
        const STATUS: u32 = 0x20;
        let p = lf_checker_rt::callee_cdecl!(1, u32, rd32(this + HDL));
        let w = rd32(p + STATUS);
        if w & (1 << 5) != 0 && w & (1 << 12) == 0 { 1 } else { 0 }
    }
});
