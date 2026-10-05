// original: 0x00e696f0 veh_table_init_e696f0 (proposed)
/// Initialise a fixed table of 32 vehicle records, then emit one record.
///
/// Calls the per-record initialiser (thiscall, object pointer in ECX) for each of 32
/// consecutive records starting at the table base, advancing by the record stride,
/// then forwards a constant record descriptor to the shared record routine (cdecl)
/// and returns its answer. Takes no arguments (cdecl, no stack words). The loop
/// counter runs down from 0x1f and the body also runs for zero, hence 32+1 calls.
///
/// Original: 0x00e696f0 (cdecl, no arguments; 32 thiscalls + 1 cdecl call).
lf_checker_rt::export!(cdecl, rw_00e696f0() -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x01666A70;
        const COUNT_DOWN_FROM: u32 = 0x1F;
        const STRIDE: u32 = 0x10;
        const RECORD: u32 = 0x00E72680;
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
