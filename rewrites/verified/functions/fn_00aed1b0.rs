// original: 0x00AED1B0 kv_heapify_pass (proposed)

/// One heapify pass over a key/value range, notifying per slot.
///
/// `base` and `end` delimit 8-byte entries; `n = (end - base) / 8` (shift).
/// Ranges shorter than two entries return at once. Otherwise the notify
/// callee is called for slot `(n - 2) / 2` and then for every lower slot
/// down to 0, each time with (`base`, slot, `n`, key, value, `ctx`).
/// Returns nothing meaningful (eax holds the last callee result, or entry
/// garbage on short ranges).
///
/// Original: 0x00AED1B0 (cdecl, three stack words, one direct callee).
lf_checker_rt::export!(cdecl, rw_00aed1b0(base: u32, end: u32, ctx: u32) -> () {
    unsafe {
        const ENTRY: u32 = 8;
        const NOTIFY_CALLEE: u32 = 1;
        let n = (end.wrapping_sub(base) as i32) >> 3;
        if n < 2 {
            return;
        }
        let mut slot = (n - 2) / 2;
        loop {
            let addr = base.wrapping_add((slot as u32).wrapping_mul(ENTRY));
            let key = ((addr) as *const u32).read_unaligned();
            let val = ((addr.wrapping_add(4)) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, base, slot as u32, n as u32, key, val, ctx);
            if slot == 0 {
                break;
            }
            slot -= 1;
        }
    }
});
