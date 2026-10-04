// original: 0x00b6e7d0 select_anim_float_gated_and_invoke (proposed)
/// Resolve an animation id with a float-gated override, then invoke.
///
/// Same dispatcher shape as its siblings (seat at `+0x28`, record at `+0x24`):
/// seats 5/6/7/8 first give 0x13b/0x13d/0x13c/0x13e. Then, when the dword at
/// record+0x1300 is 1, a float at record+0x20's record +8 overrides the id:
/// for seats 5 and 6 the id becomes 0x166 when NOT(f > threshold) and 0x164
/// otherwise, where the threshold is a global float; for other seats it
/// becomes 0x167 when NOT(0.0 > f) and 0x165 otherwise. The negated
/// greater-than matches `comiss`+`setbe`, which also takes the first id for
/// NaN inputs.
///
/// Original: 0x00b6e7d0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00b6e7d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const ID_OFF: u32 = 0x1c;
        const OUT_OFF: u32 = 0x20;
        const REC_OFF: u32 = 0x24;
        const SEAT_OFF: u32 = 0x28;
        const INNER_OFF: u32 = 0x20;
        const GATE_OFF: u32 = 0x1300;
        const FVAL_OFF: u32 = 8;
        const KEY_OFF: u32 = 0x2e;
        const SLOT_OFF: u32 = 0xc4;
        const TABLE: u32 = 0x01295cd8;
        const THRESH: u32 = 0x00fe8628;
        const UNSET: u32 = 0xffffffff;
        let idp = (this + ID_OFF) as *mut u32;
        if idp.read_unaligned() != UNSET {
            return 0;
        }
        let seat = ((this + SEAT_OFF) as *const u32).read_unaligned();
        match seat.wrapping_sub(5) {
            0 => idp.write_unaligned(0x13b),
            1 => idp.write_unaligned(0x13d),
            2 => idp.write_unaligned(0x13c),
            3 => idp.write_unaligned(0x13e),
            _ => {}
        }
        let p = ((this + REC_OFF) as *const u32).read_unaligned();
        if ((p + GATE_OFF) as *const u32).read_unaligned() == 1 {
            let x = ((p + INNER_OFF) as *const u32).read_unaligned();
            let f = ((x + FVAL_OFF) as *const f32).read_unaligned();
            if seat == 5 || seat == 6 {
                let g = (lf_checker_rt::relocated(THRESH) as *const f32).read_unaligned();
                idp.write_unaligned(if !(f > g) { 0x166 } else { 0x164 });
            } else {
                idp.write_unaligned(if !(0.0 > f) { 0x167 } else { 0x165 });
            }
        }
        let w = ((p + KEY_OFF) as *const i16).read_unaligned() as i32 as u32;
        let t = (lf_checker_rt::relocated(TABLE).wrapping_add(w.wrapping_mul(4)) as *const u32).read_unaligned();
        let slot = ((t + SLOT_OFF) as *const u32).read_unaligned();
        let ans: u32 = lf_checker_rt::callee_cdecl!(1, u32, slot, this + ID_OFF, arg, p, 0, 0, 0, 1);
        ((this + OUT_OFF) as *mut u32).write_unaligned(ans);
        0
    }
});

