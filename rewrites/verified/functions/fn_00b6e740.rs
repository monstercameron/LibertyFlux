// original: 0x00b6e740 select_anim_flagged_and_invoke (proposed)
/// Resolve an animation id from a seat index and a flag byte, then invoke.
///
/// Same shape as its sibling at 0x00b6e6c0 with the seat at `+0x2c`, the
/// record at `+0x28`, and an extra flag byte at `+0x30`: seat 5 gives 0x151
/// when the flag is 0 and 0x155 otherwise; seats 6/7/8 give 0x153/0x152/0x154;
/// any other seat leaves the id at -1. Early return (id already set) and the
/// 8-word dispatcher call are otherwise identical.
///
/// Original: 0x00b6e740 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00b6e740(this: u32, arg: u32) -> u32 {
    unsafe {
        const ID_OFF: u32 = 0x1c;
        const OUT_OFF: u32 = 0x20;
        const REC_OFF: u32 = 0x28;
        const SEAT_OFF: u32 = 0x2c;
        const FLAG_OFF: u32 = 0x30;
        const KEY_OFF: u32 = 0x2e;
        const SLOT_OFF: u32 = 0xc4;
        const TABLE: u32 = 0x01295cd8;
        const UNSET: u32 = 0xffffffff;
        let idp = (this + ID_OFF) as *mut u32;
        if idp.read_unaligned() != UNSET {
            return 0;
        }
        let seat = ((this + SEAT_OFF) as *const u32).read_unaligned();
        match seat.wrapping_sub(5) {
            0 => {
                let f = ((this + FLAG_OFF) as *const u8).read();
                idp.write_unaligned(if f != 0 { 0x155 } else { 0x151 });
            }
            1 => idp.write_unaligned(0x153),
            2 => idp.write_unaligned(0x152),
            3 => idp.write_unaligned(0x154),
            _ => {}
        }
        let p = ((this + REC_OFF) as *const u32).read_unaligned();
        let w = ((p + KEY_OFF) as *const i16).read_unaligned() as i32 as u32;
        let t = (lf_checker_rt::relocated(TABLE).wrapping_add(w.wrapping_mul(4)) as *const u32).read_unaligned();
        let slot = ((t + SLOT_OFF) as *const u32).read_unaligned();
        let ans: u32 = lf_checker_rt::callee_cdecl!(1, u32, slot, this + ID_OFF, arg, p, 0, 0, 0, 1);
        ((this + OUT_OFF) as *mut u32).write_unaligned(ans);
        0
    }
});

