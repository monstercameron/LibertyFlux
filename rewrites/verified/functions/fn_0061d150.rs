// original: 0x0061D150 net_init_channel_table

/// Initialize a channel and clear its record table.
///
/// Runs the endpoint initializer over `this`, zeroes the table count
/// at `this+0x290`, and zeroes 32 records of 16 bytes starting at
/// `this+0x298` (the down-counter's bottom-checked loop runs once more
/// than its start value). Returns `this`.
/// Original: 0x0061D150 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061D150(this: u32) -> u32 {
    unsafe {
        const ENDPOINT_INIT: u32 = 1;
        const COUNT_SLOT: u32 = 0x290;
        const TABLE_BASE: u32 = 0x298;
        const RECORDS: u32 = 32;
        const RECORD_WORDS: u32 = 4;
        lf_checker_rt::callee_thiscall!(ENDPOINT_INIT, u32, this);
        ((this + COUNT_SLOT) as *mut u32).write_unaligned(0);
        for r in 0..RECORDS {
            for w in 0..RECORD_WORDS {
                ((this + TABLE_BASE + r * 0x10 + w * 4) as *mut u32).write_unaligned(0);
            }
        }
        this
    }
});
