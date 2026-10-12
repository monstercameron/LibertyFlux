// original: 0x00C5DF60 CTaskSimpleArrestPed::vf1
/// Allocate an arrest-task copy and pass the source task's subject pointer to its initializer. A failed allocation returns null without reading the source field.
///
/// The relocated pool global is passed to an intercepted thiscall allocator.
/// The successful path calls the task-specific initializer; the null path returns zero.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5df60(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }
        const SOURCE_TASK_OFFSET: u32 = 0x14;
        let source_task = core::ptr::read_unaligned(this.wrapping_add(SOURCE_TASK_OFFSET) as *const u32);
        lf_checker_rt::callee_thiscall!(2, u32, allocated, source_task)
    }
}
