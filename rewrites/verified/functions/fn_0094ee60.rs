// original: 0x0094EE60 build_struct_then_run (proposed)

/// Build a 3-word struct from two floats and run two callees over it.
///
/// Forms the words (`a`, `b`, 0) on the stack and passes a pointer to them
/// to callee 1 (one stack word; the pointer's value differs per side so the
/// contract skips it and snapshots the 3 words instead). Callee 1's answer
/// becomes the object pointer for callee 2 (thiscall, no stack words),
/// whose answer is returned. The floats are only moved, never computed.
///
/// Original: 0x0094EE60 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_0094EE60(a: u32, b: u32) -> u32 {
    unsafe {
        const BUILD: u32 = 1;
        const RUN: u32 = 2;
        let st = [a, b, 0u32];
        let h = lf_checker_rt::callee_cdecl!(BUILD, u32, st.as_ptr() as u32);
        lf_checker_rt::callee_thiscall!(RUN, u32, h)
    }
});
