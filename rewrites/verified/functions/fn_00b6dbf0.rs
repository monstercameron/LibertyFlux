// original: 0x00B6DBF0 CTaskSimpleStartCar::~CTaskSimpleStartCar

use lf_checker_rt::{callee_thiscall, relocated};

/// Release the timing node owned by a start-car task, install its animation
/// task vtable, and forward destruction to the animation-task base.
///
/// The this pointer addresses the task record. The timing-node field is at
/// `+0x34`; when non-null, the detach helper receives the node in ECX and the
/// address of that field as its stack argument. The vtable word is loaded
/// from the relocated image reference. The base destructor is a tail call,
/// so its EAX result is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00B6DBF0(task: u32) -> u32 {
    unsafe {
        const TIMING_NODE: u32 = 0x34;
        const TASK_VTABLE: u32 = 0x00EB_2014;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_word(task, relocated(TASK_VTABLE));
        if read_word(task.wrapping_add(TIMING_NODE)) != 0 {
            let timing_node_slot = task.wrapping_add(TIMING_NODE);
            let _ = callee_thiscall!(1, u32, read_word(timing_node_slot), timing_node_slot);
        }
        callee_thiscall!(2, u32, task)
    }
});
