// original: 0x00538C60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompMafiaWork, player_schema::LeaderboardInfo, 10>::vf8
/// Map a helper-provided category result to the leaderboard's small status code.
///
/// The metadata helper supplies an indexed key array; the category helper receives
/// the selected key. A failed metadata lookup or all-ones category returns zero.
/// The category is decremented with wrapping arithmetic and dispatched through
/// five cases: slots zero and four return four, slots one and two return eight,
/// and every other slot returns zero.
lf_checker_rt::export!(thiscall, rw_00538C60(_this: u32, argument: u32) -> u32 {
    unsafe {
        const WORD_BYTES: u32 = 4;
        const NOT_FOUND: u32 = u32::MAX;
        const QUERY_CALLEE: u32 = 1;
        const QUERY_SELECTOR: u32 = 8;
        const CATEGORY_CALLEE: u32 = 2;
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        fn indexed_address(base: u32, index: u32, word_bytes: u32) -> u32 {
            base.wrapping_add(index.wrapping_mul(word_bytes))
        }
        #[repr(C)]
        struct CategoryDescriptor {
            _reserved: [u32; 5],
            keys: u32,
        }
        const KEYS_OFFSET: usize = 20;
        const _: [(); KEYS_OFFSET] = [(); core::mem::offset_of!(CategoryDescriptor, keys)];
        const _: [(); 24] = [(); core::mem::size_of::<CategoryDescriptor>()];
        const TABLE_INDEX_LIMIT: u32 = 4;
        const RESULT_FOUR: u32 = 4;
        const RESULT_EIGHT: u32 = 8;
        let mut query = CategoryDescriptor { _reserved: [0; 5], keys: 0 };
        let query_address = &mut query as *mut CategoryDescriptor as usize as u32;
        let helper_succeeded =
            (lf_checker_rt::callee_fastcall!(QUERY_CALLEE, u32, QUERY_SELECTOR, query_address) as u8) != 0;
        if !helper_succeeded { return 0; }
        let key = unsafe { read_word(indexed_address(query.keys, argument, WORD_BYTES)) };
        let category = lf_checker_rt::callee_thiscall!(CATEGORY_CALLEE, u32, key);
        if category == NOT_FOUND { return 0; }
        let slot = category.wrapping_sub(1);
        if slot > TABLE_INDEX_LIMIT { return 0; }
        match slot {
            0 => RESULT_FOUR,
            1 | 2 => RESULT_EIGHT,
            4 => RESULT_FOUR,
            _ => 0,
        }
    }
});
