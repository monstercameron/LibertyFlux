// original: 0x00938850 stream_ready_gate_b (proposed)

/// Decide whether the secondary streaming request may start.
///
/// The mode must exceed 2, the marker must sit below the counter, the
/// class probe must answer 0x17, 0x49 or 0x18 while the family reads 7,
/// a nonzero poll needs the clear flag down, and the scan plus three
/// final checks must all read zero. Answers 1 in AL only then. Early
/// gates answer the entry EAX (pinned to zero by this contract); later
/// ones the last call's leftover.
lf_checker_rt::export!(cdecl, rw_00938850() -> u32 {
    unsafe {
        const COUNT: u32 = 1;
        const CLASS: u32 = 2;
        const POLL: u32 = 3;
        const SCAN: u32 = 4;
        const CHECK_A: u32 = 5;
        const CHECK_B: u32 = 6;
        const CHECK_C: u32 = 7;
        const MODE: u32 = 0x11A4EF4;
        const MARKER: u32 = 0x11A4EFC;
        const FAMILY: u32 = 0x1160C40;
        const CLEAR: u32 = 0x11609F6;
        const LOW_MASK: u32 = 0xFFFF_FF00;
        let g = |va: u32| -> u32 {
            lf_checker_rt::global::<u32>(va).read_unaligned()
        };
        if g(MODE) <= 2 {
            return 0;
        }
        let e: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
        if g(MARKER) >= e {
            return e & LOW_MASK;
        }
        let a: u32 = lf_checker_rt::callee_cdecl!(CLASS, u32, 0, 0xFFFF_FFFF);
        if g(FAMILY) != 7 {
            return a & LOW_MASK;
        }
        if a != 0x17 && a != 0x49 && a != 0x18 {
            return a & LOW_MASK;
        }
        let t: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
        if (t & 0xFF) != 0
            && lf_checker_rt::global::<u8>(CLEAR).read() != 0
        {
            return t & LOW_MASK;
        }
        let s: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32,);
        if (s & 0xFF) == 0 {
            return s & LOW_MASK;
        }
        let x: u32 = lf_checker_rt::callee_cdecl!(CHECK_A, u32,);
        if (x & 0xFF) != 0 {
            return x & LOW_MASK;
        }
        let y: u32 = lf_checker_rt::callee_cdecl!(CHECK_B, u32,);
        if (y & 0xFF) != 0 {
            return y & LOW_MASK;
        }
        let z: u32 = lf_checker_rt::callee_cdecl!(CHECK_C, u32,);
        if (z & 0xFF) != 0 {
            return z & LOW_MASK;
        }
        (z & LOW_MASK) | 1
    }
});
