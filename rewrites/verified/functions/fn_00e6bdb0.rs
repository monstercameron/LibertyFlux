// original: 0x00e6bdb0 veh_callback_register_2
/// Register one callback with the registrar helper.
///
/// Pushes the code pointer 0x00E72C30 and calls the registrar (stubbed,
/// cdecl/1, caller pops the argument). Returns the registrar's answer.
/// Takes no arguments.
///
/// Original: 0x00E6BDB0, cdecl, no arguments.
export!(cdecl, rw_00e6bdb0() -> u32 {
    unsafe {
        const CODEPTR: u32 = 0xE72C30;
        callee_cdecl!(1, u32, relocated(CODEPTR))
    }
});
