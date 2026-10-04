// original: 0x008ef000 find_unique_group_match
/// Walk group lists following unique matches until the walk settles.
///
/// Starting from `arg0`, each step scans the current record's group list
/// (found like [`rw_008eef90`] does, through the global table and the
/// instance map at `this + 0x804`): entries resolving to no record, to the
/// excluded record, or to a record whose status nibble is set are skipped; a
/// candidate shallower than `arg0`'s depth fails the whole walk (returns 0)
/// while deeper ones are skipped. Zero or several matches accept (returns 1);
/// exactly one match continues the walk from it, excluding the record just
/// left. The walk fails (returns 0) when it returns to `arg0`, or when
/// `arg0`'s own depth is zero.
export!(thiscall, rw_008ef000(this_ptr: u32, arg0: u32, arg1: u32) -> u8 {
    unsafe {
        const MAP_OFF: u32 = 0x804;
        const LISTS_VA: u32 = 0x1178384;
        let lists = global::<u32>(LISTS_VA);
        let map = this_ptr.wrapping_add(MAP_OFF) as *const u32;
        let depth = (*((arg0 + 0x1B) as *const u8) >> 5) as u32;
        if depth == 0 {
            return 0;
        }
        let mut cur = arg0;
        let mut excluded = arg1;
        loop {
            let n = (*((cur + 0x1E) as *const u8) & 0xF) as usize;
            if n == 0 {
                return 1;
            }
            let gidx = *((cur + 8) as *const u16) as usize;
            let base = *lists.add(gidx);
            let start = *((cur + 0x12) as *const i16) as isize;
            let mut slot = base.wrapping_add((start as u32).wrapping_mul(8));
            let mut matches = 0u32;
            let mut last = 0u32;
            for _ in 0..n {
                let v = *(slot as *const u32);
                slot = slot.wrapping_add(8);
                let idx = (v & 0xFFFF) as usize;
                let ptr = *map.add(idx);
                if ptr == 0 {
                    continue;
                }
                let rec = ptr.wrapping_add((v >> 16).wrapping_mul(32));
                if rec == 0 || rec == excluded {
                    continue;
                }
                if *((rec + 0x1C) as *const u8) & 0xF0 != 0 {
                    continue;
                }
                let d = (*((rec + 0x1B) as *const u8) >> 5) as u32;
                if d < depth {
                    return 0;
                }
                if d != depth {
                    continue;
                }
                matches += 1;
                last = rec;
            }
            if matches == 0 || matches >= 2 {
                return 1;
            }
            excluded = cur;
            cur = last;
            if last == arg0 {
                return 0;
            }
        }
    }
});
