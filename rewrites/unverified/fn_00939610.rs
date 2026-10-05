// original: 0x00939610 stream_gate_c (proposed)

/// Decide whether streaming mode may advance, through layered gates.
///
/// Fails (answering 0 in AL) when the first scan is empty, the pool is
/// busy, the floor already covers the current value while the mode is
/// neither 5 nor 2, the override is set for any other mode, the quiet
/// flag is set, or the state is 0 or fails verification. Answers 1 in AL
/// when every gate passes. The upper bits of EAX are always the last
/// call's leftover, reproduced exactly.
lf_checker_rt::export!(cdecl, rw_00939610() -> u32 {
    unsafe {
        const SCAN: u32 = 1;
        const REPORT: u32 = 2;
        const CURRENT: u32 = 3;
        const VERIFY: u32 = 4;
        const POOL: u32 = 0x11D6FA0;
        const FLOOR: u32 = 0x11A4F08;
        const MODE: u32 = 0x11A4EF4;
        const OVERRIDE: u32 = 0x1161518;
        const QUIET: u32 = 0x18B6ED7;
        const STATE: u32 = 0x11A4EF8;
        const LOW_MASK: u32 = 0xFFFF_FF00;
        let g = |va: u32| -> u32 {
            lf_checker_rt::global::<u32>(va).read_unaligned()
        };
        let gb = |va: u32| -> u8 {
            lf_checker_rt::global::<u8>(va).read()
        };
        let t: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32,);
        if (t & 0xFF) == 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 1);
            return r & LOW_MASK;
        }
        if g(POOL) != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 1);
            return r & LOW_MASK;
        }
        let e: u32 = lf_checker_rt::callee_cdecl!(CURRENT, u32,);
        let m = g(MODE);
        if g(FLOOR) < e && m != 5 && m != 2 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 0);
            return (r & LOW_MASK) | 1;
        }
        if gb(OVERRIDE) != 0 && m != 5 && m != 2 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 1);
            return r & LOW_MASK;
        }
        if gb(QUIET) != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 1);
            return r & LOW_MASK;
        }
        let v = g(STATE);
        if v == 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 1);
            return r & LOW_MASK;
        }
        let k: u32 = lf_checker_rt::callee_cdecl!(VERIFY, u32,);
        if (k & 0xFF) != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(REPORT, u32, 1);
            return r & LOW_MASK;
        }
        (k & LOW_MASK) | 1
    }
});
