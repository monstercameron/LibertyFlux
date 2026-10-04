// original: 0x00e64100 audio_entity_init_3
/// Initialise one static audio object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at the
/// static object at `0x0121FAD8`, then passes the code pointer `0x00E716A0` to the
/// registrar helper (stubbed, cdecl/1). Returns the registrar's answer.
export!(cdecl, rw_00e64100() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x121FAD8));
        callee_cdecl!(2, u32, relocated(0xE716A0))
    }
});
