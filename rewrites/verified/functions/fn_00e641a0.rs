// original: 0x00e641a0 audio_entity_init_8
/// Initialise one static audio object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at the
/// static object at `0x0121FA60`, then passes the code pointer `0x00E716F0` to the
/// registrar helper (stubbed, cdecl/1). Returns the registrar's answer.
export!(cdecl, rw_00e641a0() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x121FA60));
        callee_cdecl!(2, u32, relocated(0xE716F0))
    }
});
