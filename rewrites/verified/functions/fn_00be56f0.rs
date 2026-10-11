// original: 0x00BE56F0 task_type_from_manager

/// Query the task manager for type 0xdf. Return the task's type field when the
/// lookup succeeds, or the all-bits-set sentinel when it returns null. The
/// manager method is intercepted by the contract and receives one selector.
lf_checker_rt::export!(cdecl, rw_00be56f0(context: u32) -> u32 {
    unsafe {
        const MANAGER_SLOT: u32 = 0x224;
        const TASK_COLLECTION: u32 = 0x44;
        const TYPE_FIELD: u32 = 0x3c;
        const TASK_TYPE: u32 = 0xdf;
        let manager = ((context + MANAGER_SLOT) as *const u32).read_unaligned();
        let task = lf_checker_rt::callee_thiscall!(1, u32, manager + TASK_COLLECTION, TASK_TYPE);
        if task == 0 {
            u32::MAX
        } else {
            ((task + TYPE_FIELD) as *const u32).read_unaligned()
        }
    }
});
