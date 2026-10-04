// original: 0x00e62330 net_init_regfwd_b
/// Run one initialisation routine, then forward a code pointer to the registrar.
///
/// Calls the init routine (stubbed, cdecl/0; it takes no stack arguments and
/// reads no entry registers), discards its answer, then pushes `0x00E70A30` and
/// calls the registrar (stubbed, cdecl/1). Takes no arguments; entry
/// registers are ignored. Returns the registrar answer unchanged.
export!(cdecl, rw_00e62330() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0xE70A30))
    }
});
