// original: 0x0088E4A0 rage::audVoiceSoft::vf3

/// Start this software voice on its child and report the level state in
/// the voice flags.
///
/// `this` points to the voice and `a0` is a rate word. When synth bit 4
/// of the flags at `+0x8c` is set, the child's start entry (callee 1) runs
/// with mode 1. Otherwise the rate helper (callee 2) converts `a0` with
/// the word at `+0xc`, and the child at `+0x130` starts (callee 3) with
/// twice that answer. Either way the level word through `+0x4` is then
/// relayed to the level entry (callee 4); when the level is zero, flag
/// bits 0 and 6 are set, otherwise bit 3 is set (NaN counts as non-zero).
///
/// Original: 0x0088E4A0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088E4A0(this: u32, a0: u32) -> () {
    unsafe {
        const FLAGS: u32 = 0x8c;
        const SYNTH: u8 = 0x10;
        const LEVEL_PTR: u32 = 0x04;
        const RATE_BASE: u32 = 0x0c;
        const CHILD: u32 = 0x130;
        const START_MODE: u32 = 1;
        const ZERO_BITS: u8 = 0x41;
        const NONZERO_BIT: u8 = 0x08;
        const CHILD_START: u32 = 1;
        const RATE_HELPER: u32 = 2;
        const CHILD_START2: u32 = 3;
        const LEVEL_ENTRY: u32 = 4;

        if ((this + FLAGS) as *const u8).read() & SYNTH != 0 {
            lf_checker_rt::callee_thiscall!(CHILD_START, u32, this, START_MODE);
        } else {
            let base = ((this + RATE_BASE) as *const u32).read_unaligned();
            let ans = lf_checker_rt::callee_cdecl!(RATE_HELPER, u32, a0, base);
            let child = ((this + CHILD) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(
                CHILD_START2,
                u32,
                child,
                ans.wrapping_mul(2)
            );
        }
        let lp = ((this + LEVEL_PTR) as *const u32).read_unaligned();
        let w = (lp as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(LEVEL_ENTRY, u32, this, w);
        let lp = ((this + LEVEL_PTR) as *const u32).read_unaligned();
        let w = (lp as *const u32).read_unaligned();
        let f = (this + FLAGS) as *mut u8;
        if f32::from_bits(w) != 0.0 {
            f.write(f.read() | NONZERO_BIT);
        } else {
            f.write(f.read() | ZERO_BITS);
        }
    }
});
