// original: 0x00597250 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_268, player_schema::LeaderboardInfo, 10>::vf12

/// Find the leaderboard row for slot INDEX in table 0x1e0.
///
/// `index` selects a slot. The function asks the leaderboard registry
/// (callee 1) for table 0x1e0, passing a scratch record in edx: on success the
/// record holds the row count (word at +0x04), the row-id array (word at
/// +0x08) and the slot table (word at +0x14). The slot table is indexed by
/// INDEX (unchecked); an entry of -1 yields -1, else the first COUNT rows
/// are scanned for that id (the count is unsigned: only zero skips the scan)
/// and the matching row index wins, or -1 when nothing matches. Registry
/// failure yields -1.
///
/// Original: stdcall, one stack argument; the registry callee takes
/// (table id, record) in (ecx, edx) and answers in al.
lf_checker_rt::export!(stdcall, rw_00597250(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1e0;
        const REGISTRY_LOOKUP: u32 = 1;
        const COUNT_OFF: u32 = 0x04;
        const LIST_OFF: u32 = 0x08;
        const SLOTS_OFF: u32 = 0x14;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut record = [0u32; 6];
        let base = record.as_mut_ptr() as u32;
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(REGISTRY_LOOKUP, u32, LEADERBOARD_ID, base);
        if (answer & 0xff) == 0 {
            return NOT_FOUND;
        }
        let slots = (base.wrapping_add(SLOTS_OFF) as *const u32).read_unaligned();
        let wanted = (slots.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if wanted == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = (base.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        if count == 0 {
            return NOT_FOUND;
        }
        let list = (base.wrapping_add(LIST_OFF) as *const u32).read_unaligned();
        let mut index: i32 = 0;
        loop {
            let entry = (list.wrapping_add((index as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if entry == wanted {
                return index as u32;
            }
            index = index.wrapping_add(1);
            if (index as u32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
