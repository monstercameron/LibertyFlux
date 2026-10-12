// original: 0x00B6DC20 CTaskSimpleWaitUntilPedIsOutCar::~CTaskSimpleWaitUntilPedIsOutCar

use lf_checker_rt::{callee_thiscall, relocated};

/// Release the two timing nodes owned by an out-of-car wait task, install its
/// simple-task vtable, and forward destruction to the simple-task base.
///
/// The this pointer addresses the task record. Timing-node fields are at
/// `+0x14` and `+0x30`; each non-null node is detached with ECX set to the
/// node and the corresponding field address as its stack argument. The
/// vtable word is loaded from the relocated image reference. The base
/// destructor is a tail call, so its EAX result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00B6DC20(task: u32) -> u32 {
    unsafe {
        const FIRST_TIMING_NODE: u32 = 0x14;
        const SECOND_TIMING_NODE: u32 = 0x30;
        const TASK_VTABLE: u32 = 0x00EB_1FBC;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_word(task, relocated(TASK_VTABLE));
        for field_offset in [FIRST_TIMING_NODE, SECOND_TIMING_NODE] {
            let timing_node_slot = task.wrapping_add(field_offset);
            let timing_node = read_word(timing_node_slot);
            if timing_node != 0 {
                let _ = callee_thiscall!(1, u32, timing_node, timing_node_slot);
            }
        }
        callee_thiscall!(2, u32, task)
    }
});
