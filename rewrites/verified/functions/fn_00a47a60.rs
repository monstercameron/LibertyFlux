// original: 0x00a47a60 Vehicle_GetDoor
/// Address of the first door record whose head word equals `want`, else null.
///
/// The door count is the signed dword at `this+0xF8C` (zero or negative
/// returns null at once); the records live at the pointer in `this+0xF88`
/// with stride 0x34 (thiscall, one stack word). The original's out-of-range
/// recheck after a hit is dead code: the loop only exits on a hit below the
/// count or by running past it.
export!(thiscall, rw_00a47a60(this: u32, want: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0xf8c;
        const RECS_OFF: u32 = 0xf88;
        const STRIDE: u32 = 0x34;
        let count = (this.wrapping_add(COUNT_OFF) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let base = (this.wrapping_add(RECS_OFF) as *const u32).read_unaligned();
        let mut i = 0i32;
        while i < count {
            let rec = base.wrapping_add((i as u32).wrapping_mul(STRIDE));
            if (rec as *const u32).read_unaligned() == want {
                return rec;
            }
            i += 1;
        }
        0
    }
});
