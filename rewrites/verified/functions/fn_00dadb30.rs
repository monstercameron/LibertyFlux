// original: 0x00DADB30 CTaskComplexAvoidPlayerTargetting::vf1

/// Clone the avoidance task through the global task manager. The selector
/// byte at `+0x1C` chooses which manager-backed helper receives the values
/// at `+0x14` and `+0x18`; the set route also passes a pointer to `+0x20`.
/// A null manager returns zero. Both helpers return this method's result.
lf_checker_rt::export!(thiscall, rw_00DADB30(this: u32) -> u32 {
    unsafe {
        const MANAGER_SLOT_VA: u32 = 0x0167E2A0;
        const MODE: u32 = 0x1C;
        const FIRST_VALUE: u32 = 0x14;
        const SECOND_VALUE: u32 = 0x18;
        const EMBEDDED_MEMBER: u32 = 0x20;
        const LOOKUP: u32 = 1;
        const SET_HELPER: u32 = 2;
        const CLEAR_HELPER: u32 = 3;

        let mode = (this.wrapping_add(MODE) as *const u8).read();
        let manager_slot = (lf_checker_rt::relocated(MANAGER_SLOT_VA) as *const u32).read();
        let manager = lf_checker_rt::callee_thiscall!(LOOKUP, u32, manager_slot);
        if manager == 0 { return 0; }
        let first = (this.wrapping_add(FIRST_VALUE) as *const u32).read_unaligned();
        let second = (this.wrapping_add(SECOND_VALUE) as *const u32).read_unaligned();
        if mode != 0 {
            lf_checker_rt::callee_thiscall!(SET_HELPER, u32, manager, first, second, this.wrapping_add(EMBEDDED_MEMBER))
        } else {
            lf_checker_rt::callee_thiscall!(CLEAR_HELPER, u32, manager, first, second)
        }
    }
});
