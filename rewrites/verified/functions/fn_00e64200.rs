// original: 0x00e64200 audio_collision_entity_init
/// Initialise one static audio object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at the
/// static object at `0x012202E0`, then passes the code pointer `0x00E71710` to the
/// registrar helper (stubbed, cdecl/1). Returns the registrar's answer.
export!(cdecl, rw_00e64200() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x12202E0));
        callee_cdecl!(2, u32, relocated(0xE71710))
    }
});
