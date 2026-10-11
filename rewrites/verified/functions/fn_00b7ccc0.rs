// original: 0x00b7ccc0 CTaskComplexPlayerDrive::vf19

/// Clear the shared last-task slot, notify the task setup helper using the
/// incoming argument, and allocate a new player-drive task from the shared
/// pool. Allocation failure returns zero. Otherwise the constructor receives
/// the source object's dword at `+0x28` and a zero argument, and its return
/// value is forwarded unchanged.
///
/// Original ABI: thiscall with one stack argument. The setup call takes the
/// incoming value in ECX with stack arguments zero and `-1`; the latter is
/// the saved sentinel word the original setup call consumes. The allocator
/// and constructor calls are also intercepted by the contract.
lf_checker_rt::export!(thiscall, rw_00b7ccc0(this: u32, setup_argument: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x0167e2a0;
        const LAST_TASK_SLOT: u32 = 0x0167e2ac;
        const CONSTRUCTOR_VALUE: u32 = 0x28;
        lf_checker_rt::global::<u32>(LAST_TASK_SLOT).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, setup_argument, 0, u32::MAX);
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(2, u32, pool);
        if fresh == 0 {
            return 0;
        }
        let constructor_value = ((this + CONSTRUCTOR_VALUE) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(3, u32, fresh, constructor_value, 0)
    }
});
