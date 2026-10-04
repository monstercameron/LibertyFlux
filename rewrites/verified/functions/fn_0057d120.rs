// original: 0x0057d120 leaderboard_172_vf7_get
/// Value at `index` in board 172's column (virtual slot 7).
///
/// Fetches the board's column through the shared leaderboard helper and
/// returns the entry at `index`, or -1 when the lookup fails.
export!(thiscall, rw_0057d120(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x184, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let col = frame[4];
        (col.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
