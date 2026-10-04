// original: 0x00549830 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_11, player_schema::LeaderboardInfo, 10>::vf2

/// Check this board's leaderboard id against an expected value and, on a match, publish the
/// board's info pointer.
///
/// Calls the second virtual slot of the object in `this` (a vtable call taking no arguments, `this`
/// still in ecx) to get the board's leaderboard id. If it differs from `expected`, or `out` is null,
/// returns 0. Otherwise writes the board's info-table address (`INFO_PTR`, a file VA in .rdata,
/// relocated at load) to `*out` and returns `out`.
///
/// Edge cases: id mismatch and null `out` both yield 0; the stored value is an image address, not a
/// small integer.
///
/// Original: 0x00549830 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00549830(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT: u32 = 4;
        const INFO_PTR: u32 = 0x00FD0284;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vtable = rd32(this);
        let target = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if get_id(this) != expected {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(INFO_PTR));
        out
    }
});
