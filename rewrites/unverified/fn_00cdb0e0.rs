// original: 0x00CDB0E0 CTaskSimpleNMRelax::vf0

/// Scalar deleting destructor for CTaskSimpleNMRelax.
///
/// Runs the class destructor on `this` first (patched callee), then, when bit
/// 0 of `flag` is set, frees the object through the task manager's delete
/// routine (patched callee) with the manager handle read from its global
/// slot. Returns `this` either way. The flag is tested as one byte; only its
/// low bit matters.
///
/// Original: 0x00CDB0E0 (thiscall: `this` in ecx, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb0e0(this: u32, flag: u32) -> u32 {
    unsafe {
        const CLASS_DTOR: u32 = 1;
        const TASK_DELETE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;

        lf_checker_rt::callee_thiscall!(CLASS_DTOR, u32, this);
        if (flag & 1) != 0 {
            let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
            lf_checker_rt::callee_thiscall!(TASK_DELETE, u32, manager, this);
        }
        this
    }
});
