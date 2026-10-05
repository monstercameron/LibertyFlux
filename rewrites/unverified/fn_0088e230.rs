// original: 0x0088E230 rage::audVoiceSoft::vf10

/// Current playback position of this software voice, or -1 when the voice
/// is not playing.
///
/// `this` points to the voice. Its own state gate (slot `+0x18` of its
/// table) runs first; when it reports false the answer is -1. Otherwise
/// the child at `+0x130` is polled (callee 2), and the answer is that
/// answer plus the frequency word at `+0x124` scaled from 16.16 fixed
/// point, halved once more and biased by the word at `+0x128`, converted
/// through the position helper (callee 3) together with the base at `+0xc`.
///
/// Original: 0x0088E230 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088E230(this: u32) -> u32 {
    unsafe {
        const OWN_GATE: u32 = 0x18;
        const CHILD: u32 = 0x130;
        const FREQ: u32 = 0x124;
        const BASE: u32 = 0x0c;
        const BIAS: u32 = 0x128;
        const POLL_CHILD: u32 = 2;
        const POS_HELPER: u32 = 3;

        let table = (this as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((table + OWN_GATE) as *const u32).read_unaligned() as usize
        );
        if gate(this) as u8 == 0 {
            return 0xffff_ffff;
        }
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let ans = lf_checker_rt::callee_thiscall!(POLL_CHILD, u32, child);
        let freq = ((this + FREQ) as *const u32).read_unaligned();
        let base = ((this + BASE) as *const u32).read_unaligned();
        let bias = ((this + BIAS) as *const u32).read_unaligned();
        let scaled =
            (((freq >> 1) << 17).wrapping_add(ans) >> 1).wrapping_add(bias);
        lf_checker_rt::callee_cdecl!(POS_HELPER, u32, scaled, base)
    }
});
