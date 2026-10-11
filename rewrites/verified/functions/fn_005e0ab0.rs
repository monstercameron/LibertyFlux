// original: 0x005E0AB0 CTaskComplexCarSetTempAction::create_first_sub_task

/// Allocate a basic-drive task through the global task pool. If allocation
/// succeeds, pass the character's current task pointer and a zero flag to the
/// constructor and return its result. If allocation fails, return zero. The
/// input character is a thiscall stack argument; the incoming ECX value is
/// unused.
lf_checker_rt::export!(thiscall, rw_005e0ab0(_this: u32, character: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const CHARACTER_TASK: u32 = 0x0b30;
        const TASK_POOL_ALLOC: u32 = 1;
        const CONSTRUCT_BASIC_DRIVE: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let pool_manager = *lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
        let task_object = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, pool_manager);
        if task_object == 0 {
            return 0;
        }

        let current_task = read_u32(character.wrapping_add(CHARACTER_TASK));
        lf_checker_rt::callee_thiscall!(CONSTRUCT_BASIC_DRIVE, u32, task_object, current_task, 0)
    }
});
