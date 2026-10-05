// original: 0x0056ad80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_106, player_schema::LeaderboardInfo, 10>::vf2

/// Leaderboard vtable slot: publish this board's tag when the caller asks
/// for the id this object reports, otherwise report no match.
///
/// `this` is the leaderboard-info object. Its virtual slot at `+4` is asked
/// for an id; when that id equals `want_id` and `out_ptr` is non-null, the
/// board's constant tag is stored through `out_ptr` and `out_ptr` is
/// returned. Any mismatch (or a null `out_ptr`) returns 0 with no store.
/// The tag carries a relocation entry, so it is written relocated (in the
/// game, loaded at its preferred base, that is the file value itself).
///
/// Original: 0x0056AD80 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0056ad80(this: u32, out_ptr: u32, want_id: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const VTABLE_SLOT: u32 = 4;
        const LEADERBOARD_TAG: u32 = 0x00fdeb94;
        let vtable = rd32(this);
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_SLOT)) as usize);
        let id = get_id(this);
        if id != want_id {
            return 0;
        }
        if out_ptr == 0 {
            return 0;
        }
        wr32(out_ptr, lf_checker_rt::relocated(LEADERBOARD_TAG));
        out_ptr
    }
});
