// original: 0x0057d160 leaderboard_172_vf8_width
/// Column width for ranked-race board 172 (virtual slot 8).
///
/// Same behaviour as the board-171 slot: schema lookup, type tag, width map
/// (tags 1/5 -> 4 bytes, tags 2/3 -> 8 bytes, anything else -> 0).
export!(thiscall, rw_0057d160(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x184, frame.as_mut_ptr() as u32);
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
