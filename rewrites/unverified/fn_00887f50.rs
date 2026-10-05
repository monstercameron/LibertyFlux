// original: 0x00887F50 stream_forward_word (proposed)

/// Forward the low word of one argument to the stream manager entry.
///
/// Zero-extends the low word of the argument and calls the manager entry
/// (callee 1) with the manager object constant in `ecx`; the callee pops
/// the word. Its answer is the answer of this function.
///
/// Original: 0x00887F50 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00887F50(arg: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0115_dc18;
        const ENTRY: u32 = 1;
        lf_checker_rt::callee_thiscall!(ENTRY, u32, MANAGER, arg & 0xffff)
    }
});
