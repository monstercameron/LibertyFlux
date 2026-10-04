// original: 0x00522770 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race44NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Stamp the caller's slot when this board's value matches.
///
/// `this` points to the board object (vtable pointer at word 0). Slot 1 of
/// the vtable is called with `this` and its result compared against `want`;
/// on a match with a non-null `out`, the mark 0xfd9a34 is stored to `out`.
/// The mark is an address (a file VA): it is stored relocated to the loaded
/// image, never as a raw constant.
/// Returns `out` on a storing match, else null.
/// Edge cases: a null `out` stores nothing but still matches; the vtable
/// call carries no stack arguments.
/// Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_00522770(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VALUE_SLOT_OFF: u32 = 4;
        const MARK: u32 = 0xfd9a34;

        let vtable = (this as *const u32).read_unaligned();
        let slot =
            ((vtable.wrapping_add(VALUE_SLOT_OFF)) as *const u32).read_unaligned();
        let get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if get(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(MARK));
        out
    }
});
