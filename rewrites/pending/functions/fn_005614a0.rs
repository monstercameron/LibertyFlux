// original: 0x005614A0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_71, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this leaderboard's column id when the caller expects it.
///
/// Reads the object's virtual table pointer from `this` and calls the slot at
/// `+4` (`VTABLE_SLOT`, the id query) with `this`. When the answer equals
/// `want` and `out` is non-null, writes the column descriptor address (file
/// VA `0xfd152c`, relocated at load) to `*out` and returns `out`; otherwise
/// returns 0 (a null `out` or a mismatch both give 0 without writing).
///
/// The original embeds the address as an immediate that the loader relocates
/// (proven: the original writes exactly file-value plus the load delta), so
/// the rewrite derives it with `relocated`, never hard-coded.
///
/// Original: 0x005614A0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_005614A0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const COLUMN_FILE_VA: u32 = 0xfd152c;
        let vtab = (this as *const u32).read_unaligned();
        let slot = (vtab.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if query(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(COLUMN_FILE_VA));
        out
    }
});
