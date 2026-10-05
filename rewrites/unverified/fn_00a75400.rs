// original: 0x00a75400 ped_task_check_flags (proposed, UNVERIFIED: deferred abs_shadow_unavailable)

/// Multi-stage flag check over a task record, returning 1 on the first
/// failing stage and 0 when every stage passes.
///
/// `s` (cdecl, one stack word) points to the record. The stages: a global
/// gate byte with a gate callee; a table lookup by a scaled index field,
/// requiring a hit whose kind word selects a 12-bit code; a second global
/// gate with two more flag checks; two magnitude probes (an integer callee
/// answer converted to float, absolute value over 100 selects a side path
/// testing a bit and a status callee); two record-pointer confirmation
/// stages; and a final pair of bit tests. Only the low return byte is set.
///
/// The first instruction reads a global gate byte through an absolute
/// address with no relocation entry, so on the checker the original faults
/// before anything else on every trial; two more unrelocated global reads
/// sit deeper. The rewrite reads those words through the relocated image.
/// Callees with no register setup are declared cdecl; sites that load ECX
/// first are declared thiscall (inferred from the setup pattern).
///
/// Original: 0x00a75400 (cdecl, one stack word: record pointer).
lf_checker_rt::export!(cdecl, rw_00a75400(s: u32) -> u32 {
    unsafe {
        const CAL_GATE: u32 = 1;
        const CAL_TABLE: u32 = 2;
        const CAL_FLAG: u32 = 3;
        const CAL_SUB: u32 = 4;
        const CAL_MAG_A: u32 = 5;
        const CAL_MAG_N: u32 = 6;
        const CAL_MAG_B: u32 = 7;
        const CAL_STATUS: u32 = 8;
        const CAL_CONFIRM: u32 = 9;
        const CAL_PAIR: u32 = 10;
        const CAL_FINAL: u32 = 11;
        const ABS128: u32 = 0x7fffffff;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        // Stage 0: global gate byte (unrelocated in the original).
        if unsafe { lf_checker_rt::global::<u8>(0x103ce47).read() } != 0 {
            let g: u32 = lf_checker_rt::callee_cdecl!(CAL_GATE, u32,);
            if (g as u8) == 0 {
                return 1;
            }
        }
        // Stage 1: scaled-index table lookup.
        let idx = rd32(s + 0x2b0).wrapping_add(3);
        let arg = rd32(s + idx.wrapping_mul(3).wrapping_mul(4).wrapping_add(0x2b0));
        let p = lf_checker_rt::callee_cdecl!(CAL_TABLE, u32, arg);
        if p == 0 {
            return 1;
        }
        let code = if rd32(p + 0xc) == 1 { 0x36f3u32 } else { 0x26e3u32 };
        // Stage 2: flag-gated checks over the linked record.
        let c398 = rd32(s + 0x398);
        let e = rd32(s + 0xe6c);
        let mut run_checks = true;
        if rd32(e + 0x264) & 0x800000 == 0 {
            if unsafe { lf_checker_rt::global::<u32>(0x1160c68).read_unaligned() } != 0 {
                let f: u32 = lf_checker_rt::callee_cdecl!(CAL_FLAG, u32,);
                if (f as u8) != 0
                    && unsafe { lf_checker_rt::global::<u8>(0x105c646).read() } == 0
                {
                    run_checks = true;
                } else {
                    run_checks = false;
                }
            }
        }
        if run_checks {
            if code & 0x1000 == 0 {
                if rd32(c398 + 0x28) & 0x3c0 != 0xc0 {
                    return 1;
                }
                if (rd32(c398 + 0x270) >> 4) & 1 == 0 {
                    return 1;
                }
            }
        }
        // Stage 3: two magnitude probes.
        let t = lf_checker_rt::callee_thiscall!(CAL_SUB, u32, s);
        let v1 = lf_checker_rt::callee_cdecl!(
            CAL_MAG_N, u32,
            lf_checker_rt::callee_thiscall!(CAL_MAG_A, u32, t)
        );
        let f1 = f32::from_bits((v1 as i32 as f32).to_bits() & ABS128);
        let side_path = if f1 > 100.0 {
            true
        } else {
            let v2 = lf_checker_rt::callee_cdecl!(
                CAL_MAG_N, u32,
                lf_checker_rt::callee_thiscall!(CAL_MAG_B, u32, t)
            );
            let f2 = f32::from_bits((v2 as i32 as f32).to_bits() & ABS128);
            f2 > 100.0
        };
        if side_path {
            if (rd32(p + 0x20) >> 2) & 1 != 0 {
                let h: u32 = lf_checker_rt::callee_cdecl!(CAL_STATUS, u32,);
                if (h as u8) == 0
                    && unsafe { lf_checker_rt::global::<u8>(0x103ce47).read() } == (h as u8)
                {
                    return 1;
                }
            }
        }
        // Stage 4: linked-record confirmations.
        if rd32(s + 0x398) == 0 {
            return 1;
        }
        let f2: u32 = lf_checker_rt::callee_cdecl!(CAL_FLAG, u32,);
        if (f2 as u8) != 0 {
            let c = rd32(s + 0x398);
            if c != 0 && rd32(c + 0x28) & 0x3c0 == 0xc0 {
                let h2: u32 = lf_checker_rt::callee_cdecl!(CAL_CONFIRM, u32,);
                if (h2 as u8) != 0 {
                    return 1;
                }
            }
        }
        let c = rd32(s + 0x398);
        if c != 0 && rd32(c + 0x28) & 0x3c0 == 0xc0 {
            let a: u32 = lf_checker_rt::callee_cdecl!(CAL_PAIR, u32, c, s);
            if (a as u8) == 0 {
                return 1;
            }
        }
        // Stage 5: final pair call and bit tests.
        let z: u32 = lf_checker_rt::callee_thiscall!(CAL_FINAL, u32, s, rd32(s + 0x398), p);
        if (z as u8) != 0 {
            return 1;
        }
        if rd32(s + 0x29c) & 0x4000 == 0 && (rd32(p + 0x20) & 1) == 0 {
            return 1;
        }
        if rd32(s + 0x264) & 0x800000 != 0 && rd32(p + 0xc) != 1 {
            return 1;
        }
        0
    }
});
