// original: 0x00526d60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race60NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's descriptor slot when its value matches.
///
/// Calls the sibling value getter through the object's function table
/// (slot 1, `this` still in `ecx`), compares the result with `expect`,
/// and on equality writes this instantiation's descriptor pointer
/// (`SLOT_FILE_VA`, a relocated image address) through `out`. Returns
/// `out` on success, 0 when the value differs or `out` is null.
///
/// Original: thiscall with two stack words (`out`, `expect`).
lf_checker_rt::export!(thiscall, rw_00526d60(this: u32, out: u32, expect: u32) -> u32 {
    unsafe {
        const SLOT_FILE_VA: u32 = 0xfdb5fc;
        const VTABLE_SLOT: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vtable = rd32(this);
        let target = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let get: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let v = get(this);
        if v != expect {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        let slot = lf_checker_rt::relocated(SLOT_FILE_VA);
        unsafe { (out as *mut u32).write_unaligned(slot); }
        out
    }
});
