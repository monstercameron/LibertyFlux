// original: 0x0092D010 select_probe_handler_tail (proposed)

/// Probe four handlers through the manager and tail-call the winner by priority.
///
/// Looks up the manager (callee 1, thiscall on `MANAGER`); a null manager
/// returns 0. Otherwise probes four handler kinds (callee 2, thiscall) with
/// `(manager, kind, 0)` for kinds `0x24`, `2`, `1`, `9` in that order, then
/// tail-calls the first non-null result in priority order `1`, `2`, `9`,
/// `0x24` through its vtable slot `+0x0c` (callee 3, thiscall,
/// intercepted through the planted slot), returning its result. All null
/// returns 0.
///
/// Original: 0x0092D010 (cdecl, no arguments). Six direct calls plus a
/// computed tail jump.
lf_checker_rt::export!(cdecl, rw_0092D010() -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0103_E498;
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        let mgr: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(MANAGER));
        if mgr == 0 {
            return 0;
        }
        let h24: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, 0x24u32, 0u32);
        let h2: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, 2u32, 0u32);
        let h1: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, 1u32, 0u32);
        let h9: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, 9u32, 0u32);
        for obj in [h1, h2, h9, h24] {
            if obj != 0 {
                let vt = rd(obj);
                let slot = rd(vt.wrapping_add(0x0c));
                let target: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                return target(obj);
            }
        }
        0
    }
});
