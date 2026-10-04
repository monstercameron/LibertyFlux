// original: 0x005433A0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_6, player_schema::LeaderboardInfo, 10>::vf2
/// Publish this board's info record when the polled kind matches.
/// ///
/// /// Reads the vtable pointer from `this` and calls its second slot with
/// /// `this` in ECX (thiscall slot 0, intercepted by planting the stub
/// /// address in a fabricated vtable). When the returned kind differs from
/// /// `expected`, or `out` is null, the result is 0 and nothing is stored.
/// /// Otherwise the board's info-record address (a fixed image address) is
/// /// written to `*out` and `out` is returned. Original is thiscall with
/// /// two stack words.
lf_checker_rt::export!(thiscall, rw_005433a0(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const KIND_CALLEE: u32 = 0;
        const INFO_RECORD: u32 = 0x00FDD924;
        let vt = (this as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(4) as *const u32).read_unaligned();
        let get_kind: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let kind = get_kind(this);
        if kind != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        ((out) as *mut u32).write_unaligned(lf_checker_rt::relocated(INFO_RECORD));
        out
    }
});
