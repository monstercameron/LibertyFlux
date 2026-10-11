// original: 0x009C0E70 ped_task_release_nonqueued_records
/// Scans 32 task records at a 0xd0-byte stride. It calls the release helper
/// with a record address when the state dword at offset 0x10 is nonzero and
/// the key dword at offset 0x18 is not 4. The function has no semantic
/// return value.
lf_checker_rt::export!(cdecl, rw_009c0e70() -> () {
    const TABLE_BASE: u32 = 0x0129_2450;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;
    const EXEMPT_KEY: u32 = 4;

    for record_index in 0..RECORD_COUNT {
        let record = TABLE_BASE.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
        let state = unsafe { lf_checker_rt::global::<u32>(record + 0x10).read() };
        let key = unsafe { lf_checker_rt::global::<u32>(record + 0x18).read() };
        if state != 0 && key != EXEMPT_KEY {
            let _ = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(record));
        }
    }
});
