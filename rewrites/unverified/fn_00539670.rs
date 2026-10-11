// original: 0x00539670 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopBombBase_BG, player_schema::LeaderboardInfo, 10>::vf12
/// Resolve the indexed schema key, then find it in a helper-provided list.
///
/// The fastcall metadata helper fills a local descriptor with an unsigned count,
/// a candidate-key array, and an indexed schema-key array. A false helper result,
/// the all-ones key sentinel, a zero count, or no match returns all ones.
/// Otherwise the function returns the zero-based match index. The list bound
/// comparison is unsigned, so high-bit counts are treated as large values.
lf_checker_rt::export!(thiscall, rw_00539670(_this: u32, argument: u32) -> u32 {
    unsafe {
        const WORD_BYTES: u32 = 4;
        const NOT_FOUND: u32 = u32::MAX;
        const QUERY_CALLEE: u32 = 1;
        const QUERY_SELECTOR: u32 = 40;
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
        struct LookupDescriptor {
            _reserved0: u32,
            count: u32,
            candidate_keys: u32,
            _reserved3: u32,
            _reserved4: u32,
            selected_keys: u32,
        }
        const COUNT_OFFSET: usize = 4;
        const CANDIDATE_KEYS_OFFSET: usize = 8;
        const SELECTED_KEYS_OFFSET: usize = 20;
        const _: [(); COUNT_OFFSET] = [(); core::mem::offset_of!(LookupDescriptor, count)];
        const _: [(); CANDIDATE_KEYS_OFFSET] = [(); core::mem::offset_of!(LookupDescriptor, candidate_keys)];
        const _: [(); SELECTED_KEYS_OFFSET] = [(); core::mem::offset_of!(LookupDescriptor, selected_keys)];
        const _: [(); 24] = [(); core::mem::size_of::<LookupDescriptor>()];
        let mut query = LookupDescriptor {
            _reserved0: 0, count: 0, candidate_keys: 0, _reserved3: 0,
            _reserved4: 0, selected_keys: 0,
        };
        let query_address = &mut query as *mut LookupDescriptor as usize as u32;
        let helper_succeeded =
            (lf_checker_rt::callee_fastcall!(QUERY_CALLEE, u32, QUERY_SELECTOR, query_address) as u8) != 0;
        if !helper_succeeded { return NOT_FOUND; }
        let selected_key = unsafe {
            read_word(indexed_address(query.selected_keys, argument, WORD_BYTES))
        };
        if selected_key == NOT_FOUND || query.count == 0 { return NOT_FOUND; }
        let mut candidate_index = 0u32;
        while candidate_index < query.count {
            let candidate_key = unsafe {
                read_word(indexed_address(query.candidate_keys, candidate_index, WORD_BYTES))
            };
            if candidate_key == selected_key { return candidate_index; }
            candidate_index = candidate_index.wrapping_add(1);
        }
        NOT_FOUND
    }
});
