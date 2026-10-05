// original: 0x00A8E530 CInteriorInst::vf26 (proposed bounds clamp)

/// Clamp two interior bound pairs into an output box.
///
/// `out` (one stack word) receives four floats seeded to `+1e6, -1e6, -1e6,
/// +1e6`. Each of the four object bounds (`this+0xC0`, `+0xC4`, `+0xD0`,
/// `+0xD4`) then tightens one slot: the first pair is also limited to
/// `±1e6` from the read-only constants. Every update happens only on an
/// ordered greater-than, so NaN bounds keep the previous value. No calls.
///
/// Original: thiscall, one stack word (output pointer), no return value.
lf_checker_rt::export!(thiscall, rw_00A8E530(this: u32, out: u32) -> u32 {
    unsafe {
        const B0_OFF: u32 = 0xc0;
        const B1_OFF: u32 = 0xc4;
        const B2_OFF: u32 = 0xd0;
        const B3_OFF: u32 = 0xd4;
        const HI_BITS: u32 = 0xe9d138;
        const LO_BITS: u32 = 0xe82788;
        const POS_SEED: u32 = 0x49742400;
        const NEG_SEED: u32 = 0xc9742400;
        let hi = f32::from_bits(
            (lf_checker_rt::relocated(HI_BITS) as *const u32).read_unaligned(),
        );
        let lo = f32::from_bits(
            (lf_checker_rt::relocated(LO_BITS) as *const u32).read_unaligned(),
        );
        let b0 = ((this + B0_OFF) as *const f32).read_unaligned();
        let b1 = ((this + B1_OFF) as *const f32).read_unaligned();
        let b2 = ((this + B2_OFF) as *const f32).read_unaligned();
        let b3 = ((this + B3_OFF) as *const f32).read_unaligned();
        ((out + 0) as *mut u32).write_unaligned(POS_SEED);
        ((out + 12) as *mut u32).write_unaligned(POS_SEED);
        ((out + 8) as *mut u32).write_unaligned(NEG_SEED);
        ((out + 4) as *mut u32).write_unaligned(NEG_SEED);
        if hi > b0 {
            ((out + 0) as *mut f32).write_unaligned(b0);
        }
        if b0 > lo {
            ((out + 8) as *mut f32).write_unaligned(b0);
        }
        if hi > b1 {
            ((out + 12) as *mut f32).write_unaligned(b1);
        }
        if b1 > lo {
            ((out + 4) as *mut f32).write_unaligned(b1);
        }
        let t = ((out + 0) as *const f32).read_unaligned();
        if t > b2 {
            ((out + 0) as *mut f32).write_unaligned(b2);
        }
        let t = ((out + 8) as *const f32).read_unaligned();
        if b2 > t {
            ((out + 8) as *mut f32).write_unaligned(b2);
        }
        let t = ((out + 12) as *const f32).read_unaligned();
        if t > b3 {
            ((out + 12) as *mut f32).write_unaligned(b3);
        }
        let t = ((out + 4) as *const f32).read_unaligned();
        if b3 > t {
            ((out + 4) as *mut f32).write_unaligned(b3);
        }
        0
    }
});
