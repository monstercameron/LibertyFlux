// original: 0x00e6c020 veh_obj_init_register_3
/// Initialise one static object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at
/// the static object at `OBJ` (0x0171BFB0), then passes the code pointer
/// 0x00E72CD0 to the registrar helper (stubbed, cdecl/1, caller pops the
/// argument). Returns the registrar's answer. Takes no arguments.
///
/// Original: 0x00E6C020, cdecl, no arguments.
export!(cdecl, rw_00e6c020() -> u32 {
    unsafe {
        const OBJ: u32 = 0x171BFB0;
        const CODEPTR: u32 = 0xE72CD0;
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ));
        callee_cdecl!(2, u32, relocated(CODEPTR))
    }
});
