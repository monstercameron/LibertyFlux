// original: 0x00B30320 clear_ped_task_records
/// Call the reset helper for all 32 task records in table order, then clear
/// the one-byte task table status. The helper is thiscall with each record as
/// its receiver; this cdecl wrapper has no arguments.
const TASK_RECORD_TABLE_VA: u32 = 0x0166_1a60;
const RESET_RECORD_STRIDE: u32 = 0x50;
const RESET_RECORD_COUNT: u32 = 32;
const TASK_RECORD_STATUS_VA: u32 = 0x0166_1401;

lf_checker_rt::export!(cdecl, rw_b30320() -> u32 {
    unsafe {
        let table = lf_checker_rt::relocated(TASK_RECORD_TABLE_VA);
        for index in 0..RESET_RECORD_COUNT {
            let record = table.wrapping_add(index * RESET_RECORD_STRIDE);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, record);
        }
        let status = lf_checker_rt::relocated(TASK_RECORD_STATUS_VA);
        (status as *mut u8).write_unaligned(0);
        0
    }
});
