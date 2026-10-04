// original: 0x00584770 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_199, player_schema::LeaderboardInfo, 10>::vf8

/// Size code of one leaderboard row's key, via the kind classifier.
///
/// `index` is a row number. The fetch callee (id 1) is called with this
/// instantiation's leaderboard id and a pointer to an eight-word stack
/// buffer; on success it leaves the key column pointer at `+0x14`. Only the
/// low byte of the fetch result is significant: zero means failure and yields
/// 0. There is no bounds check on the index.
///
/// The key at `keys + index * 4` is passed to the kind callee (id 2), which
/// returns a small kind code. The result maps kind 1 to 4, kinds 2 and 3 to
/// 8, kind 5 to 4, and anything else (including kind 4, kind 0 and -1) to 0.
///
/// Original: 0x00584770 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00584770(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x19f;
        const FETCH_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const KEYS_WORD: usize = 5;
        const SIZE_NARROW: u32 = 4;
        const SIZE_WIDE: u32 = 8;
        const SIZE_NONE: u32 = 0;
        #[inline(always)]
        unsafe fn read_u32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if (ok as u8) == 0 {
            return SIZE_NONE;
        }
        let keys = out[KEYS_WORD];
        let key = read_u32(keys.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, key);
        if kind == 0xffff_ffff {
            return SIZE_NONE;
        }
        match kind.wrapping_sub(1) {
            0 => SIZE_NARROW,
            1 | 2 => SIZE_WIDE,
            3 => SIZE_NONE,
            4 => SIZE_NARROW,
            _ => SIZE_NONE,
        }
    }
});
