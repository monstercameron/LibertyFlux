// original: 0x00AF8470 veh_slots_all_guarded (proposed)

/// Test whether every slot 3..7 holds a guarded entry in its zone.
///
/// Each slot dword at `this + 4*i` packs a record-table index (low word) and
/// a sub-record selector (high word), paired with the signed zone word at
/// `this + 0x3E + 2*(i - 3)`. A slot passes only when its index is live (not
/// `0xFFFF`, non-null record row), the 0x1E flag byte has low nibble 2 with
/// bits 4..5 clear, the 0x1C byte has no high nibble, and the zone table row
/// for the index has one of bits 3..5 set in the byte selected by the zone
/// word. The first failing slot returns 0; 1 means all five passed.
///
/// Original: 0x00AF8470 (thiscall, no stack arguments, result in AL).
lf_checker_rt::export!(thiscall, rw_00AF8470(this: u32) -> u8 {
    unsafe {
        const RECORDS: u32 = 0x1178284;
        const ZONES: u32 = 0x1178384;
        const FIRST: u32 = 3;
        const LAST: u32 = 7;
        const ZONE_WORDS: u32 = 0x3E;
        const EMPTY: u16 = 0xFFFF;
        const SUB_LEN: u32 = 32;
        let records = lf_checker_rt::relocated(RECORDS);
        let zones = lf_checker_rt::relocated(ZONES);
        for i in FIRST..=LAST {
            let slot = ((this + i * 4) as *const u32).read_unaligned();
            let idx = slot as u16;
            if idx == EMPTY {
                return 0;
            }
            let row = ((records + (idx as u32) * 4) as *const u32).read_unaligned();
            if row == 0 {
                return 0;
            }
            let rec = row.wrapping_add((slot >> 16) * SUB_LEN);
            let b1e = ((rec + 0x1E) as *const u8).read();
            if b1e & 0x0F != 2 {
                return 0;
            }
            if ((rec + 0x1C) as *const u8).read() & 0xF0 != 0 {
                return 0;
            }
            if b1e & 0x30 >= 0x20 {
                return 0;
            }
            let zone = ((this + ZONE_WORDS + (i - FIRST) * 2) as *const i16).read_unaligned() as i32;
            let zrow = ((zones + (idx as u32) * 4) as *const u32).read_unaligned();
            let zb = ((zrow.wrapping_add((zone * 8 + 5) as u32)) as *const u8).read();
            if zb & 0x38 == 0 {
                return 0;
            }
        }
        1
    }
});
