// original: 0x0089b610 aud_thunk_to_0089a920
/// Adapter thunk: forwards `(object, arg)` to a thiscall audio method.
///
/// The object pointer arrives on the stack (cdecl shape) and is moved into
/// ECX for the callee. Returns the callee's answer unchanged.
export!(cdecl, rw_0089b610(obj: u32, arg: u32) -> u32 {
    callee_thiscall!(1, u32, obj, arg)
});

