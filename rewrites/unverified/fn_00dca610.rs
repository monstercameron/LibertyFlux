// original: 0x00DCA610 CTaskComplexCarReactToVehicleCollision::vf0

/// Destroys the base collision-reaction task and optionally releases its
/// manager registration.
///
/// `this` is the task in ECX and `delete_flags` is the one stack argument.
/// The task destructor is called first. When the low bit of the flags is set,
/// the manager pointer stored in the manager global is passed with `this` to
/// the cleanup callee. The task pointer is returned in EAX and the caller's
/// single stack argument is removed by thiscall.
lf_checker_rt::export!(thiscall, rw_00dca610(this: u32, delete_flags: u32) -> u32 {
    unsafe {
        const TASK_DESTRUCTOR: u32 = 1;
        const RELEASE_MANAGER_ENTRY: u32 = 2;
        const MANAGER_GLOBAL_FILE_VA: u32 = 0x0167E2A0;

        let _ = lf_checker_rt::callee_thiscall!(TASK_DESTRUCTOR, u32, this);
        if delete_flags & 1 != 0 {
            let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_FILE_VA).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_MANAGER_ENTRY, u32, manager, this);
        }
        this
    }
});
