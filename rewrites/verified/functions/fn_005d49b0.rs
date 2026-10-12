// original: 0x005d49b0 CViewportGame::vf1
//
// Run the base viewport setup, set the game viewport's ready flag, and
// reset its pending interval. The base call's result remains in EAX.
lf_checker_rt::export!(thiscall, rw_005d49b0(this: u32) -> u32 {
    unsafe {
        const BASE_SETUP: u32 = 1;
        let result: u32 = lf_checker_rt::callee_thiscall!(BASE_SETUP, u32, this);
        let flags = (this.wrapping_add(0x558) as *const u8).read();
        (this.wrapping_add(0x558) as *mut u8).write(flags | 1);
        (this.wrapping_add(0x544) as *mut u32).write_unaligned(0x2710);
        result
    }
});
