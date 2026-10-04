// original: 0x0051ea40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race30NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Fetch this leaderboard's rank key and, when it matches the caller's
/// expected key, publish this leaderboard's tag through the out pointer.
///
/// `this` points to the leaderboard-info object: word 0 is its virtual
/// table, whose slot at `VTABLE_KEY_SLOT` is the key-fetch callee (called
/// with `this` still in ECX). When the fetched key equals `expect` and
/// `out` is non-null, the leaderboard tag (file VA `TAG`, an address in
/// the original's image, relocated at run time) is stored to `*out`.
/// Returns `out` when the keys match (even when `out` is null, returning
/// 0) and the fetched key when they differ.
///
/// Original: 0x0051ea40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0051ea40(this: u32, out: u32, expect: u32) -> u32 {
    unsafe {
        const VTABLE_KEY_SLOT: u32 = 4;
        const TAG: u32 = 0x00fcf81c;
        let vtable = (this as *const u32).read_unaligned();
        let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable.wrapping_add(VTABLE_KEY_SLOT)) as *const u32).read_unaligned() as usize,
        );
        let got = fetch(this);
        if got != expect {
            return got;
        }
        if out != 0 {
            (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG));
        }
        out
    }
});
