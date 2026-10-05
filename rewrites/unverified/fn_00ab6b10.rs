// original: 0x00ab6b10 idmap_flag_b (proposed)

/// Find `k0`, then look up `k1` in its sub-map and return a tag byte.
///
/// Probes the outer map inline (same layout as the sibling at 0x00ab6ac0):
/// an empty map or a missed chain returns 0 without calling. On a hit,
/// resolves `k1` through the sub-map lookup callee on the entry's payload
/// and returns the byte past its payload word, or 0 for a null answer.
///
/// Callees: 1 = sub-map lookup (thiscall, one word).
///
/// Original: 0x00ab6b10 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab6b10(this: u32, k0: u32, k1: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const TABLE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const NEXT_OFF: u32 = 0x14;
        const PAYLOAD_OFF: u32 = 4;
        let count = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let mut entry = ((table + (k0 % count) * 4) as *const u32).read_unaligned();
        while entry != 0 {
            if (entry as *const u32).read_unaligned() == k0 {
                break;
            }
            entry = ((entry + NEXT_OFF) as *const u32).read_unaligned();
        }
        if entry == 0 {
            return 0;
        }
        let sub = entry.wrapping_add(PAYLOAD_OFF);
        if sub == 0 {
            return 0;
        }
        let payload = lf_checker_rt::callee_thiscall!(LOOKUP, u32, sub, k1);
        if payload == 0 {
            return 0;
        }
        (((payload + 1)) as *const u8).read() as u32
    }
});
