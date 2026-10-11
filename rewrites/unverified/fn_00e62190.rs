// original: 0x00e62190 reset_network_pool_records

/// For five records spaced by 0x108 bytes, call the zero-fill helper on the preceding 0x100-byte region, then set the record's first word to all ones and its second word to zero. Return the final helper answer. The cdecl entry has no arguments. Each zero-fill call is cdecl with destination, fill value, and byte count; the final helper answer remains in EAX.
lf_checker_rt::export!(cdecl, rw_00e62190() -> u32 {
    const FIRST_RECORD: u32 = 0x01b4a458;
    const ZERO_WINDOW_BYTES: u32 = 0x100;
    const RECORD_STEP: u32 = 0x108;
    let mut record_va = FIRST_RECORD;
    let mut last_fill_result = 0u32;
    for _record_index in 0..5 {
        let zero_window_va = record_va.wrapping_sub(ZERO_WINDOW_BYTES);
        last_fill_result = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(zero_window_va), 0, ZERO_WINDOW_BYTES);
        unsafe {
            lf_checker_rt::global::<u32>(record_va).write_volatile(u32::MAX);
            lf_checker_rt::global::<u32>(record_va.wrapping_add(4)).write_volatile(0);
        }
        record_va = record_va.wrapping_add(RECORD_STEP);
    }
    last_fill_result
});
