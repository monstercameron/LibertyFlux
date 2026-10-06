// original: 0x00e718c0 audio_reinit_18c0
/// Reinstall the vtable at 0x01238898 and tail-call the init routine.
///
/// Stores the vtable address 0x00e83134 into the object header (a
/// relocated absolute, as in fn 0x00e71710), then tail-calls the init
/// routine (id 1, thiscall/0) with the object in ECX and returns its
/// answer. Takes no arguments (cdecl/0).
export!(cdecl, rw_00e718c0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x01238898;
        const VTABLE: u32 = 0x00E83134;
        lf_checker_rt::global::<u32>(OBJ).write_unaligned(relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, relocated(OBJ))
    }
});
