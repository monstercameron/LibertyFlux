// original: 0x00C5E030 CTaskSimpleThrowGrenadeFromVehicle::vf1
/// Allocate a grenade-throw task copy and forward the source vehicle state and throw state to its initializer.
///
/// The relocated pool global is passed to an intercepted thiscall allocator.
/// The successful path calls the task-specific initializer; the null path returns zero.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e030(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if allocated == 0 {
            return 0;
        }
        const VEHICLE_STATE_OFFSET: u32 = 0x2C;
        let vehicle_state = core::ptr::read_unaligned(this.wrapping_add(VEHICLE_STATE_OFFSET) as *const u32);
        const THROW_STATE_OFFSET: u32 = 0x30;
        let throw_state = core::ptr::read_unaligned(this.wrapping_add(THROW_STATE_OFFSET) as *const u32);
        lf_checker_rt::callee_thiscall!(2, u32, allocated, vehicle_state, throw_state)
    }
}
