// original: 0x008a3b60 aud_forward_to_thiscall
/// Forward `cdecl(a, b)` to `thiscall(b)` on `this = a`.
///
/// Original 0x008A3B60: same shape as rw_008a3b50, different callee.
export!(cdecl, rw_008a3b60(a: u32, b: u32) -> u32 {    callee_thiscall!(1, u32, a, b)
});
