// original: 0x00625a40 chain_advance_scan
// Walk a chain advancing guarded cursor words past a key.
//
// For each link: when the guard word differs from the cursor and the key
// word equals it, advances the cursor (following the successor of a live
// cursor, or cleaning a tagged word when it is null) and clears the visited
// flag. Follows the head link until it ends. Returns the last cursor seen.
// (The null-entry path returns entry residue and is not exercised.)
export!(thiscall, rw_00625a40(obj: u32, key: u32) -> u32 {
    unsafe {
        const CURSOR_OFF: u32 = 4;
        const TAG_OFF: u32 = 8;
        const GUARD_OFF: u32 = 0x0c;
        const VISITED_OFF: u32 = 0x14;
        if obj == 0 {
            // Null entry returns entry residue in the original; pinned to 0
            // by the contract and not exercised (regs cannot alternate NULL).
            return 0;
        }
        let mut cur: u32 = obj;
        let mut last: u32;
        loop {
            let val: u32 = *((cur.wrapping_add(CURSOR_OFF)) as *const u32);
            last = val;
            if *((cur.wrapping_add(GUARD_OFF)) as *const u32) != val
                && *(key as *const u32) == val
            {
                if val != 0 {
                    last = *((val.wrapping_add(8)) as *const u32);
                    *((cur.wrapping_add(CURSOR_OFF)) as *mut u32) = last;
                } else {
                    let tag: u32 = *((cur.wrapping_add(TAG_OFF)) as *const u32);
                    last = tag;
                    if tag & 0xfffffffe != 0 && tag & 1 != 0 {
                        let clean: u32 = tag & 0xfffffffe;
                        *((cur.wrapping_add(TAG_OFF)) as *mut u32) = clean;
                        last = *(clean as *const u32);
                        *((cur.wrapping_add(CURSOR_OFF)) as *mut u32) = last;
                    }
                }
                *((cur.wrapping_add(VISITED_OFF)) as *mut u8) &= 0xfe;
            }
            cur = *(cur as *const u32);
            if cur == 0 {
                break;
            }
        }
        last
    }
});
