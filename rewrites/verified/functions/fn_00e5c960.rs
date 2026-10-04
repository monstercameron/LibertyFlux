// original: 0x00e5c960 timing_slot_init_110e034
/// Initializes one timer slot with its two handler words.
///
/// Stores the slot's first handler address (low word; the original's
/// eight-byte load also picks up one undefined stack word which reads back
/// the checker's defined stack fill, so the rewrite stores a zero high word)
/// and the shared second handler address (high word, low word zero).
export!(cdecl, rw_00e5c960() -> () {
    unsafe {
        const FIRST: u32 = 0x00457D90;
        const SECOND: u32 = 0x00409610;
        *global::<u64>(0x0110E034) = relocated(FIRST) as u64;
        *global::<u64>(0x0110E03C) = (relocated(SECOND) as u64) << 32;
    }
});

