// original: 0x00539290 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopBombBase, player_schema::LeaderboardInfo, 10>::vf13
/// Search a helper-provided key list and return the mapped value at the match.
///
/// The fastcall metadata helper fills a local descriptor with a signed count,
/// a key array, and a parallel value array. A false helper result, a non-positive
/// count, or no matching key returns all ones. A match returns the corresponding
/// value word. Both the count guard and loop bound are signed comparisons.
lf_checker_rt::export!(thiscall, rw_00539290(_this: u32, argument: u32) -> u32 {
    unsafe {
        const WORD_BYTES: u32 = 4;
        const NOT_FOUND: u32 = u32::MAX;
        const QUERY_CALLEE: u32 = 1;
        const QUERY_SELECTOR: u32 = 3;
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
        struct KeyMapDescriptor {
            _reserved: [u32; 3],
            count: u32,
            keys: u32,
            values: u32,
        }
        const COUNT_OFFSET: usize = 12;
        const KEYS_OFFSET: usize = 16;
        const VALUES_OFFSET: usize = 20;
        const _: [(); COUNT_OFFSET] = [(); core::mem::offset_of!(KeyMapDescriptor, count)];
        const _: [(); KEYS_OFFSET] = [(); core::mem::offset_of!(KeyMapDescriptor, keys)];
        const _: [(); VALUES_OFFSET] = [(); core::mem::offset_of!(KeyMapDescriptor, values)];
        const _: [(); 24] = [(); core::mem::size_of::<KeyMapDescriptor>()];
        let mut query = KeyMapDescriptor {
            _reserved: [0; 3], count: 0, keys: 0, values: 0,
        };
        let query_address = &mut query as *mut KeyMapDescriptor as usize as u32;
        let helper_succeeded =
            (lf_checker_rt::callee_fastcall!(QUERY_CALLEE, u32, QUERY_SELECTOR, query_address) as u8) != 0;
        let count = query.count as i32;
        if !helper_succeeded || count <= 0 { return NOT_FOUND; }
        let mut key_index = 0i32;
        while key_index < count {
            let key = unsafe {
                read_word(indexed_address(query.keys, key_index as u32, WORD_BYTES))
            };
            if key == argument {
                if key_index == -1 { return NOT_FOUND; }
                return unsafe {
                    read_word(indexed_address(query.values, key_index as u32, WORD_BYTES))
                };
            }
            key_index = key_index.wrapping_add(1);
        }
        NOT_FOUND
    }
});
