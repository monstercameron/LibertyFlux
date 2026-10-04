// original: 0x008f0230 ring_search_64
/// 64-step ring search over the object's slot table. Each 8-byte slot holds a
/// key byte and a count dword; starting from the object's cursor, up to 64
/// slots are visited with wraparound. The search succeeds early when a
/// slot's count drops below the start count minus the limit, fails when a
/// keyed slot byte falls outside the inclusive [low, high] window (in either
/// argument order), and succeeds when all 64 slots pass. Only the low 16 bits
/// of the result are defined: the pass flag in AL over the high arg byte.
export!(thiscall, rw_008f0230(this: u32, limit: u32, low: u32, high: u32) -> u32 {
    const SLOTS: u32 = 0x40;
    let start = unsafe { *((this.wrapping_add(8)) as *const u8) };
    let table = unsafe { *((this.wrapping_add(0xC)) as *const u32) };
    let key = unsafe { *((this.wrapping_add(4)) as *const u8) };
    let slot_count = |idx: u32| unsafe {
        *((table.wrapping_add(idx.wrapping_mul(8)).wrapping_add(4)) as *const u32)
    };
    let slot_key = |idx: u32| unsafe {
        *((table.wrapping_add(idx.wrapping_mul(8))) as *const u8)
    };
    let first = slot_count(start as u32);
    let mut floor = first.wrapping_sub(limit);
    if limit > first {
        floor = 0;
    }
    let lo = low as u8;
    let hi = high as u8;
    let mut step: u8 = 0;
    loop {
        // Wrapping cursor: slots before the start wrap past the ring end.
        let back = step.min(0x3F);
        let idx = if step <= start {
            (start as u32).wrapping_sub(step as u32)
        } else {
            (start as u32)
                .wrapping_sub(back as u32)
                .wrapping_add(SLOTS)
        } & 0xFF;
        if slot_count(idx) < floor {
            break;
        }
        let k = slot_key(idx) ^ key;
        let inside = if lo > hi {
            k >= hi && k <= lo
        } else {
            k >= lo && k <= hi
        };
        if !inside {
            return ((high & 0xFF) << 8) | 0;
        }
        step = step.wrapping_add(1);
        if step >= SLOTS as u8 {
            break;
        }
    }
    ((high & 0xFF) << 8) | 1
});
