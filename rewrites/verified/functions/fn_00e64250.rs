// original: 0x00e64250 audio_entity_init_11
/// Initialise one static audio object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at the
/// static object at `0x0123144C`, then passes the code pointer `0x00E71770` to the
/// registrar helper (stubbed, cdecl/1). Returns the registrar's answer.
export!(cdecl, rw_00e64250() -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x123144C));
        callee_cdecl!(2, u32, relocated(0xE71770))
    }
});
