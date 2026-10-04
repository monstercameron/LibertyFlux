// original: 0x00E5CD00 timer_register_callback_680
/// Register one mainloop-timing callback and return the registrar's answer.
///
/// Pushes the callback's code address and calls the registrar with it
/// (cdecl/1, callee id 2 in the proof contract). Takes no arguments;
/// entry registers are ignored and no memory is touched besides call scratch.
export!(cdecl, rw_00e5cd00() -> u32 {
    unsafe { callee_cdecl!(2, u32, relocated(0x00E6E680)) }
});
