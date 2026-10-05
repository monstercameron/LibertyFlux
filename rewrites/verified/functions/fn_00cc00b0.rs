// original: 0x00CC00B0 task_update_dispatch (proposed)

/// Poll a ped task's lists and advance its state, returning 1 when the task
/// made progress and 0 when it is finished or stalled.
///
/// `this` is the task object (head pointer at `+0x24`, scratch floats at
/// `+0x4..0x10`, flags at `+0x50`) and `arg0` points at a selector whose first
/// word picks the fast path (`0x3b`: walk the first/next list applying a
/// -4.0 rate to every entry whose tag is not `0x3b`, then commit). Otherwise
/// the alternate list is polled, flag `0x40000` is reconciled with the head's
/// `0x400` bit, and the two scratch vectors must both be exactly zero (any
/// nonzero sum, or the head's `0x400` bit, diverts to the failure path, which
/// applies a -8.0 rate through the touch call's return value and returns 0).
/// The main path
/// polls twice more and commits one of two six-argument calls depending on
/// the answers, looks up two slots and finishes through two final calls.
/// Only the low byte of the return value is set.
///
/// Original: 0x00CC00B0 (thiscall, one stack argument, al return).
lf_checker_rt::export!(thiscall, rw_00cc00b0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const FIND_FIRST: u32 = 1;
        const APPLY_RATE: u32 = 2;
        const FIND_NEXT: u32 = 3;
        const COMMIT_A: u32 = 4;
        const FIND_ALT: u32 = 5;
        const TOUCH: u32 = 6;
        const POLL: u32 = 7;
        const RESOLVE: u32 = 8;
        const CHECK: u32 = 9;
        const COMMIT_B: u32 = 10;
        const LOOKUP: u32 = 11;
        const FINISH_A: u32 = 12;
        const FINISH_B: u32 = 13;
        const NEG4: u32 = 0xc080_0000;
        const POS4: u32 = 0x4080_0000;
        const NEG8: u32 = 0xc100_0000;
        const TAG: u32 = 0x3b;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let esi = this;
        let head = rd32(esi.wrapping_add(0x24));
        let mgr = rd32(head.wrapping_add(0x78));
        if rd32(arg0) == TAG {
            let first = lf_checker_rt::callee_thiscall!(FIND_FIRST, u32, mgr, 0u32, 2u32);
            if first != 0 {
                let mut touched = 0u32;
                let mut cur = first;
                loop {
                    if rd32(cur.wrapping_add(0x10)) != TAG {
                        lf_checker_rt::callee_thiscall!(APPLY_RATE, u32, cur, NEG4);
                        touched = 1;
                    }
                    cur = lf_checker_rt::callee_thiscall!(FIND_NEXT, u32, mgr, 0u32, 2u32);
                    if cur == 0 {
                        break;
                    }
                }
                if touched != 0 {
                    lf_checker_rt::callee_thiscall!(COMMIT_A, u32, mgr, TAG, 0x0fu32, POS4, 0xffff_ffffu32);
                    return 1;
                }
            }
        }
        let head2 = rd32(esi.wrapping_add(0x24));
        if rd8(head2.wrapping_add(0x29c)) & 4 == 0 {
            let alt = lf_checker_rt::callee_thiscall!(FIND_ALT, u32, rd32(head2.wrapping_add(0x78)), 0u32, 8u32);
            if alt != 0 {
                lf_checker_rt::callee_thiscall!(APPLY_RATE, u32, alt, NEG4);
            }
        }
        let head3 = rd32(esi.wrapping_add(0x24));
        if rd32(head3.wrapping_add(0x2a0)) & 0x400 != 0 {
            wr32(esi.wrapping_add(0x50), rd32(esi.wrapping_add(0x50)) | 0x40000);
        }
        let touched_val = lf_checker_rt::callee_thiscall!(TOUCH, u32, mgr, 0x52u32, 0u32);
        let fx = rdf(esi.wrapping_add(4));
        let fy = rdf(esi.wrapping_add(8));
        let f = add(mul(fx, fx), mul(fy, fy));
        let gx = rdf(esi.wrapping_add(0x0c));
        let gy = rdf(esi.wrapping_add(0x10));
        let g = add(mul(gx, gx), mul(gy, gy));
        let head4 = rd32(esi.wrapping_add(0x24));
        // The original's flag-test sequence after each unordered float compare is an exact
        // equality test: it proceeds only when the sum is +-0.0, failing on
        // any nonzero value (NaN included), not only on NaN.
        if f != 0.0 || g != 0.0 || rd32(head4.wrapping_add(0x2a0)) & 0x400 != 0 {
            if touched_val != 0 {
                lf_checker_rt::callee_thiscall!(APPLY_RATE, u32, touched_val, NEG8);
            }
            return 0;
        }
        wr32(esi.wrapping_add(0x50), rd32(esi.wrapping_add(0x50)) & 0xfffb_ffff);
        if lf_checker_rt::callee_thiscall!(POLL, u32, head4) & 0xff == 0
            || rd32(esi.wrapping_add(0x50)) & 0x400 != 0
        {
            return rw_00cc00b0_slow(esi, arg0, mgr, head4);
        }
        let rc = head4.wrapping_add(0x2b0);
        let code;
        let a = lf_checker_rt::callee_thiscall!(RESOLVE, u32, rc);
        if a != 0 {
            let b = lf_checker_rt::callee_thiscall!(RESOLVE, u32, rc);
            let q = lf_checker_rt::callee_thiscall!(CHECK, u32, b);
            code = if q & 0xff != 0 { 0x3du32 } else { 0x3cu32 };
        } else {
            code = 0x3c;
        }
        lf_checker_rt::callee_thiscall!(COMMIT_B, u32, mgr, code, 0x50u32, 0u32, 0u32, POS4, 0xffff_ffffu32);
        let u = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, 0x0fu32);
        if u != 0 {
            lf_checker_rt::callee_thiscall!(APPLY_RATE, u32, u, NEG4);
        }
        wr32(esi.wrapping_add(0x40), rd32(esi.wrapping_add(0x3c)));
        lf_checker_rt::callee_thiscall!(FINISH_A, u32, esi);
        lf_checker_rt::callee_cdecl!(FINISH_B, u32, rd32(esi.wrapping_add(0x24)), arg0);
        1
    }
}
);

