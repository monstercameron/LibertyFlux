// original: 0x00a911c0 stream_pick_slot_by_measure (proposed)

/// Pick one of two slot values by a float measure the callee reports.
///
/// The word at `this+0x2e` (signed) indexes a global pointer table; the
/// pointed object's dword at `+0x70` and the first argument go to the
/// selector (callee 1, thiscall/1), whose answer points at two candidate
/// dwords (`+0x40`, `+0x44`). The measurer (callee 2, thiscall/2) takes the
/// second argument and a stack out-slot pre-set to 1000.0 for the
/// `arg0*96` entry past `this+0x80`. A zero low byte in its answer selects
/// nothing (-1). Else the reported float `f` is compared: a negative `f`
/// above the lower constant selects the first candidate at once; otherwise
/// a positive `f` below the upper constant selects the second, else nothing.
///
/// Returns the selected candidate or -1. Thiscall: object in ecx, two
/// stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00a911c0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x2e;
        const KIND_TABLE: u32 = 0x01295cd8;
        const SEL_OFF: u32 = 0x70;
        const CAND0: u32 = 0x40;
        const CAND1: u32 = 0x44;
        const BASE_OFF: u32 = 0x80;
        const INIT_BITS: u32 = 0x447a0000;
        const LO_CONST: u32 = 0x00fe8d70;
        const HI_CONST: u32 = 0x00fe87e4;
        const NOTHING: u32 = 0xffffffff;
        let kind = ((this + KIND_OFF) as *const i16).read_unaligned() as i32 as u32;
        let obj = ((lf_checker_rt::relocated(KIND_TABLE) + kind.wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        let sel = ((obj + SEL_OFF) as *const u32).read_unaligned();
        let cands = lf_checker_rt::callee_thiscall!(1, u32, sel, a0);
        let first = ((cands + CAND0) as *const u32).read_unaligned();
        let second = ((cands + CAND1) as *const u32).read_unaligned();
        let entry = ((this + BASE_OFF) as *const u32).read_unaligned()
            + a0.wrapping_mul(3).wrapping_mul(32);
        let mut slot = INIT_BITS;
        let ans = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            entry,
            a1,
            &mut slot as *mut u32 as u32
        );
        let mut ret = NOTHING;
        if (ans & 0xff) != 0 {
            let f = f32::from_bits(slot);
            if 0.0 > f {
                let lo = f32::from_bits(
                    (lf_checker_rt::relocated(LO_CONST) as *const u32).read_unaligned(),
                );
                if f > lo {
                    return first;
                }
            }
            if f > 0.0 {
                let hi = f32::from_bits(
                    (lf_checker_rt::relocated(HI_CONST) as *const u32).read_unaligned(),
                );
                if hi > f {
                    ret = second;
                }
            }
        }
        ret
    }
});
