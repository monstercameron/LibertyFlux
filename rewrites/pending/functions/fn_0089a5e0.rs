// original: 0x0089a5e0 aud_thunk_to_0089a070
/// Adapter thunk: forwards `(object, a, b)` to a thiscall audio method.
///
/// Three-argument variant of the stack-to-ECX adapter. Returns the callee's
/// answer unchanged.
export!(cdecl, rw_0089a5e0(obj: u32, a: u32, b: u32) -> u32 {
    callee_thiscall!(1, u32, obj, a, b)
});

