// original: 0x008a4540 aud_forward_to_thiscall
/// Forward `cdecl(a, b)` to `thiscall(b)` on `this = a`.
///
/// Original 0x008A4540.
export!(cdecl, rw_008a4540(a: u32, b: u32) -> u32 {    callee_thiscall!(1, u32, a, b)
});
