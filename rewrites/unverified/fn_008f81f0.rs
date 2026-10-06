// original: 0x008f81f0 input_forward_6f (proposed)

/// Forward a handle plus six float words to the six-float sink callee.
///
/// The seven stack words are a handle `p1` and six float bit patterns. The
/// function copies the six floats to fresh stack slots and tail-calls the
/// sink (thiscall: ECX = `p1`, six stack words); the sink's answer is the
/// return value. Entry registers are ignored. The floats move with plain
/// word copies, so NaN payloads survive bit-exactly and no arithmetic runs.
///
/// Stdcall: seven stack words, callee cleans 0x1C.
lf_checker_rt::export!(stdcall, rw_008f81f0(p1: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, f6: u32) -> u32 {
    const C_SINK: u32 = 1;
    lf_checker_rt::callee_thiscall!(C_SINK, u32, p1, f1, f2, f3, f4, f5, f6)
});
