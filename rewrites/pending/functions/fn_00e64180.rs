// original: 0x00e64180 audio_entity_init_7
/// Initialise one static audio object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at the
/// static object at `0x0121F9D4`, then passes the code pointer `0x00E716E0` to the
/// registrar helper (stubbed, cdecl/1). Returns the registrar's answer.
export!(cdecl, rw_00e64180() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x121F9D4));
        callee_cdecl!(2, u32, relocated(0xE716E0))
    }
});
