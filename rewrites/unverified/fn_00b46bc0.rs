// original: 0x00B46BC0 CWeightedTaskSet::~CWeightedTaskSet__deleting

/// Installs the task-set type table, optionally forwards the object for deletion when the low flag bit is set, and returns the object address.
lf_checker_rt::export!(thiscall, rw_00b46bc0(this_ptr: u32, deleting_flags: u32) -> u32 {
    const DELETE_FLAG: u32 = 1;
    const TASKSET_VTABLE_VA: u32 = 0x00eae7d4;
    const ALLOCATOR_CONTEXT_VA: u32 = 0x016669c8;
    unsafe {
        core::ptr::write_unaligned(this_ptr as *mut u32, lf_checker_rt::relocated(TASKSET_VTABLE_VA));
        if (deleting_flags & DELETE_FLAG) != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this_ptr);
        }
    }
    this_ptr

});
