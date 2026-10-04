// original: 0x0052e1f0 leaderboard_fetch_indexed

/// Fetch one dword of leaderboard schema data by index.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race26Standard, player_schema::LeaderboardInfo, 10>::vf7`.
///
/// `index` selects the dword. The fetch callee (schema kind `KIND`) fills a
/// six-word out-struct on the stack; word `OUT_ARRAY` (+0x10) receives the
/// data-array pointer. When the callee reports failure (zero low byte) the
/// result is the `MISSING` sentinel (0xFFFF_FFFF); otherwise the result is
/// `array[index]` read as one little-endian dword.
///
/// Calling convention: thiscall slot, `this` ignored, one stack argument,
/// callee pops 4. The fetch callee takes (ECX = kind, EDX = out-struct),
/// pops nothing and answers in AL.
lf_checker_rt::export!(thiscall, rw_0052e1f0(_this: u32, index: u32) -> u32 {
    /// Schema-table selector passed to the fetch callee in ECX.
    const KIND: u32 = 0x2f;
    /// Word holding the data-array pointer inside the fetch out-struct (+0x10).
    const OUT_ARRAY: usize = 4;
    /// Failure sentinel.
    const MISSING: u32 = 0xffff_ffff;
    let mut out = [0u32; 6];
    out[OUT_ARRAY] = 0;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, KIND, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return MISSING;
    }
    let array = out[OUT_ARRAY];
    unsafe { (array.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() }
});
