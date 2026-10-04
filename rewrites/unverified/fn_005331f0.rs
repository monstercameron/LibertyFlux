// original: 0x005331f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race45Standard, player_schema::LeaderboardInfo, 10>::vf2

/// Confirm this leaderboard's identity and publish its descriptor address.
///
/// Calls the object's slot-1 virtual (vtable slot at `+4`, object pointer in
/// ECX) and compares the answer with `want`. When they differ, or when `out`
/// is null, returns 0. Otherwise stores this board's descriptor address
/// through `out` and returns `out`. The descriptor is an image address (it
/// has a relocation entry), so it is derived with `relocated()` from the
/// file VA, never hard-coded.
///
/// Original: 0x005331f0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_005331f0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const VTABLE_SLOT_ONE: u32 = 4;
        const DESCRIPTOR_FILE_VA: u32 = 0xfcf194;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtable = rd32(this);
        let slot = rd32(vtable.wrapping_add(VTABLE_SLOT_ONE));
        let target: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let got = target(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        let descriptor = lf_checker_rt::relocated(DESCRIPTOR_FILE_VA);
        unsafe { (out as *mut u32).write_unaligned(descriptor) };
        out
    }
});
