// original: 0x009B7E70 initialize_cam_instruction_defaults
/// Initializes thirteen adjacent camera instruction records with zero, one, and all-ones defaults. Each record is 0x30 bytes; the loop counter is zero when returned. The entry has five direct callers in the mapped code.
lf_checker_rt::export!(thiscall, rw_009b7e70(this: u32) -> u32 {
    const RECORD_COUNT: u32 = 13;
    const RECORD_STRIDE: u32 = 0x30;
    const STATE_ZERO: u32 = 0x00;
    const UNIT_FLOAT: u32 = 0x08;
    const TIMER_LIMIT: u32 = 0x14;
    const FIRST_INDEX: u32 = 0x18;
    const SECOND_INDEX: u32 = 0x1C;
    const THIRD_INDEX: u32 = 0x20;
    const ACTIVE_FLAG: u32 = 0x24;
    const RECORD_FLAG: u32 = 0x2C;
    const FLOAT_ONE_BITS: u32 = 0x3F800000;
    const DEFAULT_DURATION: u32 = 1000;
    for record_index in 0..RECORD_COUNT {
        let record = this.wrapping_add(record_index * RECORD_STRIDE);
        unsafe {
            (record.wrapping_add(STATE_ZERO) as *mut u32).write_unaligned(0);
            (record.wrapping_add(STATE_ZERO + 4) as *mut u32).write_unaligned(0);
            (record.wrapping_add(UNIT_FLOAT) as *mut u32).write_unaligned(FLOAT_ONE_BITS);
            (record.wrapping_add(STATE_ZERO + 0x0C) as *mut u32).write_unaligned(0);
            (record.wrapping_add(STATE_ZERO + 0x10) as *mut u32).write_unaligned(0);
            (record.wrapping_add(TIMER_LIMIT) as *mut u32).write_unaligned(DEFAULT_DURATION);
            (record.wrapping_add(FIRST_INDEX) as *mut u32).write_unaligned(u32::MAX);
            (record.wrapping_add(SECOND_INDEX) as *mut u32).write_unaligned(u32::MAX);
            (record.wrapping_add(THIRD_INDEX) as *mut u32).write_unaligned(u32::MAX);
            (record.wrapping_add(ACTIVE_FLAG) as *mut u32).write_unaligned(1);
            (record.wrapping_add(RECORD_FLAG) as *mut u8).write(0);
        }
    }
    0
});
