// original: 0x00576530 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_148, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this race's row vtable when the row's type tag matches.
///
/// `this` is the leaderboard row, `out_ptr` receives this race's row vtable
/// address and `want` is the expected type tag. The tag query runs through
/// the row's own vtable slot 1 (thiscall, `this` in ECX). When the returned
/// tag equals `want` and `out_ptr` is non-null, the vtable address is stored
/// there and `out_ptr` is returned; any mismatch or a null `out_ptr` yields
/// 0. The stored address is image-relative (a relocation entry covers it).
lf_checker_rt::export!(thiscall, rw_00576530(this: u32, out_ptr: u32, want: u32) -> u32 {
    unsafe {
        /// Vtable slot, in bytes, holding the row-type tag query.
        const TAG_SLOT: u32 = 4;
        /// File VA of this race's row vtable, published through `out_ptr`.
        const ROW_VTABLE: u32 = 0xFD992C;
        let vt = (this as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(TAG_SLOT) as *const u32).read_unaligned();
        let tag_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if tag_of(this) != want {
            return 0;
        }
        if out_ptr == 0 {
            return 0;
        }
        (out_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(ROW_VTABLE));
        out_ptr
    }
});
