// original: 0x00533f10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race48Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Tag an output slot when this board's key matches the wanted key.
///
/// Calls virtual slot 1 of the object in `this` (no arguments, key in EAX)
/// and compares the key with `want`. When they are equal and `out` is not
/// null, installs this board's vtable pointer `TAG_VA` (a relocated image
/// address: slot 2 of that table is this very function) at `*out` and
/// returns `out`. Otherwise returns 0 and writes nothing.
///
/// Original: thiscall with two stack words, callee pops 8.

lf_checker_rt::export!(thiscall, rw_00533f10(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        /// Linked address of the installed vtable (has a HIGHLOW
        /// reloc entry, so it moves with the image base).
        const TAG_VA: u32 = 0x00fd3374;

        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(4) as *const u32).read_unaligned();
        let key_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if key_of(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG_VA));
        out
    }
});
