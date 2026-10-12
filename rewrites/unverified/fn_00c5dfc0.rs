// original: 0x00C5DFC0 CTaskSimpleNewGangDriveBy::vf1
/// Allocate a new gang-driveby task copy and forward the source, embedded driveby-state address, scalar blend bits, and weapon state to its initializer.
///
/// The relocated pool global is passed to an intercepted thiscall allocator.
/// The successful path calls the task-specific initializer; the null path returns zero.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5dfc0(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }
        const SOURCE_TASK_OFFSET: u32 = 0x20;
        let source_task = core::ptr::read_unaligned(this.wrapping_add(SOURCE_TASK_OFFSET) as *const u32);
        const DRIVEBY_STATE_OFFSET: u32 = 0x30;
        let driveby_state = this.wrapping_add(DRIVEBY_STATE_OFFSET);
        const BLEND_BITS_OFFSET: u32 = 0x40;
        let blend_bits = core::ptr::read_unaligned(this.wrapping_add(BLEND_BITS_OFFSET) as *const u32);
        const WEAPON_STATE_OFFSET: u32 = 0x4C;
        let weapon_state = core::ptr::read_unaligned(this.wrapping_add(WEAPON_STATE_OFFSET) as *const u32);
        lf_checker_rt::callee_thiscall!(2, u32, allocated, source_task, driveby_state, blend_bits, weapon_state)
    }
}
