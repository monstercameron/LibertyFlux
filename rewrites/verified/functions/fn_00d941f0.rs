// original: 0x00d941f0 probe_traverse (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, callee_fastcall, export, global, relocated};
/// Walk a probe list, testing each entry against the query until one hits.
///
/// `a` points to the probe list head, `b` to an entry to skip. The probe
/// id global must be non-zero and differ from `a`. On the first call the
/// init callee (id 0) runs once and its double answer is narrowed to the
/// threshold global. The table callee (id 1) resolves the list; the low
/// nibble of the head's flag word is the entry count (zero means return 0).
/// Each entry: the caller primes its two frame words (low forced to a
/// constant mask, high accumulating a second mask pair), then the generate
/// callee (id 3) fills one word; 0xffff in its
/// high half or 0x6000 in its low bits skips the entry; otherwise the
/// table callee (id 2) resolves the low 12 bits (null skips) and the entry
/// address (table base + high-half*40) is compared against `b` (equal
/// skips) and its tag against the object's tag (differing tags, or an
/// entry equal to the probe id, run the test block; otherwise skip). The
/// test block runs the step callee (id 4) twice, then the probe callee
/// (id 5, answered 2 to proceed); the entry tag is refreshed, the miss
/// vector (fill-defined zeros minus the query point) is measured, and the
/// hit gates run: an exactly-zero length calls back into the traversal
/// (id 7, the stubbed self-call site); otherwise the threshold, the
/// half-unit box, and the depth limit (20) gate in turn, each failure
/// storing the entry and returning 1. A recursive hit returns 1; a miss
/// drops the depth and tries the next entry; exhausted entries return 0.
///
/// The self-call site is stubbed, so multi-depth recursion is unobserved
/// (see `narrowed`); the init callee's constant vector argument is likewise
/// unobserved and its answer flows through the integer channel.
///
/// The norm callee (id 6) takes the miss vector (fill-defined zeros minus
/// the query point) in `ecx`; its three words are snapshotted per call.
/// The step/probe frame pointers are pure outputs and stay zeroed.
///
/// Original: 0x00d941f0 (thiscall, two stack words, al 0/1 result).
lf_checker_rt::export!(thiscall, rw_00d941f0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const ID_INIT: u32 = 0;
        const ID_TAB1: u32 = 1;
        const ID_TAB2: u32 = 2;
        const ID_GEN: u32 = 3;
        const ID_STEP: u32 = 4;
        const ID_PROBE: u32 = 5;
        const ID_NORM: u32 = 6;
        const ID_SELF: u32 = 7;
        const GID: u32 = 0x179fd68;
        const DEPTH: u32 = 0x179fd70;
        const GFLAG: u32 = 0x17a3618;
        const GFLOAT: u32 = 0x17a3614;
        const GVEC0: u32 = 0x17a33c0;
        const GVEC2: u32 = 0x17a33e0;
        const GRES: u32 = 0x179fd6c;
        const GRESV: u32 = 0x17a33f0;
        const PREF_G0: u32 = 0x17a33c0;
        const PREF_G1: u32 = 0x17a33d0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        let idv = rd32(lf_checker_rt::relocated(GID));
        wr32(lf_checker_rt::relocated(DEPTH), rd32(lf_checker_rt::relocated(DEPTH)).wrapping_add(1));
        if a == idv || a == 0 || idv == 0 {
            return 0;
        }
        let flag = rd32(lf_checker_rt::relocated(GFLAG));
        if (flag & 1) == 0 {
            let bits: u64 = lf_checker_rt::callee_cdecl!(ID_INIT, u64,);
            wr32(lf_checker_rt::relocated(GFLAG), flag | 1);
            wr32(lf_checker_rt::relocated(GFLOAT), (f64::from_bits(bits) as f32).to_bits());
        }
        let a4 = rd32(a + 4);
        let table1: u32 = lf_checker_rt::callee_cdecl!(ID_TAB1, u32, (a4 >> 0x11) & 0xfff);
        let field = (rd32(a) >> 0x15) & 0xf;
        if field == 0 {
            return 0;
        }
        let mut edi = 0u32;
        let mut e08 = field.wrapping_sub(1);
        let mut esi = 0u32;
        let mut edx = a;
        let mut frame = [0u32; 2];
        let dummy = [0u32; 8];
        let mut norm = [0u32; 8];
        loop {
            let base = rd32(table1 + 0x64)
                .wrapping_add((((rd32(edx + 4) & 0x1ffff).wrapping_add(e08)) & 0xffffffff).wrapping_mul(8));
            frame[0] = 0xffff0fff;
            frame[1] = (frame[1] | 0x0fffffff) & 0xefffffff;
            let _: u32 = lf_checker_rt::callee_thiscall!(ID_GEN, u32, base, frame.as_mut_ptr() as u32);
            esi = frame[0];
            let w = (esi >> 16) & 0xffff;
            let mut tail = false;
            if w == 0xffff || (esi & 0x6000) != 0 {
                // skip entry
            } else {
                let table2: u32 = lf_checker_rt::callee_cdecl!(ID_TAB2, u32, esi & 0xfff);
                if table2 == 0 {
                    // skip entry
                } else {
                    edx = table2;
                    let entry2 = rd32(table2 + 0x6c).wrapping_add(w.wrapping_mul(40));
                    if entry2 == b {
                        // skip entry
                    } else if rd16(entry2 + 8) != rd16(this + 0xc40) || entry2 == idv {
                        tail = true;
                    } else {
                        // skip entry
                    }
                    if tail {
                        let wbase = rd32(table1 + 0x60);
                        let c1 = ((a4 & 0x1ffff).wrapping_add(e08)) & 0xffffffff;
                        let wd1 = rd16(wbase.wrapping_add(c1.wrapping_mul(2))) as u32;
                        let _: u32 = lf_checker_rt::callee_thiscall!(ID_STEP, u32, table1, wd1, dummy.as_ptr() as u32);
                        let c2 = ((a4 & 0x1ffff).wrapping_add(edi)) & 0xffffffff;
                        let wd2 = rd16(wbase.wrapping_add(c2.wrapping_mul(2))) as u32;
                        let _: u32 = lf_checker_rt::callee_thiscall!(ID_STEP, u32, table1, wd2, dummy.as_ptr() as u32);
                        let tans: u32 = lf_checker_rt::callee_cdecl!(
                            ID_PROBE, u32, lf_checker_rt::relocated(PREF_G0),
                            lf_checker_rt::relocated(PREF_G1),
                            dummy.as_ptr() as u32, dummy.as_ptr() as u32, dummy.as_ptr() as u32);
                        if tans == 2 {
                            wr16(entry2 + 8, rd16(this + 0xc40));
                            let gx = f32::from_bits(rd32(lf_checker_rt::relocated(GVEC0)));
                            let gy = f32::from_bits(rd32(lf_checker_rt::relocated(GVEC0 + 4)));
                            let gz = f32::from_bits(rd32(lf_checker_rt::relocated(GVEC0 + 8)));
                            let mx = fsub(0.0, gx);
                            let my = fsub(0.0, gy);
                            let mz = fsub(0.0, gz);
                            norm[0] = mx.to_bits();
                            norm[1] = my.to_bits();
                            norm[2] = mz.to_bits();
                            e08 = mz.to_bits();
                            let _: u32 = lf_checker_rt::callee_thiscall!(ID_NORM, u32, norm.as_ptr() as u32);
                            let dot = fadd(fadd(fmul(my, my), fmul(mx, mx)), fmul(mz, mz));
                            if dot == 0.0 {
                                let s: u32 = lf_checker_rt::callee_thiscall!(ID_SELF, u32, this, entry2, a);
                                if (s & 0xff) != 0 {
                                    return 1;
                                }
                                wr32(lf_checker_rt::relocated(DEPTH),
                                     rd32(lf_checker_rt::relocated(DEPTH)).wrapping_sub(1));
                            } else {
                                let g20 = f32::from_bits(rd32(lf_checker_rt::relocated(GVEC2)));
                                let g21 = f32::from_bits(rd32(lf_checker_rt::relocated(GVEC2 + 4)));
                                let g22 = f32::from_bits(rd32(lf_checker_rt::relocated(GVEC2 + 8)));
                                let gthr = f32::from_bits(rd32(lf_checker_rt::relocated(GFLOAT)));
                                let x1 = fadd(fadd(fmul(g21, my), fmul(g20, mx)), fmul(mz, g22));
                                if gthr > x1 {
                                    wr32(lf_checker_rt::relocated(GRES), entry2);
                                    wr32(lf_checker_rt::relocated(GRESV), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 4), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 8), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 12), 0);
                                    return 1;
                                }
                                if mz.abs() > 0.5 {
                                    wr32(lf_checker_rt::relocated(GRES), entry2);
                                    wr32(lf_checker_rt::relocated(GRESV), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 4), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 8), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 12), 0);
                                    return 1;
                                }
                                if (rd32(lf_checker_rt::relocated(DEPTH)) as i32) > 20 {
                                    wr32(lf_checker_rt::relocated(GRES), entry2);
                                    wr32(lf_checker_rt::relocated(GRESV), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 4), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 8), 0);
                                    wr32(lf_checker_rt::relocated(GRESV + 12), 0);
                                    return 1;
                                }
                                let s: u32 = lf_checker_rt::callee_thiscall!(ID_SELF, u32, this, entry2, a);
                                if (s & 0xff) != 0 {
                                    return 1;
                                }
                                wr32(lf_checker_rt::relocated(DEPTH),
                                     rd32(lf_checker_rt::relocated(DEPTH)).wrapping_sub(1));
                            }
                        }
                    }
                }
            }
            edx = a;
            e08 = edi;
            edi = edi.wrapping_add(1);
            if edi >= field {
                return 0;
            }
        }
    }
});
