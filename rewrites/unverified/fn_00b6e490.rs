// original: 0x00B6E490 CTaskSimpleCarSetPedInVehicle::vf1

use lf_checker_rt::{callee_thiscall, relocated};

/// Allocate and construct the CTaskSimpleCarSetPedInVehicle task.
///
/// The thiscall entry receives the task description in ECX. It reads the
/// task-pool pointer from the relocated global slot, asks the pool for an
/// object, and returns zero if allocation fails. Otherwise it forwards the
/// listed fields from the description to the task-specific constructor.
/// Byte fields are zero-extended, float fields are forwarded as their raw
/// words, and the constructor's EAX result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00B6E490(task: u32) -> u32 {
    unsafe {
        const TASK_POOL_POINTER: u32 = 0x0167E2A0;
        const TASK_FIELD_14: u32 = 0x14;
        const TASK_FIELD_18: u32 = 0x18;
        const TASK_FIELD_1C: u32 = 0x1C;
        const TASK_FIELD_1D: u32 = 0x1D;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read_unaligned() }
        }

        let task_pool = read_word(relocated(TASK_POOL_POINTER));
        let allocated = callee_thiscall!(1, u32, task_pool);
        if allocated == 0 {
            return 0;
        }
        let constructor_result = callee_thiscall!(2, u32, allocated, read_word(task.wrapping_add(TASK_FIELD_14)), read_word(task.wrapping_add(TASK_FIELD_18)), read_byte(task.wrapping_add(TASK_FIELD_1C)) as u32, read_byte(task.wrapping_add(TASK_FIELD_1D)) as u32);
        constructor_result
    }
});
