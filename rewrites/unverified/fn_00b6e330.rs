// original: 0x00B6E330 CTaskSimpleCarCloseDoorFromInside::vf1

use lf_checker_rt::{callee_thiscall, relocated};

/// Allocate and construct the CTaskSimpleCarCloseDoorFromInside task.
///
/// The thiscall entry receives the task description in ECX. It reads the
/// task-pool pointer from the relocated global slot, asks the pool for an
/// object, and returns zero if allocation fails. Otherwise it forwards the
/// listed fields from the description to the task-specific constructor.
/// Byte fields are zero-extended, float fields are forwarded as their raw
/// words, and the constructor's EAX result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00B6E330(task: u32) -> u32 {
    unsafe {
        const TASK_POOL_POINTER: u32 = 0x0167E2A0;
        const TASK_FIELD_24: u32 = 0x24;
        const TASK_FIELD_28: u32 = 0x28;

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
        let constructor_result = callee_thiscall!(2, u32, allocated, read_word(task.wrapping_add(TASK_FIELD_24)), read_word(task.wrapping_add(TASK_FIELD_28)));
        constructor_result
    }
});
