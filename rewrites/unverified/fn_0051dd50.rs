// original: 0x0051dd50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race27NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Rank lookup: fetch this leaderboard's id/key tables, take the key for
/// `index`, and return its position in the id table.
///
/// Calls the table-fetch callee with the leaderboard index
/// (`LEADERBOARD_INDEX`) in ECX and a six-word scratch frame in EDX; the
/// callee fills `count` at frame `+4`, the id-table pointer at `+8` and the
/// key-table pointer at `+20`. Returns `MISSING` (-1) when the fetch
/// reports failure (low byte of the answer clear), when the key is -1
/// (empty slot), when the count is zero, or when the key is absent from
/// the first `count` ids; otherwise the zero-based position. The scan
/// treats `count` as unsigned.
///
/// Original: 0x0051dd50 (thiscall, one stack word; `this` is unread).
lf_checker_rt::export!(thiscall, rw_0051dd50(this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_INDEX: u32 = 0x3d;
        const FETCH_CALLEE: u32 = 1;
        const MISSING: u32 = 0xffff_ffff;
        let _ = this;
        #[repr(C)]
        struct FetchOut {
            _pad0: u32,
            count: u32,
            ids: u32,
            _pad1: u32,
            _pad2: u32,
            keys: u32,
        }
        let mut out: FetchOut = core::mem::zeroed();
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_CALLEE,
            u32,
            LEADERBOARD_INDEX,
            (&mut out as *mut FetchOut) as u32
        );
        if (ok & 0xff) == 0 {
            return MISSING;
        }
        let key = ((out.keys.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        if key == MISSING {
            return MISSING;
        }
        let n = out.count;
        if n == 0 {
            return MISSING;
        }
        let mut i = 0u32;
        loop {
            let v =
                ((out.ids.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if v == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= n {
                return MISSING;
            }
        }
    }
});
