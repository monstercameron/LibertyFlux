// original: 0x00e616a0 timing_lock_init_and_register
/// Initialise the timing lock, then register the callback.
///
/// Calls the system's critical-section initializer through the game's own
/// import slot, then returns the registrar's answer.
export!(cdecl, rw_00e616a0() -> u32 {
    unsafe {
        let slot = *global::<u32>(0x00E731C4);
        let init: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        init(relocated(0x01A01458));
        callee_cdecl!(2, u32, relocated(0x00E70230))
    }
});
