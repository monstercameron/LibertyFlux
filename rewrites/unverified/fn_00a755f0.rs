// original: 0x00a755f0 ped_task_gate_flag_pair (proposed, UNVERIFIED: deferred abs_shadow_unavailable)

/// Gate on paired flag bytes with a middle record-lookup section.
///
/// `arg` (stdcall, one stack word) points to a record; a lookup callee maps
/// it to a wide state block `s`, and the answer is the OR of two halves.
/// The first half scans three groups of flag-byte pairs: within a group the
/// bytes are XORed against a shared base byte and each difference is tested
/// against 0x7f, and the half is set when any group has exactly one side
/// above the bound; if the first two groups are both clear the half falls
/// through to four probe callees on two interior blocks, any positive answer
/// setting it. The middle section resolves a record from `arg+0x2b0` through
/// two more lookups and compares a global mode word (see below); when that
/// selects the second half, two more flag-pair groups are scanned the same
/// way for the low bit. Returns 1 when either half is set, else 0.
///
/// The mode-word read uses an absolute address with no relocation entry, so
/// on the checker the original faults there on every trial that reaches it;
/// the rewrite reads the same word through the relocated image. All calls
/// are direct: one thiscall lookup, two thiscall probes (two sites each),
/// one thiscall resolver (three sites) and one cdecl table call (two sites).
///
/// Original: 0x00a755f0 (stdcall, one stack word: record pointer).
lf_checker_rt::export!(stdcall, rw_00a755f0(arg: u32) -> u32 {
    unsafe {
        const CAL_LOOKUP: u32 = 1;
        const CAL_PROBE_A: u32 = 2;
        const CAL_PROBE_B: u32 = 3;
        const CAL_RESOLVE: u32 = 4;
        const CAL_TABLE: u32 = 5;
        const BOUND: u8 = 0x7f;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// One flag-pair group: true when exactly one of the two base-XORed
        /// differences exceeds the bound.
        #[inline(always)]
        unsafe fn group(s: u32, base: u32, o1: u32, o2: u32) -> bool {
            unsafe {
                let d = rd8(s + base);
                let c = rd8(s + o1) ^ d;
                let a = rd8(s + o2) ^ d;
                (c > BOUND) != (a > BOUND)
            }
        }

        let s = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, arg);
        let mut bl: u8 = 0;
        // Gate pair: when either difference exceeds the bound the scan
        // continues through two more groups and the probe calls, otherwise
        // only the first group below decides.
        let c1 = rd8(s + 0x26fe) ^ rd8(s + 0x26fc);
        let c2 = rd8(s + 0x2c0e) ^ rd8(s + 0x2c0c);
        if c1 > BOUND || c2 > BOUND {
            if group(s, 0x2a4c, 0x2a4e, 0x2a4f) || group(s, 0x2a5c, 0x2a5e, 0x2a5f) {
                bl = 1;
            } else {
                let p1 = lf_checker_rt::callee_thiscall!(CAL_PROBE_A, u32, s + 0x2a88);
                let p2 = lf_checker_rt::callee_thiscall!(CAL_PROBE_B, u32, s + 0x2a88);
                let p3 = lf_checker_rt::callee_thiscall!(CAL_PROBE_A, u32, s + 0x2a78);
                let p4 = lf_checker_rt::callee_thiscall!(CAL_PROBE_B, u32, s + 0x2a78);
                if (p1 as u8) != 0 || (p2 as u8) != 0 || (p3 as u8) != 0 || (p4 as u8) != 0 {
                    bl = 1;
                }
            }
        } else if group(s, 0x2a4c, 0x2a4e, 0x2a4f) {
            bl = 1;
        }
        // Middle record section.
        let edi = arg.wrapping_add(0x2b0);
        let mut cl: u8 = 0;
        if edi != 0 {
            let r1 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, edi);
            if r1 != 0 {
                let r2 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, edi);
                let p = lf_checker_rt::callee_cdecl!(CAL_TABLE, u32, rd32(r2 + 0x18));
                if rd32(p + 0xc) != 1 {
                    // Unrelocated mode word: the original faults here under
                    // the checker (no relocation entry, writable section).
                    let mode = unsafe { lf_checker_rt::global::<u32>(0x11d6fd4).read_unaligned() };
                    if mode != 2 {
                        cl = 1;
                    } else {
                        let r3 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, edi);
                        let q = lf_checker_rt::callee_cdecl!(CAL_TABLE, u32, rd32(r3 + 0x18));
                        if rd32(q + 4) != 8 {
                            cl = 1;
                        }
                    }
                }
            }
        }
        let mut al: u8 = 0;
        if cl != 0 {
            if group(s, 0x2a6c, 0x2a6e, 0x2a6f) || group(s, 0x2bec, 0x2bee, 0x2bef) {
                al = 1;
            }
        }
        ((bl != 0 || al != 0) as u32) & 0xff
    }
});
