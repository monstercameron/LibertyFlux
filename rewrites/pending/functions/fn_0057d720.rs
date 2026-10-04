// original: 0x0057d720 leaderboard_174_vf12_rowpos
/// Position of board 174's row value in the short list (virtual slot 12).
///
/// Same behaviour as the board-172 slot: column lookup, row value at
/// `index`, scan of the short column, position or -1.
export!(thiscall, rw_0057d720(_this: u32, index: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 8];
        let ok = callee_fastcall!(1, u32, 0x186, frame.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let col_a = frame[5];
        let v =
            (col_a.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if v == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = frame[1];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let col_b = frame[2];
        let mut i: u32 = 0;
        while i < count {
            let w = (col_b.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if w == v {
                return i;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
