// original: 0x00bb7620 SET_REDUCE_VEHICLE_MODEL_BUDGET
//
// Forwards a single boolean-ised flag with the incoming-slot graffiti
// (measured offset, checked each trial).
export!(cdecl, rw_00bb7620(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args != 0) as u8;
        let probe = 0u32;
        let slot = (core::ptr::addr_of!(probe) as u32).wrapping_add(RW_00BB7620_K);
        *(slot as *mut u8) = flag;
        let coerced = (ctx as u32 & 0xFFFF_FF00) | flag as u32;
        callee_cdecl!(1, u32, coerced)
    }
});

/// Measured bytes from `probe` to the incoming context slot in rw_00bb7620.
const RW_00BB7620_K: u32 = 0x48;
