// original: 0x0057d9e0 leaderboard_174_vf7_get
/// Value at `index` in board 174's column (virtual slot 7).
///
/// Same behaviour as the board-172 slot: column lookup, indexed read, -1 on
/// lookup failure.
export!(thiscall, rw_0057d9e0(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x186, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let col = frame[4];
        (col.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
