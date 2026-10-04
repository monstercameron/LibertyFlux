// original: 0x0057d630 leaderboard_173_vf9_rowkind
/// Row kind for ranked-race board 173 (virtual slot 9).
///
/// Fetches the board's schema row for `index` through the shared leaderboard
/// helper, classifies the row's stored value with the shared type query, and
/// maps the tag to a row kind: tag 1 means 0, tag 2 means 1, tag 3 means 3,
/// tag 5 means 2, and anything else (including a failed lookup) means -1.
export!(thiscall, rw_0057d630(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x185, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame[5];
        let v =
            (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let tag = callee_thiscall!(2, u32, v);
        match tag {
            1 => 0,
            2 => 1,
            3 => 3,
            5 => 2,
            _ => 0xFFFF_FFFF,
        }
    }
});
