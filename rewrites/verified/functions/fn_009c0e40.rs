// original: 0x009C0E40 ped_task_release_active_records
/// Scans the 32 task records at a 0xd0-byte stride and calls the release
/// helper with each record address whose state dword at offset 0x10 is
/// nonzero. The helper call is intercepted and compared by the checker; the
/// function has no semantic return value.
lf_checker_rt::export!(cdecl, rw_009c0e40() -> () {
    const TABLE_BASE: u32 = 0x0129_2450;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;

    for record_index in 0..RECORD_COUNT {
        let record = TABLE_BASE.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
        let state = unsafe { lf_checker_rt::global::<u32>(record + 0x10).read() };
        if state != 0 {
            let _ = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(record));
        }
    }
});
