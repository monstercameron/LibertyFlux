// original: 0x00531360 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race38Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's interface pointer when the id matches.
///
/// `this` is the object, `out` an out-pointer, `want` the requested
/// id. The object's virtual slot 1 is called (callee 1, reached
/// through the fabricated vtable on both sides) and its answer is
/// compared with `want`. On a match with a non-null `out`, the
/// interface constant 0xfd8ccc is stored through `out` and `out` is
/// returned; otherwise the result is null. The constant is an image
/// address with a relocation entry, so it is written relocated,
/// exactly as the loader-adjusted original writes it.
///
/// Original: 0x00531360 (thiscall, receiver in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_00531360(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const IFACE: u32 = 0xfd8ccc;
        const VTABLE_SLOT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let slot = vtable.wrapping_add(VTABLE_SLOT);
        let target = (slot as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        if probe(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(IFACE));
        out
    }
});
