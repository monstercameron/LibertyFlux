// original: 0x0052d970 leaderboard_classify_size

/// Classify one dword of leaderboard schema data by index.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race24Standard, player_schema::LeaderboardInfo, 10>::vf8`.
///
/// `index` selects the dword through the same fetch as the sibling fetch
/// routine (schema kind `KIND`, array pointer at out-struct word `OUT_ARRAY`,
/// +0x14). A failed fetch yields 0. Otherwise the dword goes to the rank
/// callee in ECX; its answer maps to a size class: 1 -> 4, 2 or 3 -> 8,
/// 5 -> 4, and anything else (including -1 and 4) -> 0.
///
/// Calling convention: thiscall slot, `this` ignored, one stack argument,
/// callee pops 4. Both callees pop nothing; the rank callee answers in EAX.
lf_checker_rt::export!(thiscall, rw_0052d970(_this: u32, index: u32) -> u32 {
    /// Schema-table selector passed to the fetch callee in ECX.
    const KIND: u32 = 0x25;
    /// Word holding the data-array pointer inside the fetch out-struct (+0x14).
    const OUT_ARRAY: usize = 5;
    let mut out = [0u32; 6];
    out[OUT_ARRAY] = 0;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, KIND, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return 0;
    }
    let array = out[OUT_ARRAY];
    let raw = unsafe { (array.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() };
    let v = lf_checker_rt::callee_thiscall!(2, u32, raw);
    if v == 0xffff_ffff {
        return 0;
    }
    match v.wrapping_sub(1) {
        0 => 4,
        1 | 2 => 8,
        4 => 4,
        _ => 0,
    }
});
