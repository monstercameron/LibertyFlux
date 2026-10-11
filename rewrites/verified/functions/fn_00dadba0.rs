// original: 0x00DADBA0 CTaskComplexInvestigateDeadPed::vf1

/// Resolve the global task manager, then ask it to create this task's
/// follow-up object. A null manager returns zero without a second call.
/// The first helper receives the manager slot in ECX. The factory receives
/// the returned manager in ECX and task data from offset `+0x14`; the
/// investigation task also passes a signed-independent lifetime of 10000.
lf_checker_rt::export!(thiscall, rw_00DADBA0(this: u32) -> u32 {
    unsafe {
        const GLOBAL_MANAGER_VA: u32 = 0x0167e2a0;
        const SINGLETON: u32 = 1;
        const FACTORY: u32 = 2;

        const REQUEST_LIFETIME: u32 = 0x2710;
        const VALUE_FIELD: u32 = 0x14;
        let manager_slot = (lf_checker_rt::relocated(GLOBAL_MANAGER_VA) as *const u32).read();
        let manager = lf_checker_rt::callee_thiscall!(SINGLETON, u32, manager_slot);
        if manager == 0 { return 0; }
        let value = (this.wrapping_add(VALUE_FIELD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(FACTORY, u32, manager, value, REQUEST_LIFETIME)
    }
});
