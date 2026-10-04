// original: 0x00904A40 ui_binding_lookup (proposed)
//
// Resolve an input binding index to a handler code, fetch its value, and
// optionally re-resolve by proximity.
//
// `idx` selects a row pointer from the binding table; a 16-bit word at row
// `+2`, times 80, is the byte offset `EB` into the binding-rows block (row
// layout: two threshold floats at `+0`, the score word at `+8`, a signed word
// at `+0x28`, a gate byte at `+0x2C`). A zero gate byte returns the default
// code. Otherwise the signed word is resolved and classified; the class word
// at result `+4` picks a small constant added to the base code (class 1 also
// consults the resolved value itself), and any other class runs a nested
// switch on the resolved value minus 4 (default past 44, thirteen arms
// otherwise, several consulting the selector global). The chosen code goes
// through `out` (unless null) together with one fetch call carrying the mode
// (2, or 3/0 when the mode flag is set) or a per-arm constant. A resolved
// code of -1, or equal to the default, returns the default; a set mode flag
// returns the code. Otherwise two sample floats are read through the sampler
// callee and, when their squared distance from the row thresholds is below
// 1600, the score word is scored: a negative score with confirmation returns
// the alternate code, a positive one with confirmation the alternate plus
// one, anything else the resolved code. Only the low byte of the confirm
// answer is significant. Returns the resolved code.
//
// Original: 0x00904A40 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00904A40(idx: u32, out: u32) -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const CLASSIFY: u32 = 2;
        const FETCH: u32 = 3;
        const SAMPLE: u32 = 4;
        const SCORE: u32 = 5;
        const CONFIRM: u32 = 6;
        const TABLE: u32 = 0x0118_F6F8;
        const ROWS: u32 = 0x0161_5678;
        const WORD_OFF: u32 = 0x28;
        const GATE_OFF: u32 = 0x2C;
        const MODE_FLAG: u32 = 0x0116_09F6;
        const SEL2: u32 = 0x011D_6FD4;
        const BASE: u32 = 0x0103_44A4;
        const DFLT: u32 = 0x0103_4494;
        const ALT: u32 = 0x0103_4498;
        const K2A: u32 = 0x00FE_8C68;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdb(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdw(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn out_call(out: u32, second: u32) {
            unsafe {
                if out != 0 {
                    let mut z: u32 = 0;
                    let r: u32 = lf_checker_rt::callee_cdecl!(FETCH, u32, &mut z as *mut u32 as u32, second);
                    (out as *mut u32).write_unaligned(rd(r));
                }
            }
        }

        let dflt = rd(lf_checker_rt::global::<u32>(DFLT) as u32); let rows = lf_checker_rt::relocated(ROWS);
        let p = rd(lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)));
        let eb = (rdw(p.wrapping_add(2)) as u32).wrapping_mul(80);
        if rdb(eb.wrapping_add(rows).wrapping_add(GATE_OFF)) == 0 {
            return dflt;
        }
        let mode_flag = rdb(lf_checker_rt::global::<u32>(MODE_FLAG) as u32);
        let sel2 = rd(lf_checker_rt::global::<u32>(SEL2) as u32) as i32;
        let mode: u32 = if mode_flag == 0 {
            2
        } else if sel2 == 2 {
            0
        } else {
            3
        };
        let arg = rdw(eb.wrapping_add(rows).wrapping_add(WORD_OFF)) as i16 as i32 as u32;
        let v0: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, arg);
        let cls: u32 = lf_checker_rt::callee_cdecl!(CLASSIFY, u32, v0);
        let class = rd(cls.wrapping_add(4));
        let base = rd(lf_checker_rt::global::<u32>(BASE) as u32);
        let esi: u32 = match class {
            1 => {
                let c = if v0 == 1 {
                    out_call(out, mode);
                    base.wrapping_add(8)
                } else if v0 == 3 {
                    out_call(out, mode);
                    base.wrapping_add(9)
                } else if sel2 == 1 && v0 == 0x18 {
                    out_call(out, mode);
                    base.wrapping_add(0x6A)
                } else {
                    out_call(out, mode);
                    base.wrapping_add(8)
                };
                c
            }
            4 => {
                out_call(out, mode);
                base.wrapping_add(2)
            }
            5 => {
                out_call(out, mode);
                base.wrapping_add(3)
            }
            6 => {
                out_call(out, mode);
                base.wrapping_add(7)
            }
            2 => {
                out_call(out, mode);
                base
            }
            3 => {
                out_call(out, mode);
                base.wrapping_add(1)
            }
            _ => {
                let sw = v0.wrapping_sub(4);
                if sw > 0x2C {
                    out_call(out, 0);
                    return dflt;
                }
                match sw {
                    0 => {
                        out_call(out, mode);
                        base.wrapping_add(5)
                    }
                    1 | 15 => {
                        out_call(out, mode);
                        base.wrapping_add(6)
                    }
                    2 | 14 => {
                        out_call(out, mode);
                        base.wrapping_add(4)
                    }
                    17 => {
                        out_call(out, mode);
                        if sel2 < 1 { base } else { base.wrapping_add(0x6C) }
                    }
                    18 => {
                        out_call(out, mode);
                        if sel2 < 1 { base } else { base.wrapping_add(1) }
                    }
                    20 => {
                        out_call(out, mode);
                        if sel2 < 1 { base } else { base.wrapping_add(0x6A) }
                    }
                    21 => {
                        out_call(out, mode);
                        if sel2 == 0 { base } else { base.wrapping_add(5) }
                    }
                    22 => {
                        out_call(out, mode);
                        if sel2 < 1 { base } else { base.wrapping_add(1) }
                    }
                    23 => {
                        out_call(out, mode);
                        base
                    }
                    24 => {
                        out_call(out, mode);
                        if sel2 < 1 { base } else { base.wrapping_add(0x6B) }
                    }
                    32 => {
                        out_call(out, mode);
                        if sel2 < 2 { base } else { base.wrapping_add(0x77) }
                    }
                    43 => {
                        out_call(out, 0x10);
                        base.wrapping_add(0x0A)
                    }
                    44 => {
                        out_call(out, 7);
                        base.wrapping_add(0x0B)
                    }
                    _ => {
                        out_call(out, 0);
                        return dflt;
                    }
                }
            }
        };
        if esi == 0xFFFF_FFFF || esi == dflt {
            return dflt;
        }
        if mode_flag != 0 {
            return esi;
        }
        let mut s: [u32; 2] = [0, 0];
        
        let _: u32 = lf_checker_rt::callee_cdecl!(SAMPLE, u32, s.as_mut_ptr() as u32);
        let d1 = sub(f32::from_bits(s[0]), rdf(eb.wrapping_add(rows)));
        let d0 = sub(f32::from_bits(s[1]), rdf(eb.wrapping_add(rows).wrapping_add(4)));
        let sum = add(mul(d1, d1), mul(d0, d0));
        let k2 = rdf(lf_checker_rt::global::<u32>(K2A) as u32);
        if k2 > sum {
            let t3 = rd(eb.wrapping_add(rows).wrapping_add(8));
            let sc: u32 = lf_checker_rt::callee_cdecl!(SCORE, u32, t3);
            let sci = sc as i32;
            if sci < 0 {
                let cf: u32 = lf_checker_rt::callee_cdecl!(CONFIRM, u32,);
                if (cf & 0xFF) != 0 {
                    return rd(lf_checker_rt::global::<u32>(ALT) as u32);
                }
                return esi;
            } else if sci > 0 {
                let cf: u32 = lf_checker_rt::callee_cdecl!(CONFIRM, u32,);
                if (cf & 0xFF) != 0 {
                    return rd(lf_checker_rt::global::<u32>(ALT) as u32).wrapping_add(1);
                }
                return esi;
            }
            return esi;
        }
        esi
    }
});
