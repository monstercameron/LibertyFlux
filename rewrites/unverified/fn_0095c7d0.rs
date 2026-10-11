// original: 0x0095C7D0 append_task_value
/// Emit one task value after resolving its object resource id when necessary.
///
/// The cdecl wrapper takes an object pointer and a value. If the object's
/// resource id at byte offset 0x64 is 0xffffffff, it calls the id resolver and
/// stores the returned word there. It then calls the thiscall writer with the
/// shared output cursor in ECX and the object and value as stack arguments,
/// advances the cursor by 0x20 with 32-bit wrapping arithmetic, and returns the
/// writer's EAX value unchanged.
lf_checker_rt::export!(cdecl, rw_0095c7d0(object: u32, value: u32) -> u32 {
    const OBJECT_RESOURCE_ID: u32 = 0x64;
    const UNASSIGNED_RESOURCE_ID: u32 = u32::MAX;
    const OUTPUT_OFFSET_VA: u32 = 0x0120F290;
    const OUTPUT_BASE_VA: u32 = 0x0120F294;
    const RESOLVE_RESOURCE_ID: u32 = 1;
    const WRITE_TASK_VALUE: u32 = 2;
    const OUTPUT_BYTES: u32 = 0x20;

    unsafe {
        let resource_id = (object.wrapping_add(OBJECT_RESOURCE_ID) as *const u32)
            .read_unaligned();
        if resource_id == UNASSIGNED_RESOURCE_ID {
            let resolved_id = lf_checker_rt::callee_cdecl!(
                RESOLVE_RESOURCE_ID,
                u32,
                object
            );
            (object.wrapping_add(OBJECT_RESOURCE_ID) as *mut u32)
                .write_unaligned(resolved_id);
        }

        let output_offset = lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA).read_unaligned();
        let output_base = lf_checker_rt::global::<u32>(OUTPUT_BASE_VA).read_unaligned();
        let output_cursor = output_base.wrapping_add(output_offset);
        let writer_result = lf_checker_rt::callee_thiscall!(
            WRITE_TASK_VALUE,
            u32,
            output_cursor,
            object,
            value
        );
        lf_checker_rt::global::<u32>(OUTPUT_OFFSET_VA)
            .write_unaligned(output_offset.wrapping_add(OUTPUT_BYTES));
        writer_result
    }
});
