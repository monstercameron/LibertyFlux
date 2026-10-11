// original: 0x00C864B0 fn_00c864b0_state_reset

/// Reset forty fixed-stride task records and the owning object's small state
/// fields. The object pointer is in ECX; the original is thiscall and returns
/// the same pointer. Each record receives one zeroed dword and one zeroed
/// halfword; untouched padding remains untouched.
lf_checker_rt::export!(thiscall, rw_00c864b0_state_reset(object: u32) -> u32 {
    unsafe {
        const TASK_RECORDS: u32 = 40;
        const TASK_RECORD_STRIDE: u32 = 0x60;
        const TASK_RECORDS_OFFSET: u32 = 0x78;
        const OBJECT_FLAGS_OFFSET: u32 = 0xF6C;
        const OBJECT_SENTINEL_OFFSET: u32 = 0xF70;
        const OBJECT_ZERO_BLOCK_0: u32 = 0xF50;
        const OBJECT_ZERO_BLOCK_1: u32 = 0xF58;
        const OBJECT_COUNTER_OFFSET: u32 = 0xF68;

        let mut index = 0;
        while index < TASK_RECORDS {
            let record = object + TASK_RECORDS_OFFSET + index * TASK_RECORD_STRIDE;
            (record as *mut u32).write_unaligned(0);
            ((record + 4) as *mut u16).write_unaligned(0);
            index += 1;
        }
        ((object + OBJECT_FLAGS_OFFSET) as *mut u16).write_unaligned(0);
        ((object + OBJECT_SENTINEL_OFFSET) as *mut u32).write_unaligned(0);
        ((object + OBJECT_ZERO_BLOCK_0) as *mut u64).write_unaligned(0);
        ((object + OBJECT_ZERO_BLOCK_1) as *mut u64).write_unaligned(0);
        ((object + OBJECT_COUNTER_OFFSET) as *mut u32).write_unaligned(0);
    }
    object
});
