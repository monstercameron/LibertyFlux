// original: 0x009bba40 init_input_dispatch_pair (proposed)

/// Initialise an input dispatch record and run two dispatch probes through it.
///
/// `this` points to the record: word `+0x00` cleared and 0xC0C00000 (-6.0f)
/// stored at `+0x08`, bracketed by two helper calls (id 1, cdecl,
/// `(0xA2F250, 0xA2F240)`; id 2, cdecl, no arguments; both answers ignored).
/// Then builds two probe structs on its stack and passes each to the probe
/// helper (id 3 and id 4, thiscall, `(0, code, 0, 0)` with the probe routine
/// address as `code`), which fills in three words of the first and two of
/// the second; the tag 0x0430260 is stored at the next word of the first
/// struct and in the gap word just below it. Finally passes four
/// double-words copied from the structs by value (eight stack words) to the
/// evaluator (id 5, cdecl) and returns its answer.
///
/// The frame pointers themselves differ between the sides and are never
/// compared; the filled struct words are snapshotted at call time and the
/// evaluator's eight argument words observe every value downstream.
///
/// Edge cases: none take another path; every call fires on every trial.
///
/// Original: thiscall, `this` in ECX, no stack arguments, five callees.
lf_checker_rt::export!(thiscall, rw_009bba40(this: u32) -> u32 {
    unsafe {
        const KNOWN0: u32 = 0x00a2f240;
        const KNOWN1: u32 = 0x00a2f250;
        const PROBE1: u32 = 0x009bb710;
        const PROBE2: u32 = 0x009bb740;
        const TAG: u32 = 0x00430260;
        const NEG6: u32 = 0xc0c00000;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            lf_checker_rt::relocated(KNOWN1),
            lf_checker_rt::relocated(KNOWN0)
        );
        (this as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(NEG6);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        // Frame words f[0..10] stand for the original's frame from 36 bytes
        // below entry ESP: s1 is f[5..8], s2 is f[1..3], f[8] and f[4] take
        // the tag, f[3] stays the zero fill; only relative order matters
        // because the pointers themselves are never compared.
        let mut f = [0u32; 10];
        let s1 = f.as_mut_ptr().wrapping_add(5) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            s1,
            0,
            lf_checker_rt::relocated(PROBE1),
            0,
            0
        );
        f[8] = lf_checker_rt::relocated(TAG);
        let s2 = f.as_mut_ptr().wrapping_add(1) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            s2,
            0,
            lf_checker_rt::relocated(PROBE2),
            0,
            0
        );
        f[4] = lf_checker_rt::relocated(TAG);
        let ans: u32 = lf_checker_rt::callee_cdecl!(
            5, u32, f[5], f[6], f[7], f[8], f[1], f[2], f[3], f[4]
        );
        ans
    }
});
