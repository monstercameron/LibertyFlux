// original: 0x008a5f70 audio_fwd_8a5a10
/// Cdecl/3 trampoline into a thiscall/2 audio method.
///
/// Forwards the first argument in ECX and the other two on the stack,
/// returning the callee's answer unchanged.
export!(cdecl, rw_008a5f70(a: u32, b: u32, c: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, a, b, c) }
});

