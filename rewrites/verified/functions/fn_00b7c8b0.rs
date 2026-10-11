// original: 0x00b7c8b0 CTaskComplexCarDriveTimed::vf19

/// Create a timed-drive task by allocating a fresh object from the shared
/// task pool and passing the source object's duration field to its
/// constructor. Allocation failure returns zero. The constructor receives
/// the fresh object in ECX, the dword at source offset `+0x24`, and a zero
/// second argument; its EAX result is returned unchanged.
///
/// Original ABI: thiscall with one consumed stack argument. The incoming
/// stack argument is not read, but the callee removes it on return.
lf_checker_rt::export!(thiscall, rw_00b7c8b0(this: u32, _unused: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x0167e2a0;
        const SOURCE_DURATION: u32 = 0x24;

        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if fresh == 0 {
            return 0;
        }
        let duration = ((this + SOURCE_DURATION) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, fresh, duration, 0)
    }
});
