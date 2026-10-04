// original: 0x009e9800 ped_find_live_candidate
/// Walks the candidates from `[this+0x78]` (first, then next)
/// looking for one whose score at `+0x34` reaches 0.9 and passes the
/// live check; true on the first such hit, false when the walk ends.
/// (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e9800(this_ptr: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x78;
        const SCORE_OFF: u32 = 0x34;
        const THRESH: u32 = 0xFE88BC;
        const WALK_A0: u32 = 0;
        const WALK_A1: u32 = 4;
        let t = f32::from_bits(lf_checker_rt::global::<u32>(THRESH).read_unaligned());
        let inner = (this_ptr.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let mut cur: u32 = lf_checker_rt::callee_thiscall!(1, u32, inner, WALK_A0, WALK_A1);
        loop {
            if cur == 0 {
                return 0;
            }
            let x = f32::from_bits((cur.wrapping_add(SCORE_OFF) as *const u32).read_unaligned());
            if x < t || x != x {
                cur = lf_checker_rt::callee_thiscall!(2, u32, inner, WALK_A0, WALK_A1);
                continue;
            }
            let ok: u32 = lf_checker_rt::callee_thiscall!(3, u32, cur);
            if ok & 0xFF != 0 {
                return 1;
            }
            cur = lf_checker_rt::callee_thiscall!(2, u32, inner, WALK_A0, WALK_A1);
        }
    }
});
