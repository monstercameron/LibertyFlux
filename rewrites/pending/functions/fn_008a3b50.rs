// original: 0x008a3b50 aud_forward_to_thiscall
/// Forward `cdecl(a, b)` to `thiscall(b)` on `this = a`.
///
/// Original 0x008A3B50: pushes the second word, moves the first into ECX,
/// tail-shape single call, returns the callee's answer.
export!(cdecl, rw_008a3b50(a: u32, b: u32) -> u32 {    callee_thiscall!(1, u32, a, b)
});
