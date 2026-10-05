// original: 0x00887CF0 stream_forward_1 (proposed)

/// Forward one argument to the stream manager entry.
///
/// Pushes the argument and calls the manager entry (callee 1) with the
/// manager object constant in `ecx`; the callee pops the word. Its answer
/// is the answer of this function.
///
/// Original: 0x00887CF0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00887CF0(arg: u32) -> u32 {
    unsafe {
        const MANAGER_FILE_VA: u32 = 0x0115_dc18;
        const ENTRY: u32 = 1;
        let manager = lf_checker_rt::relocated(MANAGER_FILE_VA);
        lf_checker_rt::callee_thiscall!(ENTRY, u32, manager, arg)
    }
});
