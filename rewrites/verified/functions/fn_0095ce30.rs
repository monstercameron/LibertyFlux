// original: 0x0095CE30 append_task_object_record
/// Append one object record through the shared task-output writer.
///
/// The cdecl function takes four words and treats its third word as an
/// optional object pointer. It checks the shared writer gates and scripted
/// readiness predicate, acquires the writer lock, and resolves an unassigned
/// object resource id at offset 0x64. If a 0x20-byte record would exceed the
/// output limit it calls the capacity helper and leaves through the lock
/// release path when growth is refused. Otherwise it calls the thiscall writer
/// with the base-plus-cursor pointer and the four original words, advances the
/// cursor by 0x20, writes a zero terminator, and releases the lock. It has no
/// semantic return value.
lf_checker_rt::export!(cdecl, rw_0095ce30(arg1: u32, arg2: u32, object: u32, arg4: u32) -> () {
    const READY_FLAG_VA: u32 = 0x010376E9;
    const BUSY_FLAG_VA: u32 = 0x011F7076;
    const OBJECT_RESOURCE_ID: u32 = 0x64;
    const UNASSIGNED_RESOURCE_ID: u32 = u32::MAX;
    const OUTPUT_OFFSET_VA: u32 = 0x011FA008;
    const OUTPUT_BASE_VA: u32 = 0x011FA00C;
    const LOCK_CONTEXT_VA: u32 = 0x0120F270;
    const RECORD_BYTES: u32 = 0x20;
    const OUTPUT_LIMIT: u32 = 0x00895430;
    const CHECK_READY: u32 = 1;
    const LOCK_OUTPUT: u32 = 2;
    const RESOLVE_RESOURCE_ID: u32 = 3;
    const ENSURE_OUTPUT_CAPACITY: u32 = 4;
    const WRITE_TASK_OBJECT: u32 = 5;
    const UNLOCK_OUTPUT: u32 = 6;

    unsafe {
        if lf_checker_rt::global::<u8>(READY_FLAG_VA).read() == 0
            || lf_checker_rt::global::<u8>(BUSY_FLAG_VA).read() != 0
        {
            return;
        }
        if lf_checker_rt::callee_cdecl!(CHECK_READY, u32,) as u8 == 0 {
            return;
        }

        let lock_context = lf_checker_rt::relocated(LOCK_CONTEXT_VA);
        let _: u32 = lf_checker_rt::callee_thiscall!(LOCK_OUTPUT, u32, lock_context);

        if object != 0 {
            let resource_id = (object.wrapping_add(OBJECT_RESOURCE_ID) as *const u32)
                .read_unaligned();
            if resource_id == UNASSIGNED_RESOURCE_ID {
                let resolved_id = lf_checker_rt::callee_cdecl!(RESOLVE_RESOURCE_ID, u32, object);
                (object.wrapping_add(OBJECT_RESOURCE_ID) as *mut u32)
                    .write_unaligned(resolved_id);
            }
        }

        let mut output_offset = lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA)
            .read_unaligned();
        if output_offset.wrapping_add(RECORD_BYTES) > OUTPUT_LIMIT {
            let can_grow = lf_checker_rt::callee_cdecl!(ENSURE_OUTPUT_CAPACITY, u32,);
            if can_grow as u8 == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_OUTPUT, u32, lock_context);
                return;
            }
            output_offset = lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA)
                .read_unaligned();
        }

        let output_base = lf_checker_rt::global::<u32>(OUTPUT_BASE_VA).read_unaligned();
        let output_cursor = output_base.wrapping_add(output_offset);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            WRITE_TASK_OBJECT,
            u32,
            output_cursor,
            arg1,
            arg2,
            object,
            arg4
        );
        let next_offset = output_offset.wrapping_add(RECORD_BYTES);
        lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA).write_unaligned(next_offset);
        (output_base.wrapping_add(next_offset) as *mut u8).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_OUTPUT, u32, lock_context);
    }
});
