// original: 0x008a5fa0 audio_fwd_8a5c60
/// Cdecl/2 trampoline into a thiscall/1 audio method.
///
/// Forwards the first argument in ECX and the second on the stack,
/// returning the callee's answer unchanged.
export!(cdecl, rw_008a5fa0(a: u32, b: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, a, b) }
});

