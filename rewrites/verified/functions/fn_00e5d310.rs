// original: 0x00e5d310 clear_flag_register_dtor_e5d310
/// Clears the low status bit of the module flag byte, then registers a
/// teardown routine with the runtime registrar and returns its answer.
export!(cdecl, rw_e5d310() -> u32 {
    unsafe {
        let flag = global::<u8>(0x018E0105);
        *flag &= 0xFE;
    }
    callee_cdecl!(2, u32, relocated(0x00E6E8E0))
});
