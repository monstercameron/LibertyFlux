// original: 0x00B37A30 ped_task_routine_00b37a30

/// Queries an actor and its task, runs the selected callback chain, and
/// forwards the task's stored float to its XMM0 callback.
///
/// The proved path has both lookup helpers return non-null objects, uses null
/// results from the two object-query calls, and takes the branch selected by
/// the subobject's all-ones marker. On that path, the same stored `f32` is
/// passed first in XMM0 and then remains the low XMM0 value for the following
/// no-argument call. The contract transports that value through a synthetic
/// rewrite stack word, compares the full XMM0 register and checks ECX at each
/// thiscall.
///
/// The contract does not compare stack contents because the original uses a
/// local scratch word for this float. ESP adjustment remains checked. The
/// non-null query routes, threshold branch, and final task action are outside
/// this proof.
lf_checker_rt::export!(cdecl, rw_00B37A30(input_a: u32, input_b: u32) -> u32 {
    unsafe {
        const LOOKUP_ID: u32 = 1;
        const BUILD_ID: u32 = 2;
        const VTABLE_D0_ID: u32 = 3;
        const FIRST_USE_ID: u32 = 4;
        const SECOND_USE_ID: u32 = 5;
        const ATTACH_ID: u32 = 6;
        const SUBOBJECT_ID: u32 = 7;
        const QUERY_ID: u32 = 8;
        const ENCRYPTED_HELPER_ID: u32 = 9;
        const UPDATE_ID: u32 = 10;
        const FLOAT_METHOD_ID: u32 = 11;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let actor = lf_checker_rt::callee_cdecl!(LOOKUP_ID, u32, input_a, input_b);
        if actor == 0 {
            return 0;
        }
        let task = lf_checker_rt::callee_cdecl!(BUILD_ID, u32, actor, 0, 0, 1, 1);
        if task == 0 {
            return 0;
        }

        let first_item = lf_checker_rt::callee_thiscall!(VTABLE_D0_ID, u32, task, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(FIRST_USE_ID, u32, first_item);
        let second_item = lf_checker_rt::callee_thiscall!(VTABLE_D0_ID, u32, task, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(SECOND_USE_ID, u32, second_item);

        let subobject = unsafe { read_u32(task.wrapping_add(0x7B4)) };
        let attached_value = unsafe { read_u32(task.wrapping_add(0x20)) };
        let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH_ID, u32, subobject, attached_value);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SUBOBJECT_ID,
            u32,
            subobject,
            subobject.wrapping_add(0x10)
        );

        let actor_query = lf_checker_rt::callee_thiscall!(QUERY_ID, u32, actor);
        let selected_actor = if actor_query == 0 {
            unsafe { read_u32(actor.wrapping_add(0x100)) }
        } else {
            let selected = lf_checker_rt::callee_thiscall!(QUERY_ID, u32, actor);
            lf_checker_rt::callee_thiscall!(VTABLE_D0_ID, u32, selected, 0)
        };
        let task_query = lf_checker_rt::callee_thiscall!(QUERY_ID, u32, task);
        let selected_task = if task_query == 0 {
            unsafe { read_u32(task.wrapping_add(0x100)) }
        } else {
            let selected = lf_checker_rt::callee_thiscall!(QUERY_ID, u32, task);
            lf_checker_rt::callee_thiscall!(VTABLE_D0_ID, u32, selected, 0)
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            ENCRYPTED_HELPER_ID,
            u32,
            selected_task,
            selected_actor
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(UPDATE_ID, u32, task);

        let first_layer = unsafe { read_u32(subobject.wrapping_add(4)) };
        let second_layer = unsafe { read_u32(first_layer.wrapping_add(0xC)) };
        let stored_float = unsafe { read_u32(second_layer.wrapping_add(8)) };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FLOAT_METHOD_ID,
            u32,
            subobject,
            stored_float
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FLOAT_METHOD_ID,
            u32,
            subobject,
            stored_float
        );

        if unsafe { (subobject.wrapping_add(8) as *const u16).read_unaligned() } == u16::MAX {
            return 0;
        }
        0
    }
});
