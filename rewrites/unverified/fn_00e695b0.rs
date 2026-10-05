// original: 0x00e695b0 veh_table_init_e695b0 (proposed)
/// Initialise a fixed table of 40 vehicle records, then emit one record.
///
/// Calls the per-record initialiser (thiscall, object pointer in ECX) for each of 40
/// consecutive records starting at the table base, advancing by the record stride,
/// then forwards a constant record descriptor to the shared record routine (cdecl)
/// and returns its answer. Takes no arguments (cdecl, no stack words). The loop
/// counter runs down from 0x27 and the body also runs for zero, hence 40+1 calls.
///
/// Original: 0x00e695b0 (cdecl, no arguments; 40 thiscalls + 1 cdecl call).
lf_checker_rt::export!(cdecl, rw_00e695b0() -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x01660364;
        const COUNT_DOWN_FROM: u32 = 0x27;
        const STRIDE: u32 = 0x6C;
        const RECORD: u32 = 0x00E725C0;
        let mut obj = lf_checker_rt::relocated(TABLE_BASE);
        let mut left = COUNT_DOWN_FROM;
        loop {
            lf_checker_rt::callee_thiscall!(1, u32, obj);
            obj = obj.wrapping_add(STRIDE);
            left = left.wrapping_sub(1);
            if (left as i32) < 0 {
                break;
            }
        }
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(RECORD))
    }
});
