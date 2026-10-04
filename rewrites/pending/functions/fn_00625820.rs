// original: 0x00625820 timing_singleton_get
// Return the timing manager singleton, initialising it once.
//
// When the initialised flag is clear, sets it, zeroes the six state words,
// runs the construct and register steps, then returns the block address.
// Returns the block address either way.
export!(cdecl, rw_00625820() -> u32 {
    unsafe {
        const BLOCK: u32 = 0x01bb6680;
        const FLAG: u32 = 0x01bb6698;
        const REGISTER_ARG: u32 = 0x00e6f230;
        const WORDS: u32 = 6;
        let flag = global::<u32>(FLAG);
        if *flag & 1 == 0 {
            *flag |= 1;
            let base = relocated(BLOCK);
            let mut i: u32 = 0;
            while i < WORDS {
                *((base.wrapping_add(i.wrapping_mul(4))) as *mut u32) = 0;
                i += 1;
            }
            callee_thiscall!(1, u32, base);
            callee_cdecl!(2, u32, relocated(REGISTER_ARG));
        }
        relocated(BLOCK)
    }
});
