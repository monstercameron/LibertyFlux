// original: 0x00589bb0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_219, player_schema::LeaderboardInfo, 10>::vf2

/// Leaderboard-handle match for one ranked episodic-race leaderboard.
///
/// Calls the second virtual slot of the object in `this` (slot at vtable
/// byte 4, object passed in ECX, no stack arguments) and compares the result
/// with `want`. When they match and `out` is non-null, writes the image
/// address 0xfddc9c (file VA, relocated at load) to `out` and returns `out`;
/// otherwise returns null (mismatch, or null `out`). Original is thiscall
/// with two stack arguments (the callee pops 8 bytes).

lf_checker_rt::export!(thiscall, rw_00589bb0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const VTABLE_SLOT: u32 = 4;
        const HANDLE_FILE_VA: u32 = 0xfddc9c;

        let vtable = rd32(this);
        let slot = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let fetch: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let got = fetch(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        unsafe { (out as *mut u32).write_unaligned(lf_checker_rt::relocated(HANDLE_FILE_VA)) };
        out
    }
});
