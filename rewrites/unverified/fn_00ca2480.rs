// original: 0x00CA2480 ped_task_flag_pack (proposed)

/// Pack three distance-threshold comparisons into a task status byte.
///
/// `this` is the task (status byte at `+0x72`); `src` is a subject
/// record. When the subject kind (`[src+0x28] & 0x3c0`) is `0xc0` and
/// bit 2 of `+0x26c` is set, two probe calls run: a ranged
/// check on the subject's extension (`+0x224`, skipped when null or
/// when adding `0x2e0` wraps to null) whose low byte is kept, and a
/// lookup with argument 0 whose answer, when non-null, either sets a
/// match flag (its link word at `+0xf50` equals `src`) or feeds a
/// compare call with `src`. A control flag is set when the linked
/// record (`+0xb30`) is non-null and reports 1. The state byte being 2
/// only forces the third comparison bit to zero at the end.
///
/// The distance from the subject's anchor (its slot at `+0x20` plus
/// `0x30`, or `src+0x10` when the slot is null) to the shared centre
/// point is then compared against three scaled limits. Bit 2 of `+0x26c`
/// and the byte at `+0x219` select the base scale from shared factors
/// (refined by the control flag and a shared switch); the nonzero probe
/// answer overrides it; a callee answer selects one of two shared counts
/// which, scaled, forms the factor unless shared switches force a
/// constant. Three setcc-style comparisons (unordered counts as
/// below-or-equal, never as above) feed one bit each, combined with the
/// match/control flags and a shared mode byte into bits 3..5 of the
/// status byte, whose other bits are kept.
///
/// Original: 0x00CA2480 (thiscall, one stack argument, no return value).
/// Callees: 0 ranged check, 1 lookup, 2 compare, 3 count select.
/// Float order is the original's.
lf_checker_rt::export!(thiscall, rw_00ca2480(this: u32, src: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0xc0;
        const STATE: u32 = 0xa60;
        const STATE_SKIP: u8 = 2;
        const BITS: u32 = 0x26c;
        const MODE: u32 = 0x219;
        const EXT: u32 = 0x224;
        const EXT_OFF: u32 = 0x2e0;
        const LINKED: u32 = 0xb30;
        const LINKED_FLAG: u32 = 0x1304;
        const ANCHOR: u32 = 0x20;
        const ANCHOR_OFF: u32 = 0x30;
        const STATUS: u32 = 0x72;
        const STATUS_KEEP: u8 = 0xc7;
        const RANGE_ARG: u32 = 0x419;
        const LINK_WORD: u32 = 0xf50;
        const WANT_SWITCH: u32 = 0x12;
        const CX: u32 = 0x0128_e340;
        const CY: u32 = 0x0128_e344;
        const CZ: u32 = 0x0128_e348;
        const UNIT: u32 = 0x0103_f6c8;
        const F_A: u32 = 0x0105_0d0c;
        const F_B: u32 = 0x0105_0d08;
        const F_C: u32 = 0x00fe_8afc;
        const F_D: u32 = 0x0103_f6bc;
        const F_E: u32 = 0x0105_0d04;
        const F_F: u32 = 0x0105_0d10;
        const F_P: u32 = 0x0105_0d14;
        const F_Q: u32 = 0x0105_0d18;
        const SEL_G: u32 = 0x0105_c884;
        const SEL_H: u32 = 0x0105_c888;
        const SW_I: u32 = 0x011f_7060;
        const CMP_J: u32 = 0x0120_88b4;
        const CMP_K: u32 = 0x00f1_c040;
        const SW_L: u32 = 0x0103_7720;
        const SW_M: u32 = 0x011f_66a0;
        const F_O: u32 = 0x00ed_7068;
        const F_N: u32 = 0x00fe_8a24;
        const MODE_G: u32 = 0x0118_dc42;
        const RANGE_CHECK: u32 = 0;
        const LOOKUP: u32 = 1;
        const COMPARE: u32 = 2;
        const COUNT_SEL: u32 = 3;

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
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read()) }
        }
        #[inline(always)]
        unsafe fn gu(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let is_kind = rd32(src.wrapping_add(KIND)) & KIND_MASK == KIND_WANT;
        let mut state_is_skip = false;
        let mut probe_lo = 0u8;
        let mut matched = false;
        let mut cmp_flag = 0u32;
        let mut ctl_flag = 0u32;
        // Frame slot 0x14 starts as the entry ecx with its low byte cleared
        // and is only overwritten when the linked-record check runs.
        let mut slot14 = this & 0xffff_ff00;
        let mut bit_dl = 0u8;
        let mut bit_dh = 0u8;
        if is_kind {
            state_is_skip = rd8(src.wrapping_add(STATE)) == STATE_SKIP;
            let v = rd32(src.wrapping_add(BITS)) >> 2;
            bit_dh = rd8(src.wrapping_add(MODE));
            bit_dl = (v & 1) as u8;
            // The skip jump below tests this bit: an AND between the compare
            // and the jump overwrites the compare's flags.
            if v & 1 != 0 {
                let q = rd32(src.wrapping_add(EXT));
                if q != 0 && q.wrapping_add(EXT_OFF) != 0 {
                    let a: u32 = lf_checker_rt::callee_thiscall!(
                        RANGE_CHECK,
                        u32,
                        q.wrapping_add(EXT_OFF),
                        RANGE_ARG,
                        0,
                    );
                    probe_lo = a as u8;
                }
                if v & 1 != 0 {
                    let r = rd32(src.wrapping_add(LINKED));
                    if r != 0 {
                        if rd32(r.wrapping_add(LINKED_FLAG)) == 1 {
                            ctl_flag = 1;
                            slot14 = 1;
                        } else {
                            slot14 = 0;
                        }
                    }
                }
                let p: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0);
                if p != 0 {
                    if rd32(p.wrapping_add(LINK_WORD)) == src {
                        matched = true;
                    } else {
                        let a2: u32 = lf_checker_rt::callee_thiscall!(COMPARE, u32, p, src);
                        if a2 as u8 != 0 {
                            cmp_flag = 1;
                        }
                    }
                }
            }
        }
        // id3's ecx is slot 0x14 reloaded when the probes ran, else the
        // still-untouched entry value; slot14 already holds either case.
        let ecx3 = slot14;

        let anchor_slot = rd32(src.wrapping_add(ANCHOR));
        let base = if anchor_slot != 0 {
            anchor_slot.wrapping_add(ANCHOR_OFF)
        } else {
            src.wrapping_add(0x10)
        };
        let dx = sub(rdf(base), gf(CX));
        let dy = sub(rdf(base.wrapping_add(4)), gf(CY));
        let dz = sub(rdf(base.wrapping_add(8)), gf(CZ));
        let dist = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz)).sqrt();
        let unit = gf(UNIT);
        let mut scale: f32;
        if bit_dl != 0 {
            if bit_dh != 0 {
                scale = mul(unit, gf(F_A));
            } else {
                let t = gf(F_B);
                scale = mul(unit, t);
                if ctl_flag as u8 != 0 && gu(SW_I) == 1 {
                    scale = mul(add(t, gf(F_C)), gf(F_D));
                }
            }
        } else {
            scale = mul(unit, gf(F_E));
        }
        if probe_lo != 0 {
            scale = mul(unit, gf(F_F));
        }
        let a3: u32 = lf_checker_rt::callee_thiscall!(COUNT_SEL, u32, ecx3);
        let sel = if a3 as u8 != 0 { gu(SEL_H) } else { gu(SEL_G) };
        let mut factor = mul((sel as i32) as f32, gf(F_O));
        let reach_m = gu(SW_I) == 1 || gu(CMP_J) != gu(CMP_K) || gu(SW_L) == WANT_SWITCH;
        if reach_m && gu(SW_M) != 0 {
            factor = gf(F_N);
        }
        let limit = mul(factor, scale);
        let ah = lf_checker_rt::global::<u8>(MODE_G).read();
        let mut dl = (!(dist > limit)) as u8;
        if matched {
            dl = 1;
        } else if cmp_flag != 0 && ctl_flag != 0 {
            dl = 1;
        } else if ah == 0 {
            dl = 1;
        }
        let t1 = mul(mul(unit, gf(F_P)), factor);
        let t2 = mul(mul(unit, gf(F_Q)), factor);
        let mut cl = (!(dist > t1)) as u8;
        if ah == 0 {
            cl = 1;
        }
        let mut al2 = (dist > t2) as u8;
        if state_is_skip {
            al2 = 0;
        }
        let mut packed = ((cl & 1) << 1) | (al2 & 1);
        let old = rd8(this.wrapping_add(STATUS));
        packed = (packed << 1) | (dl & 1);
        packed <<= 3;
        ((this.wrapping_add(STATUS)) as *mut u8).write(packed | (old & STATUS_KEEP));
        0
    }
});
