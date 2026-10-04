// original: 0x00dfc31a strtoul
/// `strtoul` front end: same shape as `strtol` with mode word 1.
export!(cdecl, rw_00dfc31a(text: u32, endptr: u32, base: u32) -> u32 {
    unsafe {
        let table = if *global::<u32>(0x17AC3C4) != 0 {
            0
        } else {
            relocated(0x1058AE0)
        };
        callee_cdecl!(1, u32, table, text, endptr, base, 1)
    }
});
