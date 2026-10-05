// original: 0x00B82760 thread_can_run_check
/// Decide whether a script thread may run this tick.
///
/// Fails when the liveness probe (callee 1) fails, when the mode word is
/// 2, or when the state word is `0x41F8`, `0x2000` or `0x4F10`. State
/// `0xFF0` passes outright; any other state passes when the override byte
/// `OV` is set. Returns 1 when the thread may run.
///
/// Original: 0x00B82760 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B82760(this: u32) -> u32 {
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
        const MODE: u32 = 0x0C;
        const STATE: u32 = 0x44;
        const OV: u32 = 0x96;
        if lf_checker_rt::callee_thiscall!(1, u32, this) & 0xFF == 0 {
            return 0;
        }
        if rd32(this + MODE) == 2 {
            return 0;
        }
        let st = rd32(this + STATE);
        if st > 0x41F8 {
            if st == 0x4F10 {
                return 0;
            }
            return (rd8(this + OV) != 0) as u32;
        }
        if st == 0x41F8 {
            return 0;
        }
        if st == 0xFF0 {
            return 1;
        }
        if st == 0x2000 {
            return 0;
        }
        (rd8(this + OV) != 0) as u32
    }
});
