// original: 0x008d8190 NativeImpl_REQUEST_INTERIOR_MODELS
/// Native `REQUEST_INTERIOR_MODELS`: forward one argument with a fixed `this`.
///
/// Passes the module base (a relocated immediate, never the cell contents)
/// as ECX and the caller's argument on the stack; returns the callee answer.
export!(cdecl, rw_008d8190(arg: u32) -> u32 {
    callee_thiscall!(1, u32, relocated(0x0103E8D0), arg)
});
