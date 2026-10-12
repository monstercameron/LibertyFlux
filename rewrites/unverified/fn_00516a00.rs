// original: 0x00516A00 rage::rlConcreteLeaderboard<player_schema::Leaderboard_Ranked_Race1NoHolds,player_schema::LeaderboardInfo>::LeaderboardInfo>
///
/// Report whether no row matching `key` sits in a rejected node.
///
/// `this` points to a holder whose first word heads a node list linked
/// through +0x1C. Each node carries a flags word at +0, a byte entry count
/// at +5 and an entry array at +0x14, with entries of 0x10 bytes matched
/// against `key` exactly as the row search does (type bytes equal and
/// exactly 3, both word ORs nonzero, sub-type bytes and both words equal;
/// an entry whose key is all zero never matches). The first matching entry
/// anywhere decides: its node's flags must have bits 1 and 4 set and bit 3
/// clear, giving 1, else 0. A list with no matching entry, or an empty list,
/// gives 1.
///
/// thiscall: the holder pointer arrives in ECX, the key pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00516a00(this: u32, key: u32) -> u32 {
    const NEXT_AT: u32 = 0x1C;
    const COUNT_AT: u32 = 5;
    const ENTRIES_AT: u32 = 0x14;
    const ENTRY_STRIDE: u32 = 0x10;
    const TYPE_AT: u32 = 8;
    const SUB_AT: u32 = 9;
    const WANTED_TYPE: u8 = 3;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let key_w0 = read_u32(key);
        let key_w4 = read_u32(key.wrapping_add(4));
        let key_type = read_u8(key.wrapping_add(TYPE_AT));
        let key_sub = read_u8(key.wrapping_add(SUB_AT));
        let mut node = read_u32(this);
        while node != 0 {
            let count = read_u8(node.wrapping_add(COUNT_AT)) as u32;
            let entries = read_u32(node.wrapping_add(ENTRIES_AT));
            let mut i = 0u32;
            while i < count {
                let entry = entries.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
                i += 1;
                let entry_type = read_u8(entry.wrapping_add(TYPE_AT));
                if entry_type != key_type {
                    continue;
                }
                if entry_type < 1 {
                    continue;
                }
                if entry_type != WANTED_TYPE {
                    continue;
                }
                if read_u32(entry) | read_u32(entry.wrapping_add(4)) == 0 {
                    continue;
                }
                if key_type.wrapping_sub(1) > 2 {
                    continue;
                }
                if key_type != WANTED_TYPE {
                    continue;
                }
                if key_w0 | key_w4 == 0 {
                    continue;
                }
                if read_u8(entry.wrapping_add(SUB_AT)) != key_sub {
                    continue;
                }
                if read_u32(entry) != key_w0 {
                    continue;
                }
                if read_u32(entry.wrapping_add(4)) != key_w4 {
                    continue;
                }
                let flags = read_u32(node);
                let ok = flags & 0x02 != 0 && flags & 0x10 != 0 && flags & 0x08 == 0;
                return if ok { 1 } else { 0 };
            }
            node = read_u32(node.wrapping_add(NEXT_AT));
        }
        1
    }
});
