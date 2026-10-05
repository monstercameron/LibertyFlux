// original: 0x00e692e0 veh_array_init_records_e692e0 (proposed)
/// Initialise a fixed array of 24 vehicle records, clear two globals, emit one record.
///
/// Calls the per-record initialiser (stdcall, 4 stack words: a constant table pointer
/// then 8, 8 and the record address) for each of 24 consecutive records starting at
/// the array base, advancing by 0x54 each time; writes zero to two adjacent globals;
/// then forwards a constant record descriptor to the shared record routine (cdecl)
/// and returns its answer. Takes no arguments (cdecl, no stack words). The loop
/// counter runs down from 0x17 and the body also runs for zero, hence 24 calls.
///
/// Original: 0x00e692e0 (cdecl, no arguments; 24 stdcalls + 1 cdecl call).
lf_checker_rt::export!(cdecl, rw_00e692e0() -> u32 {
    unsafe {
        const ARRAY_BASE: u32 = 0x01614CA0;
        const COUNT_DOWN_FROM: u32 = 0x17;
        const STRIDE: u32 = 0x54;
        const FIXED_TABLE: u32 = 0x00451C10;
        const FLAG_A: u32 = 0x01615470;
        const FLAG_B: u32 = 0x01615474;
        const RECORD: u32 = 0x00E72560;
        let mut obj = lf_checker_rt::relocated(ARRAY_BASE);
        let mut left = COUNT_DOWN_FROM;
        loop {
            lf_checker_rt::callee_stdcall!(1, u32, obj, 8, 8, lf_checker_rt::relocated(FIXED_TABLE));
            obj = obj.wrapping_add(STRIDE);
            left = left.wrapping_sub(1);
            if (left as i32) < 0 {
                break;
            }
        }
        core::ptr::write_unaligned(lf_checker_rt::global::<u32>(FLAG_A), 0);
        core::ptr::write_unaligned(lf_checker_rt::global::<u32>(FLAG_B), 0);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(RECORD))
    }
});

