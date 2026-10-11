// original: 0x00DA6CC0 CTaskComplexFleeShooting::~CTaskComplexFleeShooting__deleting

/// Run this task's destructor and optionally release the task allocation.
///
/// `this` is the task object. `delete_flag` is a caller flag whose low bit
/// requests deallocation. The destructor helper always receives `this`;
/// when the bit is set, the game memory-manager pointer is read from its
/// global slot and the delete helper receives that manager and `this`.
/// The function returns `this` and uses the thiscall convention with one
/// stack argument, which the callee removes.
///
/// Both helpers are scripted by the checker contract. This rewrite covers
/// the flag test, the global load, call order and arguments, and return value.
lf_checker_rt::export!(thiscall, rw_00DA6CC0(this: u32, delete_flag: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const OPERATOR_DELETE: u32 = 2;
        const MEMORY_MANAGER_SLOT: u32 = 0x0167e2a0;
        const DELETE_REQUESTED: u32 = 0x01;

        let _ = lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if delete_flag & DELETE_REQUESTED != 0 {
            let manager = (lf_checker_rt::relocated(MEMORY_MANAGER_SLOT) as *const u32).read();
            let _ = lf_checker_rt::callee_thiscall!(OPERATOR_DELETE, u32, manager, this);
        }
        this
    }
});
