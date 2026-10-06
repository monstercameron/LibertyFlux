// original: 0x008f8250 input_forward_4f (proposed)

/// Forward a handle plus four float words to the four-float sink callee.
///
/// The five stack words are a handle `p1` and four float bit patterns. The
/// function copies the four floats to fresh stack slots and tail-calls the
/// sink (thiscall: ECX = `p1`, four stack words); the sink's answer is the
/// return value. Entry registers are ignored. The floats move with plain
/// word copies, so NaN payloads survive bit-exactly and no arithmetic runs.
///
/// Stdcall: five stack words, callee cleans 0x14.
lf_checker_rt::export!(stdcall, rw_008f8250(p1: u32, f1: u32, f2: u32, f3: u32, f4: u32) -> u32 {
    const C_SINK: u32 = 1;
    lf_checker_rt::callee_thiscall!(C_SINK, u32, p1, f1, f2, f3, f4)
});
