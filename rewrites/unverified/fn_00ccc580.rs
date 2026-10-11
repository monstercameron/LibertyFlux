// original: 0x00CCC580 CTaskComplexMoveAboutInjured::vf18

/// Handle four event codes for the injured movement task. The receiver is a
/// task object whose dispatch object is at `+0x08` and whose current target is
/// at `+0x14`; the one stack argument is an event record. The event type is
/// obtained through dispatch slot `+0x0c`. Code `0xcd` sends a four-word pose
/// to the motion sink; code `0xcb` and the signed-greater-than `0x38d` family
/// may request a task action depending on the target kind bits at `+0x28`.
/// The exact signed comparison is intentional. Code `0x40c` also requires the
/// event word at `+0xd68` to be zero. Failed requests store the timeout bits at
/// task `+0x18`; other unmatched events return zero. The callee return value is
/// returned unchanged on the two action paths. The frame-local pose buffer's
/// fourth word is unwritten when no target exists; the contract pins that
/// incoming scratch word to zero and snapshots the buffer at the sink call.
///
/// ABI: thiscall, `ECX = task`, one stack word `event`, callee pops four bytes.
pub(crate) unsafe fn run(
    task: u32,
    event: u32,
    timeout_bits: u32,
) -> u32 {
    const TASK_DISPATCH_OBJECT: u32 = 0x08;
    const TASK_TARGET: u32 = 0x14;
    const TASK_TIMEOUT: u32 = 0x18;
    const DISPATCH_VTABLE: u32 = 0x00;
    const EVENT_TYPE_SLOT: u32 = 0x0c;
    const EVENT_PAYLOAD: u32 = 0x20;
    const EVENT_STATUS_WORD: u32 = 0x0d68;
    const TARGET_TYPE_FLAGS: u32 = 0x28;
    const TARGET_TYPE_SHIFT: u32 = 6;
    const TARGET_TYPE_MASK: u32 = 0x0f;
    const EVENT_USE_POSE: u32 = 0x00cd;
    const EVENT_START_ACTION: u32 = 0x00cb;
    const EVENT_TIMEOUT: u32 = 0x038d;
    const EVENT_TIMEOUT_SPECIAL: u32 = 0x040c;
    const ACTION_KIND_A: u32 = 2;
    const ACTION_KIND_B: u32 = 3;
    const CALLEE_MANAGER_LOOKUP: u32 = 2;
    const CALLEE_POSE_SINK: u32 = 3;
    const CALLEE_ACTION_SINK: u32 = 4;
    const CALLEE_TIMEOUT_ACTION: u32 = 5;
    const MANAGER_GLOBAL_VA: u32 = 0x0167_e2a0;
    const POSE_SCALE_BITS_VA: u32 = 0x00ee_f940;
    const POSE_PARAM_A_VA: u32 = 0x00ee_f944;
    const POSE_PARAM_B_VA: u32 = 0x00ee_f948;
    const POSE_PARAM_C_VA: u32 = 0x00ee_f94c;
    const POSE_SINK_WEIGHT: u32 = 0x4000_0000;
    const TIMEOUT_DURATION_MS: u32 = 3000;
    const TIMEOUT_ACTION_BITS: u32 = 0x4100_0000;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn relocated_word(file_va: u32) -> u32 {
        unsafe { read_u32(lf_checker_rt::relocated(file_va)) }
    }

    #[inline(always)]
    unsafe fn event_kind(dispatch_object: u32) -> u32 {
        unsafe {
            let vtable = read_u32(dispatch_object.wrapping_add(DISPATCH_VTABLE));
            let function = read_u32(vtable.wrapping_add(EVENT_TYPE_SLOT));
            let get_kind: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(function as usize);
            get_kind(dispatch_object)
        }
    }

    #[inline(always)]
    unsafe fn manager_context() -> u32 {
        unsafe { relocated_word(MANAGER_GLOBAL_VA) }
    }

    #[inline(always)]
    unsafe fn manager_lookup() -> u32 {
        unsafe {
            lf_checker_rt::callee_thiscall!(
                CALLEE_MANAGER_LOOKUP,
                u32,
                manager_context()
            )
        }
    }

    #[inline(always)]
    unsafe fn make_timeout(task: u32, timeout_bits: u32) -> u32 {
        unsafe {
            write_u32(task.wrapping_add(TASK_TIMEOUT), timeout_bits);
            let manager = manager_lookup();
            if manager == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                CALLEE_TIMEOUT_ACTION,
                u32,
                manager,
                TIMEOUT_DURATION_MS,
                0,
                0,
                TIMEOUT_ACTION_BITS
            )
        }
    }

    unsafe {
        let dispatch_object = read_u32(task.wrapping_add(TASK_DISPATCH_OBJECT));
        let code = event_kind(dispatch_object);

        if (code as i32) > EVENT_TIMEOUT as i32 {
            if code != EVENT_TIMEOUT_SPECIAL {
                return 0;
            }
            if read_u32(event.wrapping_add(EVENT_STATUS_WORD)) != 0 {
                return make_timeout(task, timeout_bits);
            }
            let target = read_u32(task.wrapping_add(TASK_TARGET));
            if target == 0 {
                return make_timeout(task, timeout_bits);
            }
            let target_kind = (read_u32(target.wrapping_add(TARGET_TYPE_FLAGS))
                >> TARGET_TYPE_SHIFT)
                & TARGET_TYPE_MASK;
            if target_kind != ACTION_KIND_A && target_kind != ACTION_KIND_B {
                return make_timeout(task, timeout_bits);
            }
            let manager = manager_lookup();
            if manager == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CALLEE_ACTION_SINK,
                u32,
                manager,
                target,
                0,
                relocated_word(POSE_SCALE_BITS_VA),
                relocated_word(POSE_PARAM_A_VA),
                relocated_word(POSE_PARAM_B_VA),
                relocated_word(POSE_PARAM_C_VA),
                0
            );
        }

        if code == EVENT_TIMEOUT {
            return make_timeout(task, timeout_bits);
        }

        if code == EVENT_START_ACTION {
            let target = read_u32(task.wrapping_add(TASK_TARGET));
            if target == 0 {
                return make_timeout(task, timeout_bits);
            }
            let target_kind = (read_u32(target.wrapping_add(TARGET_TYPE_FLAGS))
                >> TARGET_TYPE_SHIFT)
                & TARGET_TYPE_MASK;
            if target_kind != ACTION_KIND_A && target_kind != ACTION_KIND_B {
                return make_timeout(task, timeout_bits);
            }
            let manager = manager_lookup();
            if manager == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CALLEE_ACTION_SINK,
                u32,
                manager,
                target,
                0,
                relocated_word(POSE_SCALE_BITS_VA),
                relocated_word(POSE_PARAM_A_VA),
                relocated_word(POSE_PARAM_B_VA),
                relocated_word(POSE_PARAM_C_VA),
                0
            );
        }

        if code != EVENT_USE_POSE {
            return 0;
        }

        let event_payload = read_u32(event.wrapping_add(EVENT_PAYLOAD));
        let mut pose_words = [0u32; 4];
        pose_words[0] = read_u32(event_payload.wrapping_add(0x30));
        pose_words[1] = read_u32(event_payload.wrapping_add(0x34));
        pose_words[2] = read_u32(event_payload.wrapping_add(0x38));
        let target = read_u32(task.wrapping_add(TASK_TARGET));
        if target != 0 {
            let target_pose = read_u32(target.wrapping_add(0x20));
            let source = if target_pose != 0 {
                target_pose.wrapping_add(0x30)
            } else {
                target.wrapping_add(0x10)
            };
            for (index, word) in pose_words.iter_mut().enumerate() {
                *word = read_u32(source.wrapping_add((index as u32) * 4));
            }
        }

        let manager = manager_lookup();
        if manager == 0 {
            return 0;
        }
        let pose_pointer = pose_words.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_POSE_SINK,
            u32,
            manager,
            0,
            pose_pointer,
            0,
            1,
            event_payload.wrapping_add(0x30),
            POSE_SINK_WEIGHT,
            1
        )
    }
}

lf_checker_rt::export!(thiscall, rw_00ccc580(task: u32, event: u32) -> u32 {
    unsafe { run(task, event, 0x3c23_d70a) }
});
