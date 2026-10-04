// original: 0x00e60830 timing_state_register_indirect
/// Call the state hook through the data table, then register the callback.
///
/// Loads the hook address from the data slot `0x00E731C4` and calls it
/// (stdcall/1) with the state address `0x019F38FC`; the caller performs no
/// cleanup, so the callee pops its argument. Then passes the code pointer
/// `0x00E6FA90` to the registrar (stubbed, cdecl/1) and returns its answer.
export!(cdecl, rw_00e60830() -> u32 {
    unsafe {
        let slot = global::<u32>(0xE731C4);
        let hook: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute::<u32, extern "stdcall" fn(u32) -> u32>(*slot);
        let _: u32 = hook(relocated(0x19F38FC));
        callee_cdecl!(2, u32, relocated(0xE6FA90))
    }
});
