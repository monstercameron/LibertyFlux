// original: 0x00540f30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_TeamGangBase, player_schema::LeaderboardInfo, 10>::vf7
///
/// leaderboard row lookup by index: fetch the leaderboard's key table for id 0x49, return row `index`.
///
/// Asks the leaderboard helper (callee 1, passed the leaderboard id in ECX
/// and an out-struct in EDX) for this board's key table; the table pointer
/// lands at out-struct offset 0x10. If the helper reports failure the
/// result is NOT_FOUND. Otherwise returns the `index`-th dword of the table
/// with no bounds check (a wild index faults on the original too).
///
/// Original: thiscall/1 in ECX (ignored) plus one stack word, the callee pops 4 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_00540f30(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x49;
        const TABLE_SLOT: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return NOT_FOUND;
        }
        let table = out[TABLE_SLOT];
        rd32(table.wrapping_add(index.wrapping_mul(4)))
    }
});
