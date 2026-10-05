// original: 0x00e6c0d0 veh_obj_init_register_4
/// Initialise one static object and register its callback.
///
/// Calls the object routine (stubbed, thiscall/0) with ECX pointing at
/// the static object at `OBJ` (0x0171C968), then passes the code pointer
/// 0x00E72D00 to the registrar helper (stubbed, cdecl/1, caller pops the
/// argument). Returns the registrar's answer. Takes no arguments.
///
/// Original: 0x00E6C0D0, cdecl, no arguments.
export!(cdecl, rw_00e6c0d0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x171C968;
        const CODEPTR: u32 = 0xE72D00;
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ));
        callee_cdecl!(2, u32, relocated(CODEPTR))
    }
});
