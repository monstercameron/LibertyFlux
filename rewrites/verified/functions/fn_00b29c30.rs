// original: 0x00b29c30 slot_id_set

/// Stores a new id word for one file slot, balancing two references.
///
/// When `idx` is above 23 (unsigned) returns it unchanged and does nothing.
/// Otherwise, when the slot's current id word is nonzero, calls the release
/// callee (thiscall: the old value as object, the word address as argument);
/// writes `val` into the word; and when `val` is nonzero calls the retain
/// callee (thiscall: `val` as object, same address). Both callees pop their
/// argument, which is what keeps the caller's frame balanced. Returns the
/// last callee's answer, or `idx` when no call ran. Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b29c30(idx: u32, val: u32) -> u32 {
    unsafe {
        const SLOT_IDS: u32 = 0x01657650;
        const MAX_SLOT: u32 = 0x17;
        const RELEASE: u32 = 0;
        const RETAIN: u32 = 1;
        if idx > MAX_SLOT {
            return idx;
        }
        let cell = lf_checker_rt::relocated(SLOT_IDS) + idx.wrapping_mul(4);
        let mut r = idx;
        let old = (cell as *const u32).read_unaligned();
        if old != 0 {
            r = lf_checker_rt::callee_thiscall!(RELEASE, u32, old, cell);
        }
        (cell as *mut u32).write_unaligned(val);
        if val != 0 {
            r = lf_checker_rt::callee_thiscall!(RETAIN, u32, val, cell);
        }
        r
    }
});
