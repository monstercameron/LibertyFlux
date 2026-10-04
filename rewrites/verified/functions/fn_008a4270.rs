// original: 0x008a4270 aud_forward_to_thiscall
/// Forward `cdecl(a, b)` to `thiscall(b)` on `this = a`.
///
/// Original 0x008A4270.
export!(cdecl, rw_008a4270(a: u32, b: u32) -> u32 {    callee_thiscall!(1, u32, a, b)
});
