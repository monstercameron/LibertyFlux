// original: 0x008a4550 aud_forward_to_thiscall
/// Forward `cdecl(a, b)` to `thiscall(b)` on `this = a`.
///
/// Original 0x008A4550.
export!(cdecl, rw_008a4550(a: u32, b: u32) -> u32 {    callee_thiscall!(1, u32, a, b)
});
