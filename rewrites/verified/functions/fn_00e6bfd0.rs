// original: 0x00e6bfd0 veh_callback_register_4
/// Register one callback with the registrar helper.
///
/// Pushes the code pointer 0x00E72CC0 and calls the registrar (stubbed,
/// cdecl/1, caller pops the argument). Returns the registrar's answer.
/// Takes no arguments.
///
/// Original: 0x00E6BFD0, cdecl, no arguments.
export!(cdecl, rw_00e6bfd0() -> u32 {
    unsafe {
        const CODEPTR: u32 = 0xE72CC0;
        callee_cdecl!(1, u32, relocated(CODEPTR))
    }
});
