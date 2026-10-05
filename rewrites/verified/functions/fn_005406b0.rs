// original: 0x005406b0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_TeamCarSteal, player_schema::LeaderboardInfo, 10>::vf8
///
/// leaderboard row class size: classify one row's value into a field width.
///
/// Asks the helper (callee 1, id 0x40) for the row table (pointer at out
/// offset 20), feeds row `index`'s dword to the classifier (callee 2, value
/// in ECX), and maps its answer through a five-way switch: 1 maps to 4,
/// 2 and 3 to 8, 4 to 0, 5 to 4. A helper failure, a missing row
/// (classifier answer NOT_FOUND), or any other answer yields 0.
///
/// Original: thiscall/1, the callee pops 4 bytes.
#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

lf_checker_rt::export!(thiscall, rw_005406b0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x40;
        const ROWS_SLOT: usize = 5;
        const MISSING: u32 = 0xffff_ffff;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return 0;
        }
        let value = rd32(out[ROWS_SLOT].wrapping_add(index.wrapping_mul(4)));
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, value);
        if r == MISSING {
            return 0;
        }
        match r.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
