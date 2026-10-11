// original: 0x00c425d0 CIntermezzoEventIntro::vf3
/// Check the event's base-ready predicate, the process-wide intro latch, and
/// then the event-ready predicate. Set the one-byte latch on success. Each
/// callee returns a full word, but only its low byte controls the branch; the
/// final AL update preserves that callee's upper 24 EAX bits.
lf_checker_rt::export!(thiscall, rw_00c425d0(this: u32) -> u32 {
    const BASE_READY: u32 = 1;
    const EVENT_READY: u32 = 2;
    const INTRO_LATCH: u32 = 0x016d37cd;
    let base = lf_checker_rt::callee_thiscall!(BASE_READY, u32, this);
    let upper = base & 0xffff_ff00;
    if base as u8 != 0 {
        return upper;
    }
    let latch = unsafe { lf_checker_rt::global::<u8>(INTRO_LATCH).read_unaligned() };
    if latch != 0 {
        return upper;
    }
    let ready = lf_checker_rt::callee_thiscall!(EVENT_READY, u32, this);
    if ready as u8 != 0 {
        unsafe { lf_checker_rt::global::<u8>(INTRO_LATCH).write_unaligned(1); }
        (ready & 0xffff_ff00) | 1
    } else {
        ready & 0xffff_ff00
    }
});
