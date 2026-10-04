// original: 0x00dfc2f0 strtol
/// `strtol` front end: forward to the shared worker with the narrow table.
///
/// When the locale flag is set the worker gets a null table pointer and a
/// zero mode word, otherwise the default narrow conversion table.
export!(cdecl, rw_00dfc2f0(text: u32, endptr: u32, base: u32) -> u32 {
    unsafe {
        let table = if *global::<u32>(0x17AC3C4) != 0 {
            0
        } else {
            relocated(0x1058AE0)
        };
        callee_cdecl!(1, u32, table, text, endptr, base, 0)
    }
});
