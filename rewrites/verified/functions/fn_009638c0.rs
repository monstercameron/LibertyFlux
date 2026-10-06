// original: 0x009638c0 ready_gate_poll
/// Poll the four readiness gates for start-up (pure predicate).
///
/// Takes no arguments. Returns 1 only when the armed byte at `0x10376E9`
/// is set, the hold byte at `0x11F7076` is clear, the first probe (no
/// arguments) reports zero, the indexed probe (object pointer in ECX: zero
/// when `0x1036F14` is `-1`, else the table word at `0x11A8808[index]`)
/// reports zero, either the state byte at `0x1037868` is zero or the retry
/// byte at `0x11F6FFA` is 2, and the last two probes report zero and
/// non-zero respectively. Every probe answer is tested by low byte only.
/// Upper return bits pass the last answer through (or the caller's EAX on
/// the first two exits, so the proof pins entry EAX below 256).
lf_checker_rt::export!(cdecl, rw_009638c0() -> u32 {
    unsafe {
        const ARMED: u32 = 0x10376e9;
        const HOLD: u32 = 0x11f7076;
        const INDEX: u32 = 0x1036f14;
        const VTAB: u32 = 0x11a8808;
        const STATE: u32 = 0x1037868;
        const RETRY: u32 = 0x11f6ffa;
        const HI: u32 = 0xffff_ff00;
        if (lf_checker_rt::global::<u8>(ARMED) as *const u8).read() == 0 {
            return 0;
        }
        if (lf_checker_rt::global::<u8>(HOLD) as *const u8).read() != 0 {
            return 0;
        }
        let r1: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if (r1 & 0xff) != 0 {
            return r1 & HI;
        }
        let idx = (lf_checker_rt::global::<u32>(INDEX) as *const u32).read_unaligned();
        let ecx = if idx == 0xffff_ffff {
            0
        } else {
            (lf_checker_rt::relocated(VTAB).wrapping_add(idx.wrapping_mul(4)) as *const u32)
                .read_unaligned()
        };
        let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, ecx);
        if (r2 & 0xff) != 0 {
            return r2 & HI;
        }
        if (lf_checker_rt::global::<u8>(STATE) as *const u8).read() != 0
            && (lf_checker_rt::global::<u8>(RETRY) as *const u8).read() != 2
        {
            return r2 & HI;
        }
        let r3: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        if (r3 & 0xff) != 0 {
            return r3 & HI;
        }
        let r4: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        if (r4 & 0xff) == 0 {
            return r4 & HI;
        }
        (r4 & HI) | 1
    }
});
