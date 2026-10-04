// original: 0x00e63880 zero_table_and_notify
/// Zero 0x384 dwords at 0x11D4028, then notify with block 0xE71490.
///
/// Clears one global table with a counted store loop (the original uses a
/// repeat-stores instruction) and forwards a constant block pointer to the
/// shared notifier (cdecl/1, stubbed by the checker). Returns the notifier's
/// answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00e63880() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11D4028;
        const WORDS: usize = 0x384;
        const BLOCK: u32 = 0xE71490;
        let dst = relocated(TABLE) as *mut u32;
        for i in 0..WORDS {
            *dst.add(i) = 0;
        }
        callee_cdecl!(1, u32, relocated(BLOCK))
    }
});
