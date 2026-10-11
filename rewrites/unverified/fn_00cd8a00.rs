// original: 0x00CD8A00 CTaskComplexFollowLeaderInFormation::vf19

/// Notify the current task helper, then choose between the two existing
/// follow subtasks when the target and peer share an active handle. Otherwise
/// resolve the target's task, acquire the required contexts, optionally
/// construct a nested task, and forward both results to the final creator.
/// Null intermediate results return zero at the same points as the original.
///
/// Target flags and handle live at offsets `0x26C` and `0xB30`; the peer
/// pointer is at `this + 0x18`. The repeated context lookups and helper calls
/// are kept in their original order, and the nested task's vtable is stored
/// as a relocated image pointer.
///
/// Calling convention: thiscall with one 32-bit stack argument. Outgoing
/// helper calls use thiscall; their return values are forwarded or used as
/// pointers according to the branch.
lf_checker_rt::export!(thiscall, rw_00cd8a00(this: u32, target: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const TARGET_FLAGS: u32 = 0x26C;
    const TARGET_HANDLE: u32 = 0xB30;
    const PEER_POINTER: u32 = 0x18;
    const NESTED_TASK_VTABLE: u32 = 0x00EB_391C;
    const NOTIFY_TASK: u32 = 1;
    const SAME_HANDLE_TASK: u32 = 2;
    const OTHER_HANDLE_TASK: u32 = 3;
    const RESOLVE_TARGET: u32 = 4;
    const GET_FIRST_CONTEXT: u32 = 5;
    const MAKE_FIRST_TASK: u32 = 6;
    const GET_SECOND_CONTEXT: u32 = 7;
    const GET_NESTED_TASK: u32 = 8;
    const BASE_CONSTRUCTOR: u32 = 9;
    const MAKE_FINAL_TASK: u32 = 10;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(NOTIFY_TASK, u32, this, target);
        let target_flags = ((target + TARGET_FLAGS) as *const u8).read();
        if target_flags & 4 != 0 {
            let target_handle = ((target + TARGET_HANDLE) as *const u32).read_unaligned();
            if target_handle != 0 {
                let peer = ((this + PEER_POINTER) as *const u32).read_unaligned();
                let peer_flags = ((peer + TARGET_FLAGS) as *const u8).read();
                let peer_handle = ((peer + TARGET_HANDLE) as *const u32).read_unaligned();
                if peer_flags & 4 != 0 && peer_handle == target_handle {
                    return lf_checker_rt::callee_thiscall!(SAME_HANDLE_TASK, u32, this, 0x2D4, target);
                }
                return lf_checker_rt::callee_thiscall!(OTHER_HANDLE_TASK, u32, this, 0x2E2, target);
            }
        }

        let target_task = lf_checker_rt::callee_thiscall!(RESOLVE_TARGET, u32, target);
        if target_task == 0 {
            return 0;
        }

        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let first_context = lf_checker_rt::callee_thiscall!(GET_FIRST_CONTEXT, u32, context);
        let first_task = if first_context == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(MAKE_FIRST_TASK, u32, first_context, target_task)
        };

        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let second_context = lf_checker_rt::callee_thiscall!(GET_SECOND_CONTEXT, u32, context);
        if second_context == 0 {
            return 0;
        }

        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let nested_task = lf_checker_rt::callee_thiscall!(GET_NESTED_TASK, u32, context);
        if nested_task != 0 {
            let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, nested_task);
            (nested_task as *mut u32).write_unaligned(lf_checker_rt::relocated(NESTED_TASK_VTABLE));
            ((nested_task + 0x14) as *mut u32).write_unaligned(0);
            ((nested_task + 0x18) as *mut u8).write(0);
            ((nested_task + 0x1C) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(
            MAKE_FINAL_TASK,
            u32,
            second_context,
            first_task,
            nested_task,
            0,
            0
        )
    }
});
