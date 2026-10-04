// original: 0x00e5efc0 init_then_register_00e5efc0
/// Run the zero-argument initializer at 0x0062EA90, then register the callback at 0x00E6F270 with the registrar; returns the registrar's answer.
export!(cdecl, rw_00e5efc0() -> u32 {
    unsafe {
        callee_cdecl!(0, u32,);
        callee_cdecl!(1, u32, relocated(0x00E6F270))
    }
});
