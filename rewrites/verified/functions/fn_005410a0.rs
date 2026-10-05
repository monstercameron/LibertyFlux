// original: 0x005410a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_TeamMafya, player_schema::LeaderboardInfo, 10>::vf2
///
/// leaderboard tag store on id match: stamp the board tag when the live id matches.
///
/// Reads the current id through this object's virtual slot 1 (vtable word
/// at +4, called with this in ECX). If it differs from `want`, or `out` is
/// null, returns null. Otherwise stores the board tag at `out` and returns
/// `out`. The tag is an image address (file VA 0xfceb74, relocated at load),
/// so it goes through `relocated` like any image pointer.
///
/// Original: thiscall/2, the callee pops 8 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_005410a0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const TAG_FILEVA: u32 = 0xfceb74;
        const VTABLE_SLOT1: u32 = 4;
        let vtable = rd32(this);
        let slot = rd32(vtable.wrapping_add(VTABLE_SLOT1));
        let current: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if current(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG_FILEVA));
        out
    }
});
