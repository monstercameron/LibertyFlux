// original: 0x00cffdd0 task_kind_dispatch (proposed, stage 1)

// STAGE 1: entry flag probes, switch dispatch, default case and case 0
// (kind=5) with its shared tail. Kinds reaching other cases are gated out
// by the contract; later stages add one case at a time.

/// Dispatch a ped-task update on the task kind with pre-probed flag bytes.
///
/// `ctx` (ecx) is the task context, `ped` the ped object, `kind` the task
/// kind. Returns an 8-bit status in `al`; the upper 24 bits of `eax` are
/// whatever the last executed step left there and are reproduced exactly.
///
/// Behaviour in order:
/// 1. Probe helper 0 on `ped+0x2b0` six times in three groups, feeding the
///    word at `+0x18` of each second answer to helper 1, and derive three
///    flag bytes from bit 5 / word-equals-6 / bit 6 of helper 1's answers.
///    A fourth flag records whether `ped+0x2b0` equals 7.
/// 2. Subtract 5 from `kind`; an out-of-range index returns 0.
/// 3. Case 0 asks helper 2 about the ped; a zero answer runs the shared
///    tail (a flag-bit test, a threshold compare of `ctx+0x40` against a
///    data constant, an id-5 veto) returning 0 or 1. A nonzero answer asks
///    helper 3, xors two bytes at `+0x26fc/+0x26fe` of its answer, and
///    returns 1 unless the xor is 0x0a or less, in which case helper 4
///    decides.
///
/// Original: 0x00cffdd0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00cffdd0(ctx: u32, ped: u32, kind: u32, _p2: u32, _p3: u32, _p4: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 0x2b0;
        const VETO_FLAG: u32 = 0x270;
        const VETO_BIT: u32 = 0x8000000;
        const SEL_OFF: u32 = 0x34;
        const ID_OFF: u32 = 0x38;
        const THR_OFF: u32 = 0x40;
        const VETO_ID: u32 = 5;
        const CASE_BASE: u32 = 5;
        const CASE_MAX: u32 = 0x11;
        const STATE: u32 = 0x18;
        const BITS: u32 = 0x20;
        const TAG: u32 = 0x4;
        const TAG_WANT: u32 = 6;
        const SEVEN: u32 = 7;
        const XOR_LO: u32 = 0x26fc;
        const XOR_HI: u32 = 0x26fe;
        const XOR_MAX: u32 = 0xa;
        const GLO_A: u32 = 0x00fe88e8;
        const GLO_B: u32 = 0x00fe8ad8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        // Entry probes: three groups. Group 1 is skipped entirely when the
        // state word equals 7; groups 2 and 3 always probe at least once.
        let _d = (rd32(ped.wrapping_add(PROBE)) == SEVEN) as u32;
        let flag_c = if _d == 0 {
            let g0a: u32 = lf_checker_rt::callee_thiscall!(0, u32, ped.wrapping_add(PROBE));
            if g0a != 0 {
                let g0b: u32 = lf_checker_rt::callee_thiscall!(0, u32, ped.wrapping_add(PROBE));
                let h1: u32 = lf_checker_rt::callee_cdecl!(1, u32, rd32(g0b.wrapping_add(STATE)));
                ((rd32(h1.wrapping_add(BITS)) >> 5) & 1) != 0
            } else {
                false
            }
        } else {
            false
        };
        let g2a: u32 = lf_checker_rt::callee_thiscall!(0, u32, ped.wrapping_add(PROBE));
        let _flag_f = if g2a != 0 {
            let g2b: u32 = lf_checker_rt::callee_thiscall!(0, u32, ped.wrapping_add(PROBE));
            let h2: u32 = lf_checker_rt::callee_cdecl!(1, u32, rd32(g2b.wrapping_add(STATE)));
            rd32(h2.wrapping_add(TAG)) == TAG_WANT
        } else {
            false
        };
        let g3a: u32 = lf_checker_rt::callee_thiscall!(0, u32, ped.wrapping_add(PROBE));
        let _flag_e = if g3a != 0 {
            let g3b: u32 = lf_checker_rt::callee_thiscall!(0, u32, ped.wrapping_add(PROBE));
            let h3: u32 = lf_checker_rt::callee_cdecl!(1, u32, rd32(g3b.wrapping_add(STATE)));
            ((rd32(h3.wrapping_add(BITS)) >> 6) & 1) != 0
        } else {
            false
        };
        let _ = (flag_c, _flag_f, _flag_e);

        // Dispatch.
        let idx = kind.wrapping_sub(CASE_BASE);
        if idx > CASE_MAX {
            return idx & 0xffff_ff00;
        }
        if idx != 0 {
            // Later stages; unreachable under the stage-1 contract gates.
            return idx & 0xffff_ff00;
        }
        // Case 0.
        let r0: u32 = lf_checker_rt::callee_thiscall!(2, u32, ped);
        if r0 & 0xff == 0 {
            // Shared tail.
            if rd32(ped.wrapping_add(VETO_FLAG)) & VETO_BIT != 0 {
                return r0 & 0xffff_ff00;
            }
            let glo = if rd8(ctx.wrapping_add(SEL_OFF)) == 0 {
                rdf(lf_checker_rt::relocated(GLO_A))
            } else {
                rdf(lf_checker_rt::relocated(GLO_B))
            };
            if !(rdf(ctx.wrapping_add(THR_OFF)) > glo) {
                return r0 & 0xffff_ff00;
            }
            if rd32(ctx.wrapping_add(ID_OFF)) == VETO_ID {
                return r0 & 0xffff_ff00;
            }
            return (r0 & 0xffff_ff00) | 1;
        }
        let r1: u32 = lf_checker_rt::callee_thiscall!(3, u32, ped);
        let x = rd8(r1.wrapping_add(XOR_HI)) ^ rd8(r1.wrapping_add(XOR_LO));
        if x > XOR_MAX as u8 {
            return (r1 & 0xffff_ff00) | 1;
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(4, u32, r1);
        if r2 & 0xff == 0 {
            return r2 & 0xffff_ff00;
        }
        (r2 & 0xffff_ff00) | 1
    }
});
