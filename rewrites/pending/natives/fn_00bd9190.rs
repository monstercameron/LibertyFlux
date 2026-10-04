// original: 0x00bd9190 SET_DISPLAY_PLAYER_NAME_AND_ICON
//
// Forwards the player index plus a boolean-ised flag with the same
// incoming-slot graffiti as rw_00b9e340 (measured offset, checked each trial).
export!(cdecl, rw_00bd9190(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args.add(1) != 0) as u8;
        let probe = 0u32;
        let slot = (core::ptr::addr_of!(probe) as u32).wrapping_add(RW_00BD9190_K);
        *(slot as *mut u8) = flag;
        let coerced = (ctx as u32 & 0xFFFF_FF00) | flag as u32;
        callee_cdecl!(1, u32, *args, coerced)
    }
});

/// Measured bytes from `probe` to the incoming context slot in rw_00bd9190.
const RW_00BD9190_K: u32 = 0x54;
