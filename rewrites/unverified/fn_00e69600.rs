// original: 0x00e69600 veh_table_init_e69600 (proposed)
/// Initialise a fixed table of 31 vehicle records in place.
///
/// Calls the per-record initialiser (thiscall, object pointer in ECX, no stack words)
/// for each of 31 consecutive records starting at the table base, advancing by the
/// record stride each time, and returns the last calls answer. Takes no arguments
/// (cdecl, no stack words). The loop counter runs down from 0x1e and the body also
/// runs for zero, hence 31 calls.
///
/// Original: 0x00e69600 (cdecl, no arguments; 31 thiscalls).
lf_checker_rt::export!(cdecl, rw_00e69600() -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x01661410;
        const COUNT_DOWN_FROM: u32 = 0x1E;
        const STRIDE: u32 = 0x34;
        let mut obj = lf_checker_rt::relocated(TABLE_BASE);
        let mut left = COUNT_DOWN_FROM;
        let mut answer = 0u32;
        loop {
            answer = lf_checker_rt::callee_thiscall!(1, u32, obj);
            obj = obj.wrapping_add(STRIDE);
            left = left.wrapping_sub(1);
            if (left as i32) < 0 {
                break;
            }
        }
        answer
    }
});
