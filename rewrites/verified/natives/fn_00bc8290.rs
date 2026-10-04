// original: 0x00BC8290 SWITCH_RANDOM_TRAINS
//
// Forwards a single boolean-ised flag with the incoming-slot graffiti
// (measured offset, checked each trial).
export!(cdecl, rw_00bc8290(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args != 0) as u8;
        let coerced = flag as u32;
        callee_cdecl!(1, u32, coerced)
    }
});

