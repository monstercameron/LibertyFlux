// original: 0x00AF8390 veh_slot_has_flagged (proposed)

/// Test whether any of the slots 3..5 carries a flagged entry.
///
/// Each slot dword at `this + 4*i` packs a table index (low word) and a
/// sub-record selector (high word). A slot is skipped when its index is the
/// empty marker `0xFFFF` or the global record table has a null row for it.
/// Otherwise the flag bytes at record + 0x1E/+0x1F decide: a set bit 0 with
/// clear bit 1 in the 0x1F byte returns 1 at once, and in every other case
/// the slot returns 1 when bits 4..5 of the 0x1E byte reach 0x20. Slots are
/// tried in order; 0 means no slot matched.
///
/// Original: 0x00AF8390 (thiscall, no stack arguments, result in AL).
lf_checker_rt::export!(thiscall, rw_00AF8390(this: u32) -> u8 {
    unsafe {
        const RECORDS: u32 = 0x1178284;
        const FIRST: u32 = 3;
        const LAST: u32 = 5;
        const EMPTY: u16 = 0xFFFF;
        const SUB_LEN: u32 = 32;
        let table = lf_checker_rt::relocated(RECORDS);
        for i in FIRST..=LAST {
            let slot = ((this + i * 4) as *const u32).read_unaligned();
            let idx = slot as u16;
            if idx == EMPTY {
                continue;
            }
            let row = ((table + (idx as u32) * 4) as *const u32).read_unaligned();
            if row == 0 {
                continue;
            }
            let rec = row.wrapping_add((slot >> 16) * SUB_LEN);
            let b1f = ((rec + 0x1F) as *const u8).read();
            if b1f & 1 != 0 {
                if b1f & 2 == 0 {
                    return 1;
                }
            }
            if ((rec + 0x1E) as *const u8).read() & 0x30 >= 0x20 {
                return 1;
            }
        }
        0
    }
});
