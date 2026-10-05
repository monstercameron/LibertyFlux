// original: 0x00B82830 GtaThread::vf3
/// Run one slice of a script thread.
///
/// Prepares the scratch area (callees 1-2) and clears the hold byte. An
/// already-signalled thread, or one the gate (callee 3) releases,
/// returns its status word. Otherwise the wake is committed (callee 4),
/// the runner globals are refreshed from the config table and control
/// passes to the stepper (callee 5), whose answer is returned.
///
/// Original: 0x00B82830 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00B82830(this: u32, arg: u32) -> u32 {
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
        const HOLD: u32 = 0x87;
        const SIG: u32 = 0x9C;
        const ST: u32 = 0x0C;
        const CFG: u32 = 0x0118D804;
        const R0: u32 = 0x011DB234;
        const R1: u32 = 0x011DB278;
        const R2: u32 = 0x011E66C8;
        let seed = lf_checker_rt::callee_cdecl!(1, u32, 0x18);
        lf_checker_rt::callee_cdecl!(2, u32, this.wrapping_add(0x70), seed);
        wr8(this + HOLD, 0);
        if rd8(this + SIG) != 0 {
            return rd32(this + ST);
        }
        if lf_checker_rt::callee_cdecl!(3, u32,) & 0xFF != 0 {
            return rd32(this + ST);
        }
        lf_checker_rt::callee_thiscall!(4, u32, this);
        let cfg = (lf_checker_rt::global::<u32>(CFG)).read_unaligned();
        (lf_checker_rt::global::<u32>(R0)).write_unaligned(rd32(cfg + 0x540));
        (lf_checker_rt::global::<u32>(R1)).write_unaligned(0);
        (lf_checker_rt::global::<u8>(R2)).write(0);
        lf_checker_rt::callee_thiscall!(5, u32, this, arg)
    }
});
