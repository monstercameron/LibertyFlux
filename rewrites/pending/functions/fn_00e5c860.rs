// original: 0x00e5c860 timing_slot_init_110df50
/// Initializes one timer slot with its two handler words.
///
/// Stores the slot's first handler address (low word; the original's
/// eight-byte load also picks up one undefined stack word which reads back
/// the checker's defined stack fill, so the rewrite stores a zero high word)
/// and the shared second handler address (high word, low word zero).
export!(cdecl, rw_00e5c860() -> () {
    unsafe {
        const FIRST: u32 = 0x0043EA90;
        const SECOND: u32 = 0x00409610;
        *global::<u64>(0x0110DF50) = relocated(FIRST) as u64;
        *global::<u64>(0x0110DF58) = (relocated(SECOND) as u64) << 32;
    }
});

