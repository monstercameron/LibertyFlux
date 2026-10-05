// original: 0x00546c70 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_1, player_schema::LeaderboardInfo, 10>::vf2

/// Publishes this board's slot value when the caller names the live board.
///
/// Reads the current board id through the parent's virtual slot 1 (called
/// with this same object) and compares it with `want`. When they match and
/// `out` is non-null, stores this board's slot value `0xfdcbb4` there and
/// returns `out`; otherwise (mismatch, or a match with a null `out`)
/// returns 0. thiscall: object in ECX, `out` and `want` on the stack.
lf_checker_rt::export!(thiscall, rw_00546c70(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const SLOT_VALUE: u32 = 0xfdcbb4;
        const VTABLE_SLOT_CURRENT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let target = ((vtable.wrapping_add(VTABLE_SLOT_CURRENT))
            as *const u32)
            .read_unaligned();
        let current: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this)
        };
        if current != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(SLOT_VALUE);
        out
    }
});
