// original: 0x008895E0 stream_lookup_entry_by_kind (proposed)

/// Find a streaming entry by key, dispatching on the owner's kind.
///
/// `obj` points to the owner, `key` is the lookup key. When the kind word at
/// `obj + 0x24` is 1 the lookup goes to the record finder (patched callee)
/// and its result is returned. Otherwise the table lock at `obj + 0x38` is
/// taken (patched callee); a null table at `+0x3c` or either busy flag at
/// `+0x46`/`+0x47` releases the lock (patched callee) and yields null.
/// Otherwise the sorted search (patched callee, with `key`, the base at
/// table `+0` and the bound at `+0x10`) gives an index; a negative index
/// yields null, else the slot at `base + index * 8` after unlocking.
///
/// Original: 0x008895E0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_008895e0(obj: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_LOCK: u32 = 1;
        const SORTED_SEARCH: u32 = 2;
        const TABLE_UNLOCK: u32 = 3;
        const RECORD_FIND: u32 = 4;
        const KIND_OFF: u32 = 0x24;
        const LOCK_OFF: u32 = 0x38;
        const TABLE_OFF: u32 = 0x3c;
        const BUSY_OFF: u32 = 0x46;
        const BUSY2_OFF: u32 = 0x47;

        if ((obj + KIND_OFF) as *const u32).read_unaligned() == 1 {
            return lf_checker_rt::callee_thiscall!(RECORD_FIND, u32, obj, key);
        }
        let lock = ((obj + LOCK_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(TABLE_LOCK, u32, lock);
        let mut slot = 0u32;
        let table = ((obj + TABLE_OFF) as *const u32).read_unaligned();
        if table != 0
            && ((obj + BUSY2_OFF) as *const u8).read() == 0
            && ((obj + BUSY_OFF) as *const u8).read() == 0
        {
            let base = (table as *const u32).read_unaligned();
            let bound = ((table + 0x10) as *const u32).read_unaligned();
            let idx = lf_checker_rt::callee_thiscall!(SORTED_SEARCH, u32, obj, key, base, bound) as i32;
            if idx >= 0 {
                slot = ((base.wrapping_add((idx as u32).wrapping_mul(8))) as *const u32)
                    .read_unaligned();
            }
        }
        lf_checker_rt::callee_cdecl!(TABLE_UNLOCK, u32, lock);
        slot
    }
});
