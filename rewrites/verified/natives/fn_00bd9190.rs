// original: 0x00BD9190 SET_DISPLAY_PLAYER_NAME_AND_ICON
//
// Forwards the player index plus a boolean-ised flag with the same
// incoming-slot graffiti as rw_00b9e340 (measured offset, checked each trial).
export!(cdecl, rw_00bd9190(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args.add(1) != 0) as u8;
        let coerced = flag as u32;
        callee_cdecl!(1, u32, *args, coerced)
    }
});

