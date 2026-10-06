// original: 0x00e71f50 audio_reinit_1f50
/// Run the helper on 0x01289250, reinstall the vtable, tail-call init.
///
/// Calls the helper (id 1, thiscall/0) with 0x01289250 in ECX and
/// discards its answer, stores the vtable 0x00e83134 at 0x01289230 (a
/// relocated absolute, as in fn 0x00e71710), then tail-calls the init
/// routine (id 2, thiscall/0) with 0x01289230 in ECX and returns its
/// answer. Takes no arguments (cdecl/0).
export!(cdecl, rw_00e71f50() -> u32 {
    unsafe {
        const HELPER_OBJ: u32 = 0x01289250;
        const OBJ: u32 = 0x01289230;
        const VTABLE: u32 = 0x00E83134;
        lf_checker_rt::callee_thiscall!(1, u32, relocated(HELPER_OBJ));
        lf_checker_rt::global::<u32>(OBJ).write_unaligned(relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(2, u32, relocated(OBJ))
    }
});
