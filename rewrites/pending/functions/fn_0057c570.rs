// original: 0x0057C570 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_170, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this leaderboard's column id when the caller expects it.
///
/// Reads the object's virtual table pointer from `this` and calls the slot at
/// `+4` (`VTABLE_SLOT`, the id query) with `this`. When the answer equals
/// `want` and `out` is non-null, writes the column descriptor address (file
/// VA `0xfdf5f4`, relocated at load) to `*out` and returns `out`; otherwise
/// returns 0 (a null `out` or a mismatch both give 0 without writing).
///
/// The original embeds the address as an immediate that the loader relocates
/// (proven: the original writes exactly file-value plus the load delta), so
/// the rewrite derives it with `relocated`, never hard-coded.
///
/// Original: 0x0057C570 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0057C570(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const COLUMN_FILE_VA: u32 = 0xfdf5f4;
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
