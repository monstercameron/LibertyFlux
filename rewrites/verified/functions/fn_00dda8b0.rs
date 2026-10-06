// original: 0x00dda8b0 UIBasicClip::vf129

/// Forward the argument, then push an adjusted float to the submit part.
///
/// `this` is the clip object and `arg` is one stack word. First `arg` is
/// forwarded to slot `FWD (+0x120)` of the part at `this + PART2 (+0x1e8)`.
/// Then the float from slot `MEASURE (+0x98)` of the clip's own table is
/// read (an ST0 float result): when the low byte of `arg` is nonzero it is
/// multiplied by the global factor `MUL_G`, and the global `SUB_G` is
/// subtracted either way. The adjusted bits are passed as the one stack
/// word to slot `SINK (+0xa0)` of the submit object at `this + SUBMIT
/// (+0x1e0)`, whose answer in EAX is the result.
///
/// The float multiply and subtract run in the original's operand order
/// (accumulator first, global second), pinned against commuting, so NaN
/// signs and payloads match bit for bit.
///
/// Original: thiscall, one stack word, callee pops it (the callee pops 4 bytes), three
/// indirect calls, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00dda8b0(this: u32, arg: u32) -> u32 {
    const PART2: u32 = 0x1e8;
    const SUBMIT: u32 = 0x1e0;
    const FWD: u32 = 0x120;
    const MEASURE: u32 = 0x98;
    const SINK: u32 = 0xa0;
    const MUL_G: u32 = 0x00fe_88c0;
    const SUB_G: u32 = 0x00fe_8ae0;
    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn sub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    unsafe {
        let part2 = ((this + PART2) as *const u32).read_unaligned();
        let ptable = (part2 as *const u32).read_unaligned();
        let fwd = ((ptable + FWD) as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(fwd as usize);
        f1(part2, arg);
        let table = (this as *const u32).read_unaligned();
        let measure = ((table + MEASURE) as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(measure as usize);
        let mut v = f2(this);
        if arg & 0xff != 0 {
            let factor = f32::from_bits(
                lf_checker_rt::global::<u32>(MUL_G).read_unaligned());
            v = mul(v, factor);
        }
        let minuend = f32::from_bits(
            lf_checker_rt::global::<u32>(SUB_G).read_unaligned());
        v = sub(v, minuend);
        let submit = ((this + SUBMIT) as *const u32).read_unaligned();
        let starget = (submit as *const u32).read_unaligned();
        let sink = ((starget + SINK) as *const u32).read_unaligned();
        let f3: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(sink as usize);
        f3(submit, v.to_bits())
    }
});
