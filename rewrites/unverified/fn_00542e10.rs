// original: 0x00542E10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_4, player_schema::LeaderboardInfo, 10>::vf8
/// Byte size of one leaderboard column value by row index.
/// ///
/// /// Calls the leaderboard-info callee (fastcall slot 0) with this board's
/// /// numeric id in ECX and a scratch info block in EDX. When the callee
/// /// reports failure the result is 0. Otherwise the info block's
/// /// value-array pointer (at +0x14) is read, the row at `index` is passed
/// /// in ECX to the kind callee (thiscall slot 1), and the returned kind is
/// /// mapped through a size table: kind 1 takes 4 bytes, kinds 2 and 3 take
/// /// 8, kind 5 takes 4, and anything else (including kind 4 and -1) takes
/// /// 0. The object pointer in ECX is unused. Original is thiscall with one
/// /// stack word; the table is a jump table in the original.
lf_checker_rt::export!(thiscall, rw_00542e10(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADER_ID: u32 = 0xA4;
        const INFO_VALUES: u32 = 0x14;
        const INFO_CALLEE: u32 = 0;
        const KIND_CALLEE: u32 = 1;
        const SIZE_OF_KIND: [u32; 5] = [4, 8, 8, 0, 4];
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADER_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let values = info[(INFO_VALUES / 4) as usize];
        let v = (values as *const u32).add(index as usize).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, v);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        let t = kind.wrapping_sub(1);
        if t > 4 {
            return 0;
        }
        SIZE_OF_KIND[t as usize]
    }
});
