// original: 0x00bb9480 TASK_CLIMB_LADDER
//
// Forwards the character handle plus a boolean-ised flag with the
// incoming-slot graffiti (measured offset, checked each trial).
export!(cdecl, rw_00bb9480(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args.add(1) != 0) as u8;
        let probe = 0u32;
        let slot = (core::ptr::addr_of!(probe) as u32).wrapping_add(RW_00BB9480_K);
        *(slot as *mut u8) = flag;
        let coerced = (ctx as u32 & 0xFFFF_FF00) | flag as u32;
        callee_cdecl!(1, u32, *args, coerced)
    }
});

/// Measured bytes from `probe` to the incoming context slot in rw_00bb9480.
const RW_00BB9480_K: u32 = 0x54;
