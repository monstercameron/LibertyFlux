// original: 0x00D2CB70 CTaskComplexUseLadderOnRoute::vf20
use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated};

const TIMER_CLOCK_ADDRESS: u32 = 0x0117_35B4;
const TIMER_ENABLED: u32 = 0x44;
const TIMER_RESET: u32 = 0x45;
const TIMER_BASELINE: u32 = 0x3C;
const TIMER_DELAY: u32 = 0x40;
const TASK_FLAGS: u32 = 0x0C;
const TASK_OWNER: u32 = 0x08;
const TASK_ROUTE: u32 = 0x34;
const TASK_VECTOR: u32 = 0x20;
const TASK_VECTOR_W: u32 = 0x30;
const OWNER_KIND: u32 = 0x3A6;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write_unaligned(value) }
}

/// Advance the ladder-on-route task for one route traversal.
///
/// The task arrives in ECX and the traversal pointer is the stack argument.
/// The traversal pointer is forwarded to the task's readiness virtual call.
/// The timer comparison is signed; an enabled due timer calls that virtual
/// readiness method and returns zero. Otherwise the task owner is queried
/// for exact route kind 934. That route kind copies raw float words through
/// the route-vector helper into the owner's output fields. The helper's two
/// stack pointers overlap in one five-word scratch array.
unsafe fn ladder_route_impl(
    task: u32,
    traversal: u32,
    expected_owner_kind: u32,
    ready_flag_mask: u32,
) -> u32 {
    unsafe {
        if read_u8(task + TIMER_ENABLED) != 0 {
            if read_u8(task + TIMER_RESET) != 0 {
                let current_time = read_u32(relocated(TIMER_CLOCK_ADDRESS));
                write_u32(task + TIMER_BASELINE, current_time);
                write_u8(task + TIMER_RESET, 0);
            }

            let deadline = read_u32(task + TIMER_DELAY)
                .wrapping_add(read_u32(task + TIMER_BASELINE));
            let current_time = read_u32(relocated(TIMER_CLOCK_ADDRESS));
            if (deadline as i32) <= (current_time as i32) {
                if read_u32(task + TASK_FLAGS) & 1 == 0 {
                    let ready: u32 = callee_thiscall!(1, u32, task, traversal, 1, 0);
                    if ready & 0xFF != 0 {
                        let task_flags = read_u32(task + TASK_FLAGS);
                        write_u32(task + TASK_FLAGS, task_flags | ready_flag_mask);
                    }
                }
                return 0;
            }
        }

        let owner = read_u32(task + TASK_OWNER);
        let route = read_u32(task + TASK_ROUTE);
        if route != 0 {
            let owner_kind: u32 = callee_thiscall!(2, u32, owner);
            if owner_kind == expected_owner_kind {
                let route_interval = read_u32(route + TASK_VECTOR);
                let mut scratch_vectors = [
                    read_u32(task + TASK_VECTOR_W),
                    read_u32(task + TASK_VECTOR),
                    read_u32(task + TASK_VECTOR + 4),
                    read_u32(task + TASK_VECTOR + 8),
                    0,
                ];

                if route_interval == 0 {
                    let _route_state: u32 = callee_thiscall!(3, u32, route);
                    let _route_duration: u32 =
                        callee_thiscall!(4, u32, route + 0x10, route_interval);
                }

                let scratch = scratch_vectors.as_mut_ptr() as u32;
                let _vector_result: u32 = callee_cdecl!(5, u32, scratch + 4, scratch, route_interval);
                write_u32(owner + 0xC0, scratch_vectors[1]);
                write_u32(owner + 0xC4, scratch_vectors[2]);
                write_u32(owner + 0xC8, scratch_vectors[3]);
                write_u32(owner + 0xCC, scratch_vectors[4]);
                write_u32(owner + 0xD8, scratch_vectors[0]);
            }
        }

        owner
    }
}

lf_checker_rt::export!(thiscall, rw_00d2cb70(task: u32, traversal: u32) -> u32 {
    unsafe { ladder_route_impl(task, traversal, OWNER_KIND, 2) }
});


