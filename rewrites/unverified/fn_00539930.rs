// original: 0x00539930 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopBombBase_BG, player_schema::LeaderboardInfo, 10>::vf7
/// Return the indexed word from the value array produced by the metadata helper.
///
/// A false fastcall-helper result returns all ones. Otherwise the input index is
/// multiplied by four with 32-bit wrapping and used without a bounds check.
lf_checker_rt::export!(thiscall, rw_00539930(_this: u32, argument: u32) -> u32 {
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
        struct IndexedValuesDescriptor {
            _reserved: [u32; 4],
            values: u32,
            _reserved5: u32,
        }
        const VALUES_OFFSET: usize = 16;
        const _: [(); VALUES_OFFSET] = [(); core::mem::offset_of!(IndexedValuesDescriptor, values)];
        const _: [(); 24] = [(); core::mem::size_of::<IndexedValuesDescriptor>()];
        let mut query = IndexedValuesDescriptor {
            _reserved: [0; 4], values: 0, _reserved5: 0,
        };
        let query_address = &mut query as *mut IndexedValuesDescriptor as usize as u32;
        let helper_succeeded =
            (lf_checker_rt::callee_fastcall!(QUERY_CALLEE, u32, QUERY_SELECTOR, query_address) as u8) != 0;
        if !helper_succeeded { return NOT_FOUND; }
        let value_address = indexed_address(query.values, argument, WORD_BYTES);
        unsafe { read_word(value_address) }
    }
});
