// original: 0x0052eb50 leaderboard_classify_kind

/// Classify one dword of leaderboard schema data by kind.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race28Standard, player_schema::LeaderboardInfo, 10>::vf9`.
///
/// Same fetch-and-rank shape as the sibling size classifier (schema kind
/// `KIND`, array pointer at out-struct word `OUT_ARRAY`, +0x14), with a
/// different answer map: 1 -> 0, 2 -> 1, 3 -> 3, 5 -> 2, and anything else
/// (including -1 and 4) -> -1.
///
/// Calling convention: thiscall slot, `this` ignored, one stack argument,
/// callee pops 4. Both callees pop nothing; the rank callee answers in EAX.
lf_checker_rt::export!(thiscall, rw_0052eb50(_this: u32, index: u32) -> u32 {
    /// Schema-table selector passed to the fetch callee in ECX.
    const KIND: u32 = 0x5;
    /// Word holding the data-array pointer inside the fetch out-struct (+0x14).
    const OUT_ARRAY: usize = 5;
    /// Failure sentinel.
    const MISSING: u32 = 0xffff_ffff;
    let mut out = [0u32; 6];
    out[OUT_ARRAY] = 0;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, KIND, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return MISSING;
    }
    let array = out[OUT_ARRAY];
    let raw = unsafe { (array.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() };
    let v = lf_checker_rt::callee_thiscall!(2, u32, raw);
    if v == MISSING {
        return MISSING;
    }
    match v.wrapping_sub(1) {
        0 => 0,
        1 => 1,
        2 => 3,
        4 => 2,
        _ => MISSING,
    }
});
