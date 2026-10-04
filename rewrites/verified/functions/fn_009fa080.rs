// original: 0x009fa080 parse_stage_34
/// Parse stage: run the base stage, then fetch field 0x20 into the output
/// slot at +0x34. Any failed stage clears the slot and returns zero.
export!(cdecl, rw_009fa080(a0: u32, a1: u32) -> u32 {
    unsafe {
        if callee_cdecl!(1, u32, a0, a1) as u8 == 0 {
            return 0;
        }
        if callee_thiscall!(2, u32, a0, 0x20) as u8 == 0 {
            ((a1 as *mut u32).byte_add(0x34)).write(0);
            return 0;
        }
        let first = (a0 as *const u32).read();
        let sum = (a0 as *const u32).byte_add(0xc).read()
            .wrapping_add((a0 as *const u32).byte_add(4).read());
        let mut out: u32 = 0;
        callee_cdecl!(3, u32, first, &mut out as *mut u32 as u32, 0x20, sum);
        ((a1 as *mut u32).byte_add(0x34)).write(out);
        let ans = callee_thiscall!(4, u32, a0, 0x20);
        (ans & 0xFFFF_FF00) | 1
    }
});
