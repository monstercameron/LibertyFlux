// original: 0x00e62880 audio_thunk_forward
/// Forward to the shared audio routine with this module's singleton object.
///
/// The original loads a fixed object address into ECX and tail-jumps; the
/// rewrite expresses the same transfer as a call that forwards the object
/// pointer and returns the callee's answer.
export!(cdecl, rw_00e62880() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0115_FCD8;
        callee_thiscall!(1, u32, relocated(OBJECT))
    }
});
