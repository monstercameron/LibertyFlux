// original: 0x00539130 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompMafiaWork_BG, player_schema::LeaderboardInfo, 10>::vf9
/// Map a helper-provided category result to one of the class's result codes.
///
/// The fastcall metadata helper supplies an indexed key array, and the category
/// helper receives the selected key. A failed metadata lookup, all-ones category,
/// or category outside the five-case range returns all ones. After wrapping
/// subtraction, slots zero, one, two, three, and four return 0, 1, 3, all ones,
/// and 2 respectively.
lf_checker_rt::export!(thiscall, rw_00539130(_this: u32, argument: u32) -> u32 {
    unsafe {
        const WORD_BYTES: u32 = 4;
        const NOT_FOUND: u32 = u32::MAX;
        const QUERY_CALLEE: u32 = 1;
        const QUERY_SELECTOR: u32 = 29;
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
        let mut query = CategoryDescriptor { _reserved: [0; 5], keys: 0 };
        let query_address = &mut query as *mut CategoryDescriptor as usize as u32;
        let helper_succeeded =
            (lf_checker_rt::callee_fastcall!(QUERY_CALLEE, u32, QUERY_SELECTOR, query_address) as u8) != 0;
        if !helper_succeeded { return NOT_FOUND; }
        let key = unsafe { read_word(indexed_address(query.keys, argument, WORD_BYTES)) };
        let category = lf_checker_rt::callee_thiscall!(CATEGORY_CALLEE, u32, key);
        if category == NOT_FOUND { return NOT_FOUND; }
        let slot = category.wrapping_sub(1);
        if slot > TABLE_INDEX_LIMIT { return NOT_FOUND; }
        match slot {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => NOT_FOUND,
            4 => 2,
            _ => NOT_FOUND,
        }
    }
});
