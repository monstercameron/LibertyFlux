// original: 0x00DAE5E0 CTaskComplexAvoidPlayerTargetting::vf19

/// Request the avoidance task for a ped only after both stored references
/// are available; otherwise return zero without making a call.
///
/// The non-null tests read task fields at `+0x14` and `+0x18`. On the
/// active path, the task factory receives `this`, the incoming ped, and
/// request code `0x386`; the method returns the factory result and removes
/// its one incoming stack word.
lf_checker_rt::export!(thiscall, rw_00DAE5E0(this: u32, ped: u32) -> u32 {
    unsafe {
        const FIRST_REFERENCE: u32 = 0x14;
        const SECOND_REFERENCE: u32 = 0x18;
        const REQUEST_CODE: u32 = 0x386;
        const TASK_FACTORY: u32 = 1;
        let first = (this.wrapping_add(FIRST_REFERENCE) as *const u32).read_unaligned();
        let second = (this.wrapping_add(SECOND_REFERENCE) as *const u32).read_unaligned();
        if first == 0 || second == 0 { return 0; }
        lf_checker_rt::callee_thiscall!(TASK_FACTORY, u32, this, ped, REQUEST_CODE)
    }
});
