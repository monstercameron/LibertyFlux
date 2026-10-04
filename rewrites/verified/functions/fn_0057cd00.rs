// original: 0x0057cd00 leaderboard_171_vf8_width
/// Column width for ranked-race board 171 (virtual slot 8).
///
/// Fetches the board's schema row for `index` through the shared leaderboard
/// helper, classifies the row's stored value with the shared type query, and
/// maps the tag to a width: tags 1 and 5 mean four bytes, tags 2 and 3 mean
/// eight bytes, anything else (including a failed lookup) means zero.
export!(thiscall, rw_0057cd00(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x183, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let rows = frame[5];
        let v =
            (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let tag = callee_thiscall!(2, u32, v);
        match tag {
            1 | 5 => 4,
            2 | 3 => 8,
            _ => 0,
        }
    }
});
