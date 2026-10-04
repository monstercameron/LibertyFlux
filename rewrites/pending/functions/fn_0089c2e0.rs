// original: 0x0089c2e0 aud_thunk_to_0089bb20
/// Adapter thunk: forwards `(object, arg)` to a thiscall audio method.
///
/// The object pointer arrives on the stack (cdecl shape) and is moved into
/// ECX for the callee. Returns the callee's answer unchanged.
export!(cdecl, rw_0089c2e0(obj: u32, arg: u32) -> u32 {
    callee_thiscall!(1, u32, obj, arg)
});

