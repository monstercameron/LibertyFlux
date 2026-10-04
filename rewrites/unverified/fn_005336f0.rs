// original: 0x005336f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race46Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Index of an entry id in this board's id list, or -1.
///
/// `want` is the entry id to find. The list comes from the leaderboard
/// lookup callee (fastcall: board id in ECX, out-struct in EDX, boolean in
/// AL), which writes the entry count at struct offset `+0x0c` and a pointer
/// to the id array at `+0x10`. This instantiation asks for board
/// `LEADERBOARD_ID`.
///
/// When the lookup fails, when the count is not positive (compared signed),
/// or when no element equals `want`, the result is `NOT_FOUND` (-1).
/// Otherwise it is the index of the first equal element, scanning from 0.
///
/// The object pointer (`this`) is unused: the board is selected by the id
/// constant alone. Original: thiscall with one stack word, callee pops 4.

lf_checker_rt::export!(thiscall, rw_005336f0(_this: u32, want: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x8c;
        const NOT_FOUND: u32 = 0xffff_ffff;

        /// Out-struct filled by the lookup callee: count at `+0x0c`, array at `+0x10`.
        #[repr(C)]
        struct EntryList {
            _reserved: [u32; 3],
            count: u32,
            items: u32,
        }

        #[inline(always)]
        unsafe fn read_word(base: u32, index: u32) -> u32 {
            unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() }
        }

        let mut list = EntryList { _reserved: [0; 3], count: 0, items: 0 };
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, &mut list as *mut EntryList as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        if (list.count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut index = 0u32;
        while index < list.count {
            if read_word(list.items, index) == want {
                return index;
            }
            index += 1;
        }
        NOT_FOUND
    }
});
