// original: 0x00e69590 veh_table_init_e69590 (proposed)
/// Initialise a fixed table of 20 vehicle records in place.
///
/// Calls the per-record initialiser (thiscall, object pointer in ECX, no stack words)
/// for each of 20 consecutive records starting at the table base, advancing by the
/// record stride each time, and returns the last calls answer. Takes no arguments
/// (cdecl, no stack words). The loop counter runs down from 0x13 and the body also
/// runs for zero, hence 20 calls.
///
/// Original: 0x00e69590 (cdecl, no arguments; 20 thiscalls).
lf_checker_rt::export!(cdecl, rw_00e69590() -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x0165FD60;
        const COUNT_DOWN_FROM: u32 = 0x13;
        const STRIDE: u32 = 0x48;
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
