// original: 0x00574ff0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_143, player_schema::LeaderboardInfo, 10>::vf13

/// Look up a leaderboard key and return its value, or -1 when absent.
///
/// `this` (ECX) is ignored; `key` is the value to find. Fetches the
/// leaderboard tables for id 0x167 through the fetch callee into a
/// six-word scratch structure, of which words 3..6 are the (count, keys,
/// values) triple used here. When the fetch reports failure (low byte of
/// the answer is zero) or the count is not positive, returns -1.
/// Otherwise linearly scans `keys[0..count]` for `key` (signed bound) and
/// returns `values[i]` at the first match, or -1 when no element matches.
/// The original re-checks the found index against -1; the index is never
/// negative, so that check is dead and is not reproduced.
///
/// Original: 0x00574ff0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00574ff0(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x167;
        const FETCH: u32 = 0;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        out[3] = 0;
        out[4] = 0;
        out[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let count = out[3] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = out[4];
        let mut i = 0u32;
        while i < count as u32 {
            if rd(keys.wrapping_add(i.wrapping_mul(4))) == key {
                let vals = out[5];
                return rd(vals.wrapping_add(i.wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
