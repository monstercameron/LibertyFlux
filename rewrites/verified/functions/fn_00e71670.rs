// original: 0x00e71670 audio_thunk_1670
/// Tail-call the shared routine with the constant object at 0x012088b8.
///
/// Loads ECX with the static object address and jumps to the shared
/// routine (callee id 1, thiscall/0 tail call); the rewrite forwards the
/// same pointer through the callee table and returns the callee's answer,
/// matching the value the original leaves in EAX. Takes no arguments
/// (cdecl/0).
export!(cdecl, rw_00e71670() -> u32 {
    unsafe {
        const OBJ: u32 = 0x012088b8;
        lf_checker_rt::callee_thiscall!(1, u32, relocated(OBJ))
    }
});
