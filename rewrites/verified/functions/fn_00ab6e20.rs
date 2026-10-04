// original: 0x00ab6e20 nested_table_flag
/// Two-level lookup: `key_a` in the outer table selects a sub-table, `key_b`
/// in it selects a node whose tag bit 4 is returned. Like the original, a
/// miss on either level reads a null-derived address and faults.
export!(thiscall, rw_00ab6e20(this: *const u8, key_a: u32, key_b: u32) -> u32 {
    unsafe {
        let cap_a = *(this.add(0xC) as *const u16) as u32;
        let mut sub = 0u32;
        if cap_a != 0 {
            let buckets = *(this.add(8) as *const u32) as *const u32;
            let mut n = *buckets.add((key_a % cap_a) as usize);
            while n != 0 {
                if *(n as *const u32) == key_a {
                    sub = n.wrapping_add(4);
                    break;
                }
                n = *((n + 0x14) as *const u32);
            }
        }
        let cap_b = *((sub + 8) as *const u16) as u32;
        if cap_b == 0 {
            return ((fault_byte() >> 4) & 1) as u32;
        }
        let buckets_b = *((sub + 4) as *const u32) as *const u32;
        let mut m = *buckets_b.add((key_b % cap_b) as usize);
        loop {
            if m == 0 {
                return ((fault_byte() >> 4) & 1) as u32;
            }
            if *(m as *const u32) == key_b {
                return (((*((m + 4) as *const u8)) >> 4) & 1) as u32;
            }
            m = *((m + 0x8C) as *const u32);
        }
    }
});

// Two-level lookup: `key_a` in the outer table selects a sub-table, `key_b`
// in it selects a node whose tag bit 4 is returned. Like the original, a
// miss on either level reads a null-derived address and faults.
// Reads absolute address 0 exactly like the original's miss path (which faults).

fn fault_byte() -> u8 {
    unsafe { *((core::hint::black_box(0u32)) as *const u8) }
}
