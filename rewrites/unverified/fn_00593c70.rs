// original: 0x00593C70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_255, player_schema::LeaderboardInfo, 10>::vf8
/// Query the entry at the supplied index, ask its virtual method for the
/// entry category, then map the category to the stored width. Category
/// returns 1 and 5 select four bytes, 2 and 3 select eight bytes, and all
/// other values select zero. The local query record is six dwords; its sixth
/// word points to the entry-pointer array. Both engine calls are represented
/// by scripted stand-ins, with the second call using the selected object in
/// ECX. The original method pops its one stack argument.
lf_checker_rt::export!(thiscall, rw_00593c70(this: u32, index: u32) -> u32 {
    const QUERY_SELECTOR: u32 = 0x0000015A;
    const QUERY_CALLEE: u32 = 1;
    const CATEGORY_CALLEE: u32 = 2;
    const QUERY_OUTPUT_WORDS: usize = 6;
    const ENTRIES_POINTER_WORD: usize = 5;

    let mut query_output = [0u32; QUERY_OUTPUT_WORDS];
    let query_ready = lf_checker_rt::callee_fastcall!(
        QUERY_CALLEE, u32, QUERY_SELECTOR, query_output.as_mut_ptr() as u32
    );
    if query_ready as u8 == 0 {
        return 0;
    }

    let entries = query_output[ENTRIES_POINTER_WORD];
    let entry_offset = index.wrapping_mul(4);
    let entry = unsafe { (entries.wrapping_add(entry_offset) as *const u32).read_unaligned() };
    let category = lf_checker_rt::callee_thiscall!(CATEGORY_CALLEE, u32, entry);
    if category == u32::MAX {
        return 0;
    }

    match category.wrapping_sub(1) {
        0 => 4,
        1 | 2 => 8,
        3 => 0,
        4 => 4,
        _ => 0,
    }
});
