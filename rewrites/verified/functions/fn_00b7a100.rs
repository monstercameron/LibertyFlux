// original: 0x00B7A100 CTaskComplexDrivePointRoute::~CTaskComplexDrivePointRoute__deleting

/// Deleting destructor wrapper for CTaskComplexDrivePointRoute::~CTaskComplexDrivePointRoute.
///
/// Calls the class destructor first, then calls the task manager's delete routine when the low bit of `flag` is set. The flag is read as one byte, the manager handle is read from its relocated global slot, and the function returns `this`. The original uses the 32-bit thiscall ABI with one stack word.
lf_checker_rt::export!(thiscall, rw_00b7a100(this: u32, flag: u32) -> u32 {
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
