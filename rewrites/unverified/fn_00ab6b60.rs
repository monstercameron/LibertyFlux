// original: 0x00ab6b60 idmap_select (proposed)

/// Resolve a key pair through two nested maps and select a result word.
///
/// Looks up `k0` in the outer map (buckets at `+8`, 16-bit count at
/// `+0x0c`, link at `+0x14`); a miss dereferences the null map and faults.
/// Its payload is the inner map header (buckets at `+4`, count at `+8`,
/// link at `+0x8c`) probed for `k1`, faulting the same way on a miss. On
/// two hits compares `sel` against the flag byte past the inner payload
/// word: `sel` below it returns the word at index `k1` past the payload,
/// otherwise the word right past it.
///
/// Original: 0x00ab6b60 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00ab6b60(this: u32, k0: u32, k1: u32, sel: u32) -> u32 {
    unsafe {
        const O_TABLE: u32 = 8;
        const O_COUNT: u32 = 0x0C;
        const O_NEXT: u32 = 0x14;
        const I_TABLE: u32 = 4;
        const I_COUNT: u32 = 8;
        const I_NEXT: u32 = 0x8C;
        const PAYLOAD: u32 = 4;
        let o_count = ((this + O_COUNT) as *const u16).read_unaligned() as u32;
        let mut outer = 0u32;
        if o_count != 0 {
            let o_table = ((this + O_TABLE) as *const u32).read_unaligned();
            let mut e = ((o_table + (k0 % o_count) * 4) as *const u32).read_unaligned();
            while e != 0 {
                if (e as *const u32).read_unaligned() == k0 {
                    break;
                }
                e = ((e + O_NEXT) as *const u32).read_unaligned();
            }
            if e != 0 {
                outer = e.wrapping_add(PAYLOAD);
            }
        }
        // A missed outer lookup leaves `outer` null and the inner header
        // read faults, exactly as the original's null dereference does.
        let i_count = ((outer + I_COUNT) as *const u16).read_unaligned() as u32;
        let mut inner = 0u32;
        if i_count != 0 {
            let i_table = ((outer + I_TABLE) as *const u32).read_unaligned();
            let mut e = ((i_table + (k1 % i_count) * 4) as *const u32).read_unaligned();
            while e != 0 {
                if (e as *const u32).read_unaligned() == k1 {
                    break;
                }
                e = ((e + I_NEXT) as *const u32).read_unaligned();
            }
            if e != 0 {
                inner = e.wrapping_add(PAYLOAD);
            }
        }
        // A missed inner lookup faults on the flag-byte read below.
        let flag = ((inner + 1) as *const u8).read() as u32;
        if sel < flag {
            ((inner + 8).wrapping_add(k1.wrapping_mul(4)) as *const u32).read_unaligned()
        } else {
            ((inner + 8) as *const u32).read_unaligned()
        }
    }
});
