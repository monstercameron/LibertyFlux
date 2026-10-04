// original: 0x00BA28F0 STOP_PED_WEAPON_FIRING_WHEN_DROPPED
//
// Forwards the character handle to the engine routine. No return value.
export!(cdecl, rw_00ba28f0(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(1, u32, *args)
    }
});
