// original: 0x008a4280 aud_forward_to_thiscall
/// Forward `cdecl(a, b)` to `thiscall(b)` on `this = a`.
///
/// Original 0x008A4280.
export!(cdecl, rw_008a4280(a: u32, b: u32) -> u32 {    callee_thiscall!(1, u32, a, b)
});
