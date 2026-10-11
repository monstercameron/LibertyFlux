// original: 0x0095CED0 append_task_pointer_record
/// Append one pointer record when the shared task-output writer is ready.
///
/// This cdecl wrapper requires the ready flag to be set, the busy flag to be
/// clear, and the scripted readiness predicate to return a nonzero low byte.
/// It also applies the optional secondary-mode gate, acquires the writer lock,
/// resolves an unassigned resource id at object offset 0x64, and asks the
/// capacity helper to grow the output area when the 0x0c-byte record would
/// exceed the limit. It writes the record at base plus cursor, advances the
/// cursor by 0x0c, writes a zero terminator, and releases the lock on either
/// locked exit. The function has no semantic return value.
lf_checker_rt::export!(cdecl, rw_0095ced0(object: u32) -> () {
    const READY_FLAG_VA: u32 = 0x010376E9;
    const BUSY_FLAG_VA: u32 = 0x011F7076;
    const SECONDARY_FLAG_VA: u32 = 0x01037868;
    const SECONDARY_MODE_VA: u32 = 0x011F6FFA;
    const OBJECT_RESOURCE_ID: u32 = 0x64;
    const UNASSIGNED_RESOURCE_ID: u32 = u32::MAX;
    const OUTPUT_OFFSET_VA: u32 = 0x011FA008;
    const OUTPUT_BASE_VA: u32 = 0x011FA00C;
    const LOCK_CONTEXT_VA: u32 = 0x0120F270;
    const RECORD_BYTES: u32 = 0x0c;
    const OUTPUT_LIMIT: u32 = 0x00895430;
    const CHECK_READY: u32 = 1;
    const LOCK_OUTPUT: u32 = 2;
    const RESOLVE_RESOURCE_ID: u32 = 3;
    const ENSURE_OUTPUT_CAPACITY: u32 = 4;
    const WRITE_TASK_POINTER: u32 = 5;
    const UNLOCK_OUTPUT: u32 = 6;

    unsafe {
        if lf_checker_rt::global::<u8>(READY_FLAG_VA).read() == 0
            || lf_checker_rt::global::<u8>(BUSY_FLAG_VA).read() != 0
        {
            return;
        }
        if lf_checker_rt::global::<u8>(SECONDARY_FLAG_VA).read() != 0
            && lf_checker_rt::global::<u8>(SECONDARY_MODE_VA).read() != 2
        {
            return;
        }
        if lf_checker_rt::callee_cdecl!(CHECK_READY, u32,) as u8 == 0 {
            return;
        }

        let lock_context = lf_checker_rt::relocated(LOCK_CONTEXT_VA);
        let _: u32 = lf_checker_rt::callee_thiscall!(LOCK_OUTPUT, u32, lock_context);

        let resource_id = (object.wrapping_add(OBJECT_RESOURCE_ID) as *const u32)
            .read_unaligned();
        if resource_id == UNASSIGNED_RESOURCE_ID {
            let resolved_id = lf_checker_rt::callee_cdecl!(RESOLVE_RESOURCE_ID, u32, object);
            (object.wrapping_add(OBJECT_RESOURCE_ID) as *mut u32)
                .write_unaligned(resolved_id);
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
            WRITE_TASK_POINTER,
            u32,
            output_cursor,
            object
        );
        let next_offset = output_offset.wrapping_add(RECORD_BYTES);
        lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA).write_unaligned(next_offset);
        (output_base.wrapping_add(next_offset) as *mut u8).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_OUTPUT, u32, lock_context);
    }
});
