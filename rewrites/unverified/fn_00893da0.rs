// original: 0x00893DA0 audSound_lookup_entry (proposed)

/// Look up a sound record by key under the table lock.
///
/// `this` points to the table owner: lock word at `+0x38`, present flag at
/// `+0x4d`, descriptor at `+0x7c`. Takes the lock (patched callee), and when
/// the flag is set binary-searches (patched callee, with `key`, the base at
/// descriptor `+0x14` and the bound at `+0x24`); a negative index yields null,
/// otherwise the slot pointer at `base + index * 8`. Always releases the lock
/// (patched callee) and returns the slot or null.
///
/// Original: 0x00893DA0 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00893da0(this: u32, key: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const BSEARCH: u32 = 2;
        const UNLOCK: u32 = 3;
        const LOCK_OFF: u32 = 0x38;
        const PRESENT_OFF: u32 = 0x4d;
        const DESC_OFF: u32 = 0x7c;
        const BASE_OFF: u32 = 0x14;
        const BOUND_OFF: u32 = 0x24;

        let lock = ((this + LOCK_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, lock);
        let mut slot = 0u32;
        if ((this + PRESENT_OFF) as *const u8).read() != 0 {
            let desc = ((this + DESC_OFF) as *const u32).read_unaligned();
            let base = ((desc + BASE_OFF) as *const u32).read_unaligned();
            let bound = ((desc + BOUND_OFF) as *const u32).read_unaligned();
            let idx = lf_checker_rt::callee_thiscall!(BSEARCH, u32, this, key, base, bound) as i32;
            if idx >= 0 {
                slot = ((base.wrapping_add((idx as u32).wrapping_mul(8))) as *const u32)
                    .read_unaligned();
            }
        }
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, lock);
        slot
    }
});
