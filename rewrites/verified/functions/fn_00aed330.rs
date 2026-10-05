// original: 0x00AED330 kv_insert_step (proposed)

/// One insertion-sort step over 8-byte entries: make room for a pair.
///
/// `pos` points at the entry the pair belongs near, `key`/`val` are the
/// pair to insert. While the entry just below the hole has a key above
/// `key` (unsigned), that pair shifts down one slot; the pair is then
/// written at the hole. The fourth stack word is read by no instruction.
/// Returns the value word (left in eax); there is no lower bound, so
/// callers keep a sentinel key at the array start.
///
/// Original: 0x00AED330 (cdecl, four stack words, no calls).
lf_checker_rt::export!(cdecl, rw_00aed330(pos: u32, key: u32, val: u32, _ctx: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 8;
        const KEY_OFF: u32 = 0;
        const VAL_OFF: u32 = 4;
        let mut hole = pos;
        while key < ((hole.wrapping_sub(ENTRY).wrapping_add(KEY_OFF)) as *const u32).read_unaligned() {
            let pk = ((hole.wrapping_sub(ENTRY).wrapping_add(KEY_OFF)) as *const u32).read_unaligned();
            let pv = ((hole.wrapping_sub(ENTRY).wrapping_add(VAL_OFF)) as *const u32).read_unaligned();
            ((hole.wrapping_add(KEY_OFF)) as *mut u32).write_unaligned(pk);
            ((hole.wrapping_add(VAL_OFF)) as *mut u32).write_unaligned(pv);
            hole = hole.wrapping_sub(ENTRY);
        }
        ((hole.wrapping_add(KEY_OFF)) as *mut u32).write_unaligned(key);
        ((hole.wrapping_add(VAL_OFF)) as *mut u32).write_unaligned(val);
        // The original returns with eax holding the loaded value word.
        val
    }
});
