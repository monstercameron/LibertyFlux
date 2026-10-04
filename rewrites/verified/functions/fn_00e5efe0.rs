// original: 0x00e5efe0 register_callback_00e5efe0
/// Register the callback at 0x00E6F280 with the registrar; returns the registrar's answer.
export!(cdecl, rw_00e5efe0() -> u32 {
    unsafe {
        callee_cdecl!(0, u32, relocated(0x00E6F280))
    }
});
