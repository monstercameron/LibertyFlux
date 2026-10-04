// original: 0x00533de0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race47Standard, player_schema::LeaderboardInfo, 10>::vf8

/// Size class of the entry at a position in this board's id list.
///
/// `index` counts from 0 into the board's id array, which comes from the
/// leaderboard lookup callee (fastcall: board id in ECX, out-struct in EDX,
/// boolean in AL) at struct offset `+0x14`. This instantiation asks for
/// board `LEADERBOARD_ID`.
///
/// The id at `index` is classified through the entry-kind callee (thiscall:
/// id in ECX, kind in EAX), and the kind maps to a size: kinds 1 and 5 give
/// 4, kinds 2 and 3 give 8, anything else gives 0. Lookup failure also
/// gives 0.
///
/// The object pointer (`this`) is unused: the board is selected by the id
/// constant alone. Original: thiscall with one stack word, callee pops 4.

lf_checker_rt::export!(thiscall, rw_00533de0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x8d;
        const SMALL: u32 = 4;
        const LARGE: u32 = 8;

        /// Out-struct filled by the lookup callee: array at `+0x14`.
        #[repr(C)]
        struct EntryList {
            _reserved: [u32; 5],
            items: u32,
        }

        let mut list = EntryList { _reserved: [0; 5], items: 0 };
        let ok: u8 = lf_checker_rt::callee_fastcall!(1, u8, LEADERBOARD_ID, &mut list as *mut EntryList as u32);
        if ok == 0 {
            return 0;
        }
        let id = (list.items.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, id);
        match kind {
            1 | 5 => SMALL,
            2 | 3 => LARGE,
            _ => 0,
        }
    }
});
