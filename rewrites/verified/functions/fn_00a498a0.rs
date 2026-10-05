// original: 0x00a498a0 Vehicle_GetWheel
/// Address of the first wheel record whose head word equals `want`, else null.
///
/// Same shape as the door lookup: the signed wheel count is at `this+0xF84`
/// (zero or negative returns null), the records live at the pointer in
/// `this+0xF80` with stride 0x170 (thiscall, one stack word).
export!(thiscall, rw_00a498a0(this: u32, want: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0xf84;
        const RECS_OFF: u32 = 0xf80;
        const STRIDE: u32 = 0x170;
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
