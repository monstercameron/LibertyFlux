// original: 0x0095C7B0 append_task_record_header
/// Append a fixed-size task record header through the shared output cursor.
///
/// This no-argument cdecl wrapper reads the destination base and byte offset,
/// passes their wrapping sum in ECX with a payload pointer on the stack to a
/// thiscall writer, advances the cursor by 0x50 bytes, and returns the writer's
/// EAX value unchanged. The writer owns any output-buffer mutation.
lf_checker_rt::export!(cdecl, rw_0095c7b0() -> u32 {
    const BUFFER_OFFSET_VA: u32 = 0x0120F290;
    const BUFFER_BASE_VA: u32 = 0x0120F294;
    const HEADER_POINTER_VA: u32 = 0x0103E49C;
    const HEADER_WRITER: u32 = 1;
    const HEADER_BYTES: u32 = 0x50;

    unsafe {
        let buffer_offset = lf_checker_rt::global::<u32>(BUFFER_OFFSET_VA).read_unaligned();
        let buffer_base = lf_checker_rt::global::<u32>(BUFFER_BASE_VA).read_unaligned();
        let header_pointer = lf_checker_rt::global::<u32>(HEADER_POINTER_VA).read_unaligned();
        let destination = buffer_base.wrapping_add(buffer_offset);
        let helper_result = lf_checker_rt::callee_thiscall!(
            HEADER_WRITER,
            u32,
            destination,
            header_pointer
        );
        lf_checker_rt::global::<u32>(BUFFER_OFFSET_VA)
            .write_unaligned(buffer_offset.wrapping_add(HEADER_BYTES));
        helper_result
    }
});
