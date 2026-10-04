// original: 0x00533290 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race45Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Value paired with an entry id in this board's lists, or -1.
///
/// `want` is the entry id to find. The lists come from the leaderboard
/// lookup callee (fastcall: board id in ECX, out-struct in EDX, boolean in
/// AL), which writes the entry count at struct offset `+0x0c`, a pointer to
/// the id array at `+0x10`, and a pointer to the parallel value array at
/// `+0x14`. This instantiation asks for board `LEADERBOARD_ID`.
///
/// When the lookup fails, when the count is not positive (compared signed),
/// or when no id equals `want`, the result is `NOT_FOUND` (-1). Otherwise
/// it is the value at the first matching position. (The original also
/// compares the found index against -1 on the way out; that index is always
/// in range, so the comparison never fires.)
///
/// The object pointer (`this`) is unused: the board is selected by the id
/// constant alone. Original: thiscall with one stack word, callee pops 4.

lf_checker_rt::export!(thiscall, rw_00533290(_this: u32, want: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x87;
        const NOT_FOUND: u32 = 0xffff_ffff;

        /// Out-struct filled by the lookup callee: count at `+0x0c`, ids at
        /// `+0x10`, parallel values at `+0x14`.
        #[repr(C)]
        struct EntryLists {
            _reserved: [u32; 3],
            count: u32,
            ids: u32,
            values: u32,
        }

        #[inline(always)]
        unsafe fn read_word(base: u32, index: u32) -> u32 {
            unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() }
        }

        let mut lists = EntryLists { _reserved: [0; 3], count: 0, ids: 0, values: 0 };
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, &mut lists as *mut EntryLists as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        if (lists.count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut index = 0u32;
        while index < lists.count {
            if read_word(lists.ids, index) == want {
                return read_word(lists.values, index);
            }
            index += 1;
        }
        NOT_FOUND
    }
});
