// original: 0x00e71e70 audio_reinit_1e70
/// Reinstall two vtable pointers and tail-call the init routine.
///
/// Stores 0x00e8e23c at 0x012845d8 and the vtable 0x00e83134 at
/// 0x012845d0 (both relocated absolutes, as in fn 0x00e71710), then
/// tail-calls the init routine (id 1, thiscall/0) with 0x012845d0 in ECX
/// and returns its answer. Takes no arguments (cdecl/0).
export!(cdecl, rw_00e71e70() -> u32 {
    unsafe {
        const OBJ: u32 = 0x012845D0;
        const AUX: u32 = 0x012845D8;
        const VTABLE: u32 = 0x00E83134;
        const AUX_PTR: u32 = 0x00E8E23C;
        lf_checker_rt::global::<u32>(AUX).write_unaligned(relocated(AUX_PTR));
        lf_checker_rt::global::<u32>(OBJ).write_unaligned(relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, relocated(OBJ))
    }
});