/// Slow half of rw_00cc00b0 (kept as a plain helper so the export stays
/// readable; the call sequence is unchanged).
unsafe fn rw_00cc00b0_slow(esi: u32, arg0: u32, mgr: u32, head4: u32) -> u32 {
        unsafe {
            const APPLY_RATE: u32 = 2;
            const POLL: u32 = 7;
            const RESOLVE: u32 = 8;
            const CHECK: u32 = 9;
            const COMMIT_B: u32 = 10;
            const LOOKUP: u32 = 11;
            const FINISH_A: u32 = 12;
            const FINISH_B: u32 = 13;
            const NEG4: u32 = 0xc080_0000;
            const POS4: u32 = 0x4080_0000;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn rd8(a: u32) -> u8 {
                unsafe { (a as *const u8).read() }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }

            let t = lf_checker_rt::callee_thiscall!(POLL, u32, head4);
            if t & 0xff == 0 && rd32(esi.wrapping_add(0x50)) & 0x400 != 0 {
                let rc = head4.wrapping_add(0x2b0);
                let code;
                let a = lf_checker_rt::callee_thiscall!(RESOLVE, u32, rc);
                if a != 0 {
                    let b = lf_checker_rt::callee_thiscall!(RESOLVE, u32, rc);
                    let q = lf_checker_rt::callee_thiscall!(CHECK, u32, b);
                    code = if q & 0xff != 0 { 0x3du32 } else { 0x3cu32 };
                } else {
                    code = 0x3c;
                }
                lf_checker_rt::callee_thiscall!(COMMIT_B, u32, mgr, code, 0x51u32, 0u32, 0u32, POS4, 0xffff_ffffu32);
                let u = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, 0x0fu32);
                if u != 0 {
                    lf_checker_rt::callee_thiscall!(APPLY_RATE, u32, u, NEG4);
                }
                wr32(esi.wrapping_add(0x40), rd32(esi.wrapping_add(0x3c)));
            } else {
                let b1 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, 0x51u32);
                let a2 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, 0x50u32);
                if b1 != 0 {
                    if rd8(b1.wrapping_add(0x46)).wrapping_shr(2) & 1 == 0 {
                        lf_checker_rt::callee_thiscall!(FINISH_A, u32, esi);
                        lf_checker_rt::callee_cdecl!(FINISH_B, u32, rd32(esi.wrapping_add(0x24)), arg0);
                        return 1;
                    }
                }
                if a2 == 0 || rd8(a2.wrapping_add(0x46)).wrapping_shr(2) & 1 != 0 {
                    return 0;
                }
            }
            lf_checker_rt::callee_thiscall!(FINISH_A, u32, esi);
            lf_checker_rt::callee_cdecl!(FINISH_B, u32, rd32(esi.wrapping_add(0x24)), arg0);
            1
        }
    }
