// original: 0x00b7b300 CTaskComplexStealCar::vf1

/// Clone this task by obtaining a fresh task from the shared task pool and
/// invoking its class-specific constructor.
///
/// `this` points to the source task. The method reads the pool pointer from
/// the shared slot, calls the pool allocator with that pointer in ECX, and
/// returns zero if allocation fails. Otherwise it forwards the source
/// object's 32-bit value at +0x14 to the constructor with the new object in
/// ECX, returning the constructor's EAX result. The pool lookup and both
/// calls are observable; neither call is executed natively by the checker.
///
/// Original ABI: thiscall with no stack arguments; the source object's
/// constructor argument at +0x14 is a dword.
lf_checker_rt::export!(thiscall, rw_00b7b300(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x0167e2a0;
        const CONSTRUCTOR_ARGUMENT: u32 = 0x14;

        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if fresh == 0 {
            return 0;
        }

        let source_argument = ((this + CONSTRUCTOR_ARGUMENT) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, fresh, source_argument)
    }
});
