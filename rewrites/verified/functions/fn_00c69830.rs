// original: 0x00c69830 stream_probe_handles (proposed)

/// Probe the streaming handles: fast path by state, else by liveness.
///
/// If the primary probe finds a handle, report whether its state word
/// is 2. Otherwise both fallback probes must find a live handle; the
/// liveness check runs on the member at offset 0x224 and its low byte
/// decides. Only the low byte of the answer is defined; the upper
/// bytes keep whatever the last callee left.
///
/// Original: cdecl with no arguments, four call sites, al-only return.
lf_checker_rt::export!(cdecl, rw_00c69830() -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const FALLBACK: u32 = 2;
        const CHECK: u32 = 3;
        const STATE_OFF: u32 = 0x1300;
        const MEMBER_OFF: u32 = 0x224;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let h: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, 0);
        if h != 0 {
            return (rd32(h.wrapping_add(STATE_OFF)) == 2) as u32;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(FALLBACK, u32,);
        if p == 0 {
            return 0;
        }
        let q: u32 = lf_checker_rt::callee_cdecl!(FALLBACK, u32,);
        let member = rd32(q.wrapping_add(MEMBER_OFF));
        let r: u32 = lf_checker_rt::callee_thiscall!(CHECK, u32, member);
        ((r & 0xFF) != 0) as u32
    }
});
