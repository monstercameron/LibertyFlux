// original: 0x00BE7DC0 reset_task_storage_slots

/// Reset 128 task-storage flags and call the per-slot initializer for the
/// corresponding 108-byte records. The record array begins at file VA
/// `0x0167F780`; the byte flags begin at `0x0167F5A8`. Both addresses are
/// relocated through the checker runtime. Each initializer receives its
/// record pointer in ECX and returns a dword; the last return value is the
/// function's EAX result. The original is cdecl with no stack arguments.
lf_checker_rt::export!(cdecl, rw_00BE7DC0() -> u32 {
    const FIRST_RECORD_VA: u32 = 0x0167F780;
    const FLAG_ARRAY_VA: u32 = 0x0167F5A8;
    const RECORD_STRIDE: u32 = 0x6C;
    const FLAG_COUNT: u32 = 0x80;
    const INITIALIZER_ID: u32 = 1;

    let mut last_result = 0;
    for index in 0..FLAG_COUNT {
        unsafe { lf_checker_rt::global::<u8>(FLAG_ARRAY_VA + index).write(0) };
        let record = lf_checker_rt::relocated(FIRST_RECORD_VA + index * RECORD_STRIDE);
        last_result = lf_checker_rt::callee_thiscall!(INITIALIZER_ID, u32, record);
    }
    last_result
});
