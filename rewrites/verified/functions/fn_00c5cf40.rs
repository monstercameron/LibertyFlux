// original: 0x00c5cf40 task_forward_twelve_words (proposed)

/// Forward twelve argument words to the task dispatcher as one buffer.
///
/// Copies the twelve incoming words into a contiguous buffer and invokes
/// callee 1 with the global dispatcher in ecx, the buffer address and the
/// count 0x10, returning its answer.
///
/// Original: 0x00c5cf40 (cdecl, twelve stack words).
lf_checker_rt::export!(cdecl, rw_00c5cf40(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
    a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32,
) -> u32 {
    unsafe {
        const DISPATCHER: u32 = 0x16dce5c;
        const COUNT: u32 = 0x10;
        const SEND: u32 = 1;
        let buf = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11];
        lf_checker_rt::callee_thiscall!(
            SEND,
            u32,
            lf_checker_rt::relocated(DISPATCHER),
            buf.as_ptr() as u32,
            COUNT
        )
    }
});

