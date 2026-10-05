// original: 0x00AF7620 veh_slots_find_matching (proposed)

/// Find the first slot 4..6 whose check accepts, and report it.
///
/// Each slot dword at `this + 4*i` packs a record-table index (low word) and
/// a sub-record selector (high word), paired with the signed zone word at
/// `this + 0x40 + 2*(i - 4)`. An empty index (`0xFFFF`) or a null record row
/// ends the search with 0. Otherwise the top two bits of the zone-table byte
/// selected by the zone word choose: 1 runs checker A (callee 2), 2 runs
/// checker B (callee 1), anything else moves to the next slot. Both checkers
/// take bit 5 of the record's 0x1F byte plus a zero word; an answer of 0 or 1
/// returns 1 at once, any other answer moves on. 0 means no slot matched.
///
/// Original: 0x00AF7620 (thiscall, no stack arguments, result in AL).
lf_checker_rt::export!(thiscall, rw_00AF7620(this: u32) -> u8 {
    unsafe {
        const RECORDS: u32 = 0x1178284;
        const ZONES: u32 = 0x1178384;
        const FIRST: u32 = 4;
        const LAST: u32 = 6;
        const ZONE_WORDS: u32 = 0x40;
        const EMPTY: u16 = 0xFFFF;
        const SUB_LEN: u32 = 32;
        const CHECK_B: u32 = 1;
        const CHECK_A: u32 = 2;
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
            let zone = ((this + ZONE_WORDS + (i - FIRST) * 2) as *const i16).read_unaligned() as i32;
            let rec = row.wrapping_add((slot >> 16) * SUB_LEN);
            let zrow = ((zones + (idx as u32) * 4) as *const u32).read_unaligned();
            let zb = ((zrow.wrapping_add((zone * 8 + 5) as u32)) as *const u8).read();
            let bit = ((((rec + 0x1F) as *const u8).read() >> 5) & 1) as u32;
            let answer = match zb >> 6 {
                1 => lf_checker_rt::callee_cdecl!(CHECK_A, u32, bit, 0u32),
                2 => lf_checker_rt::callee_cdecl!(CHECK_B, u32, bit, 0u32),
                _ => continue,
            };
            if answer == 0 || answer == 1 {
                return 1;
            }
        }
        0
    }
});
