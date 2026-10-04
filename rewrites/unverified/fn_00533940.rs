// original: 0x00533940 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race46Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Entry id at a position in this board's id list, or -1.
///
/// `index` counts from 0 into the board's id array, which comes from the
/// leaderboard lookup callee (fastcall: board id in ECX, out-struct in EDX,
/// boolean in AL) at struct offset `+0x10`. This instantiation asks for
/// board `LEADERBOARD_ID`.
///
/// When the lookup fails the result is `NOT_FOUND` (-1). Otherwise it is the
/// array element at `index`; there is no bounds check, the caller passes a
/// valid position.
///
/// The object pointer (`this`) is unused: the board is selected by the id
/// constant alone. Original: thiscall with one stack word, callee pops 4.

lf_checker_rt::export!(thiscall, rw_00533940(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x8c;
        const NOT_FOUND: u32 = 0xffff_ffff;

        /// Out-struct filled by the lookup callee: array at `+0x10`.
        #[repr(C)]
        struct EntryList {
            _reserved: [u32; 4],
            items: u32,
        }

        let mut list = EntryList { _reserved: [0; 4], items: 0 };
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, &mut list as *mut EntryList as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        (list.items.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
