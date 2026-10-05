// original: 0x008CA8A0 stream_load_lists (proposed)

/// Loads the streaming lists for `mode`: mode 1 calls the list callee
/// (callee 0) three times with (flag, count, table) triples
/// (0, 12, `TABLE_A`), (1, 5, `TABLE_B`) and (1, 37, `TABLE_C`); mode 2
/// calls it once with (0, 30, `TABLE_D`); any other mode makes no calls.
///
/// One stack argument (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CA8A0(mode: u32) -> u32 {
    unsafe {
        /// List callee id.
        const LOAD: u32 = 0;
        /// Tables passed to the list callee.
        const TABLE_A: u32 = 0x1031E68;
        const TABLE_B: u32 = 0x1031E18;
        const TABLE_C: u32 = 0x1031BC8;
        const TABLE_D: u32 = 0x1031F30;
        if mode == 1 {
            let _a: u32 =
                lf_checker_rt::callee_cdecl!(LOAD, u32, 0, 12, lf_checker_rt::relocated(TABLE_A));
            let _b: u32 =
                lf_checker_rt::callee_cdecl!(LOAD, u32, 1, 5, lf_checker_rt::relocated(TABLE_B));
            let _c: u32 =
                lf_checker_rt::callee_cdecl!(LOAD, u32, 1, 37, lf_checker_rt::relocated(TABLE_C));
        } else if mode == 2 {
            let _d: u32 =
                lf_checker_rt::callee_cdecl!(LOAD, u32, 0, 30, lf_checker_rt::relocated(TABLE_D));
        }
        0
    }
});
