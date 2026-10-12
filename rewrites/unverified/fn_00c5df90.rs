// original: 0x00C5DF90 CTaskSimpleBeHit::vf1
/// Allocate a hit-reaction task copy and forward its subject, impact state, impact data, and zero-extended impact kind to the initializer.
///
/// The relocated pool global is passed to an intercepted thiscall allocator.
/// The successful path calls the task-specific initializer; the null path returns zero.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5df90(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }
        const SOURCE_TASK_OFFSET: u32 = 0x14;
        let source_task = core::ptr::read_unaligned(this.wrapping_add(SOURCE_TASK_OFFSET) as *const u32);
        const IMPACT_STATE_OFFSET: u32 = 0x1C;
        let impact_state = core::ptr::read_unaligned(this.wrapping_add(IMPACT_STATE_OFFSET) as *const u32);
        const IMPACT_DATA_OFFSET: u32 = 0x20;
        let impact_data = core::ptr::read_unaligned(this.wrapping_add(IMPACT_DATA_OFFSET) as *const u32);
        const IMPACT_KIND_OFFSET: u32 = 0x19;
        let impact_kind = (this.wrapping_add(IMPACT_KIND_OFFSET) as *const u8).read() as u32;
        lf_checker_rt::callee_thiscall!(2, u32, allocated, source_task, impact_state, impact_data, impact_kind)
    }
}
