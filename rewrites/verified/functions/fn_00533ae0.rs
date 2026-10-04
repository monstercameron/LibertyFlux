// original: 0x00533ae0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race47Standard, player_schema::LeaderboardInfo, 10>::vf12

/// Index of one entry inside this board's present-entry list, or -1.
///
/// Two stages. The leaderboard lookup callee (fastcall: board id in ECX,
/// out-struct in EDX, boolean in AL) returns the full id list (array at
/// struct offset `+0x14`) and the present-entry list (count at `+0x04`,
/// array at `+0x08`). This instantiation asks for board `LEADERBOARD_ID`.
///
/// The id at `index` in the full list is then located in the present list
/// by an unsigned-length linear search from 0. A missing id (-1) at the
/// position, lookup failure, an empty present list, or no match all give
/// `NOT_FOUND` (-1); otherwise the result is the first matching index.
///
/// The object pointer (`this`) is unused: the board is selected by the id
/// constant alone. Original: thiscall with one stack word, callee pops 4.

lf_checker_rt::export!(thiscall, rw_00533ae0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x8d;
        const NOT_FOUND: u32 = 0xffff_ffff;

        /// Out-struct filled by the lookup callee: present count at `+0x04`,
        /// present array at `+0x08`, full array at `+0x14`.
        #[repr(C)]
        struct EntryLists {
            _head: u32,
            present_count: u32,
            present: u32,
            _middle: [u32; 2],
            all: u32,
        }

        #[inline(always)]
        unsafe fn read_word(base: u32, index: u32) -> u32 {
            unsafe { (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() }
        }

        let mut lists = EntryLists { _head: 0, present_count: 0, present: 0, _middle: [0; 2], all: 0 };
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, &mut lists as *mut EntryLists as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let want = read_word(lists.all, index);
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        if lists.present_count == 0 {
            return NOT_FOUND;
        }
        let mut at = 0u32;
        while at < lists.present_count {
            if read_word(lists.present, at) == want {
                return at;
            }
            at += 1;
        }
        NOT_FOUND
    }
});
