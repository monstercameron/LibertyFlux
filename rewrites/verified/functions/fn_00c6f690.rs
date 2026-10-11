// original: 0x00C6F690 nm_message_find_slot

/// Search the NM message-slot pointer table for a record whose first and second dwords equal the query object's 16-bit fields at +0x18 and +0x16. Null entries are skipped. Return the record's dword at +8, or zero if no slot matches.
lf_checker_rt::export!(cdecl, rw_0x00c6f690(query: u32) -> u32 {
    const SLOT_COUNT_GLOBAL: u32 = 0x016D_D66C;
    const SLOT_TABLE_GLOBAL: u32 = 0x016D_D668;
    const QUERY_FIRST: usize = 0x18;
    const QUERY_SECOND: usize = 0x16;

    unsafe {
        let count = lf_checker_rt::global::<u16>(SLOT_COUNT_GLOBAL).read_unaligned() as u32;
        let entries = lf_checker_rt::global::<u32>(SLOT_TABLE_GLOBAL).read_unaligned() as *const u8;
        let query = query as *const u8;
        let first = core::ptr::read_unaligned(query.add(QUERY_FIRST) as *const u16) as u32;
        let second = core::ptr::read_unaligned(query.add(QUERY_SECOND) as *const u16) as u32;
        let mut index = 0u32;
        while index < count {
            let record = core::ptr::read_unaligned(entries.wrapping_add(index as usize * 4) as *const u32);
            if record != 0 {
                let first_match = core::ptr::read_unaligned(record as *const u32) == first;
                let second_match = core::ptr::read_unaligned((record as *const u8).add(4) as *const u32) == second;
                if first_match && second_match {
                    return core::ptr::read_unaligned((record as *const u8).add(8) as *const u32);
                }
            }
            index += 1;
        }
        0
    }
});
