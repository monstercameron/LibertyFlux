// original: 0x00E67080 task_slots_init_00E67080
/// Initialise the task slots, then forward a code reference.
///
/// Calls callee 1 then callee 2 (both thiscall, no stack
/// arguments) with ECX stepping from `TABLE` by `STRIDE` for
/// `ROWS` rows (5: the original counts edi down from 4 while
/// non-negative), then calls callee 3 (cdecl, one argument: the
/// file address `TAIL`, relocated).
///
/// Original: 0x00E67080 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E67080() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012DD2B0;
        const STRIDE: u32 = 0xA0;
        const ROWS: u32 = 5;
        const TAIL: u32 = 0x00E721A0;
        let mut row = 0u32;
        while row < ROWS {
            let obj = lf_checker_rt::relocated(TABLE.wrapping_add(row.wrapping_mul(STRIDE)));
            lf_checker_rt::callee_thiscall!(1, u32, obj);
            lf_checker_rt::callee_thiscall!(2, u32, obj);
            row += 1;
        }
        lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(TAIL));
        0
    }
});
