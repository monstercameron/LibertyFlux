// original: 0x009EAFF0 process_ped_task_state_flags
use lf_checker_rt::{callee_thiscall, export};

const FIRST_HELPER: u32 = 1;
const SECOND_HELPER: u32 = 2;
const QUERY_HELPER: u32 = 3;
const PED_STATE_HELPER: u32 = 4;
const TASK_EVENT_HELPER: u32 = 5;
const TASK_MANAGER: u32 = 0x78;
const QUERY_RESULT_OBJECT: u32 = 4;
const QUERY_RESULT_FLAGS: u32 = 0x74;
const TASK_STATE_BUFFER: u32 = 0xA90;
const TASK_CONTROLLER: u32 = 0xBB0;
const TASK_RUNTIME: u32 = 0x3C0;
const QUERY_KIND: u32 = 0x0100_0000;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_byte(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn call_state_helper(this: u32, state_buffer: u32) {
    let _ = unsafe { callee_thiscall!(PED_STATE_HELPER, u32, this, state_buffer) };
}

#[inline(always)]
unsafe fn call_task_event(this: u32, event_kind: u32) {
    let _ = unsafe { callee_thiscall!(TASK_EVENT_HELPER, u32, this, event_kind, 0) };
}

/// Updates a task component after the task manager reports availability.
/// It first probes the manager stored at `ped+0x78`; a zero answer exits before
/// any state changes. Otherwise it updates the task runtime, asks the manager
/// to fill an output object through four temporary pointer arguments, and
/// follows the selected object's readiness byte and event flags. The object
/// flags choose the fallback result slot when neither of two bits is set;
/// its low byte controls the signed event branch, while bits `0x20`, `0x40`,
/// and `0x100` schedule the corresponding state and event helper calls.
///
/// Each intercepted call uses thiscall. The query helper takes eight stack
/// words; its fifth argument writes the selected object pointer back to a
/// temporary local. Event helper arguments are pushed in reverse order, so
/// their first logical argument is the event kind and the second is zero.
unsafe fn update_task_state(ped: u32) {
    let manager = unsafe { read_word(ped.wrapping_add(TASK_MANAGER)) };
    let first_receiver = manager;
    let available = unsafe { callee_thiscall!(FIRST_HELPER, u32, first_receiver) };
    if available == 0 {
        return;
    }

    let _ = unsafe { callee_thiscall!(SECOND_HELPER, u32, ped) };

    let mut selected_object = 0u32;
    let mut result_two = 0u32;
    let mut result_three = 0u32;
    let mut result_four = 0u32;
    let selected_object_ptr = &mut selected_object as *mut u32 as u32;
    let result_two_ptr = &mut result_two as *mut u32 as u32;
    let result_three_ptr = &mut result_three as *mut u32 as u32;
    let result_four_ptr = &mut result_four as *mut u32 as u32;
    let _ = unsafe {
        callee_thiscall!(
            QUERY_HELPER,
            u32,
            manager,
            0,
            4,
            QUERY_KIND,
            1,
            selected_object_ptr,
            result_four_ptr,
            result_two_ptr,
            result_three_ptr,
        )
    };

    let mut object = selected_object;
    let object_word = unsafe { read_word(object.wrapping_add(QUERY_RESULT_OBJECT)) };
    let object_or_flag_a = (object_word >> 0x15) & 1;
    let object_or_flag_b = (object_word >> 0x18) & 1;
    if object_or_flag_a != 0 || object_or_flag_b == 0 {
        object = result_two;
        selected_object = object;
    }
    if object == 0 || unsafe { read_byte(object.wrapping_add(7)) } & 1 == 0 {
        return;
    }

    let flags = unsafe { read_word(object.wrapping_add(QUERY_RESULT_FLAGS)) };
    let runtime = ped.wrapping_add(TASK_RUNTIME);
    if flags & 0x20 != 0 {
        let state_buffer = ped.wrapping_add(TASK_STATE_BUFFER);
        let controller = ped.wrapping_add(TASK_CONTROLLER);
        unsafe {
            call_state_helper(controller, state_buffer);
            call_task_event(runtime, 0);
        }
    }
    if (flags as u8 as i8) < 0 {
        unsafe { call_task_event(runtime, 2) };
    }
    if flags & 0x40 != 0 {
        let state_buffer = ped.wrapping_add(TASK_STATE_BUFFER);
        let controller = ped.wrapping_add(TASK_CONTROLLER);
        unsafe {
            call_state_helper(controller, state_buffer);
            call_task_event(runtime, 1);
        }
    }
    if flags & 0x100 != 0 {
        unsafe { call_task_event(runtime, 3) };
    }
}

export!(thiscall, rw_009eaff0(ped: u32) -> () {
    unsafe { update_task_state(ped) }
});

