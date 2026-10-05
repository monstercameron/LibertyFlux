// original: 0x00ab6ac0 idmap_flag_a (proposed)

/// Find `key` and return the payload's flag byte, else 0.
///
/// Probes the map inline (bucket array at `+8`, 16-bit count at `+0x0c`,
/// chain link at `+0x14`): an empty map or a missed chain returns 0 without
/// calling. On a hit, re-resolves the key through the lookup callee and
/// returns the byte at its payload. A null callee answer faults reading
/// address 0, on both sides alike.
///
/// Callees: 1 = canonical lookup (thiscall, one word).
///
/// Original: 0x00ab6ac0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ab6ac0(this: u32, key: u32) -> u32 {
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
        let mut entry = ((table + (key % count) * 4) as *const u32).read_unaligned();
        while entry != 0 {
            if (entry as *const u32).read_unaligned() == key {
                break;
            }
            entry = ((entry + NEXT_OFF) as *const u32).read_unaligned();
        }
        if entry == 0 {
            return 0;
        }
        let found = entry.wrapping_add(PAYLOAD_OFF);
        if found == 0 {
            return 0;
        }
        let payload = lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, key);
        ((payload) as *const u8).read() as u32
    }
});
