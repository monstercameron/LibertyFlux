// original: 0x00cf9a10 climb_list_grow_append_slot (proposed)

/// Appends a slot to the pointer list embedded in a climb task (`+0x0` items,
/// `+0x4` count, `+0x6` capacity, all 16-bit counters) and returns the new
/// slot's address. When the list is full it grows the capacity by the given
/// amount (16-bit wraparound), allocates a new item array, copies the old
/// items over, frees the old array and installs the new one.
///
/// Original: 0x00cf9a10 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf9a10(this: u32, grow: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x0;
        const COUNT: u32 = 0x4;
        const CAPACITY: u32 = 0x6;
        const MALLOC_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        let count = ((this + COUNT) as *const u16).read_unaligned();
        let capacity = ((this + CAPACITY) as *const u16).read_unaligned();
        if count == capacity {
            let new_cap = capacity.wrapping_add(grow as u16);
            ((this + CAPACITY) as *mut u16).write_unaligned(new_cap);
            let bytes = (new_cap as u32).wrapping_mul(4);
            let fresh: u32 = lf_checker_rt::callee_cdecl!(MALLOC_CALLEE, u32, bytes);
            let old_items = ((this + ITEMS) as *const u32).read_unaligned();
            let mut i: u32 = 0;
            while (i as u16) < count {
                let v = ((old_items + i * 4) as *const u32).read_unaligned();
                ((fresh + i * 4) as *mut u32).write_unaligned(v);
                i += 1;
            }
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, old_items);
            ((this + ITEMS) as *mut u32).write_unaligned(fresh);
        }
        let n = ((this + COUNT) as *const u16).read_unaligned();
        let base = ((this + ITEMS) as *const u32).read_unaligned();
        ((this + COUNT) as *mut u16).write_unaligned(n.wrapping_add(1));
        base.wrapping_add((n as u32).wrapping_mul(4))
    }
});
