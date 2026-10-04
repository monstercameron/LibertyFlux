// original: 0x00d896a0 proposed_steer_resolve_store
// Steering resolve: probes two objects through their virtual slots,
// resolves a list entry (appending a fresh record when the slot is
// empty), transforms two sample blocks, and folds the results into two
// wrapped angle outputs. `obj` selects the entry through a word index
// into the entry table; `obj2` supplies the second sample block.
// Returns a status word determined by the exit path (a probed pointer,
// the resolver token, -1, the token base, or one of the outputs).

use core::f32::consts::TAU;
use lf_checker_rt::{callee_cdecl, export, global};

export!(cdecl, rw_d896a0(obj: u32, obj2: u32, outB: u32, outA: u32) -> u32 {
    unsafe {
        use core::f32::consts::PI;
        let vt = (obj as *const u32).read();
        let f64o: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + 0x64) as *const u32).read() as usize);
        let pA = f64o(obj);
        let f60o: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + 0x60) as *const u32).read() as usize);
        let pB = f60o(obj);
        let gap = ((pA + 8) as *const f32).read() - ((pB + 8) as *const f32).read();
        if 1.5f32 > gap {
            return pB;
        }
        let w = ((obj + 0x2e) as *const i16).read() as isize;
        let t2 = global::<u32>(0x1295cd8);
        let ex = t2.offset(w).read();
        let q = (ex.wrapping_add(8)) as *const u32;
        let r = if q.read() != 0 {
            (q.read().wrapping_add(0xec) as *const u32).read()
        } else {
            let q2 = (ex.wrapping_add(4) as *const u32).read();
            if q2 == 0 {
                return 0;
            }
            (q2.wrapping_add(0xc) as *const u32).read()
        };
        if r == 0 {
            return 0;
        }
        let mut w50 = (ex.wrapping_add(0x50) as *const i16).read();
        // id1 scratch: flag byte, one word, four words (separate locals;
        // the original overlaps flag/word in its frame, unobservably).
        let mut w10 = [0u32; 1];
        let mut fbuf = [0u32; 4];
        let mut flagb = [0u8; 4];
        if w50 == -2 {
            callee_cdecl!(4, u32, r, fbuf.as_mut_ptr() as u32,
                w10.as_mut_ptr() as u32, flagb.as_mut_ptr() as u32);
            if flagb[0] == 0 {
                (ex.wrapping_add(0x50) as *mut i16).write(-1);
                return 0xFFFFFFFF;
            }
            let g = callee_cdecl!(5, u32,);
            let old4 = ((g + 4) as *const u32).read();
            let g2 = callee_cdecl!(5, u32,);
            let lim = (g2 as *const u32).read();
            if (old4 as i32) >= (lim as i32) {
                return g2;
            }
            let g3 = callee_cdecl!(5, u32,);
            let c4 = (((g3 + 4) as *const u32).read());
            let base = ((g3 + 8) as *const u32).read();
            let slot = base.wrapping_add(c4.wrapping_shl(5));
            ((g3 + 4) as *mut u32).write(c4.wrapping_add(1));
            let g4 = callee_cdecl!(5, u32,);
            let base4 = ((g4 + 8) as *const u32).read();
            let back = ((slot.wrapping_sub(base4) as i32) >> 5) as u16;
            (ex.wrapping_add(0x50) as *mut u16).write(back);
            w50 = back as i16;
            (slot as *mut u32).write(fbuf[0]);
            ((slot + 4) as *mut u32).write(fbuf[1]);
            ((slot + 8) as *mut u32).write(fbuf[2]);
            ((slot + 0xc) as *mut u32).write(fbuf[3]);
            ((slot + 0x10) as *mut u32).write(w10[0]);
        }
        if w50 < 0 {
            return r;
        }
        let g5 = callee_cdecl!(5, u32,);
        let base5 = ((g5 + 8) as *const u32).read();
        let edi_new = base5.wrapping_add(((w50 as i32) << 5) as u32);
        let m1 = ((obj + 0x20) as *const u32).read();
        let mut o1 = [0u32; 2];
        callee_cdecl!(6, u32, o1.as_mut_ptr() as u32, m1, edi_new);
        let vt2 = (obj2 as *const u32).read();
        let f64s: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt2 + 0x64) as *const u32).read() as usize);
        let pC = f64s(obj2);
        let x = ((pC + 4) as *const f32).read() * 0.9f32;
        let m2 = ((obj2 + 0x20) as *const u32).read();
        let b2 = [0u32, x.to_bits(), 0u32];
        let mut o2 = [0u32; 2];
        callee_cdecl!(7, u32, o2.as_mut_ptr() as u32, m2, b2.as_ptr() as u32);
        let d0 = f32::from_bits(o1[1]) - f32::from_bits(o2[1]);
        let d1 = f32::from_bits(o1[0]) - f32::from_bits(o2[0]);
        // id8 answers in ST0; Rust reads it as an f64 return, and `as f32`
        // converts exactly like the original's fstp-to-float.
        let fans: f64 = callee_cdecl!(8, f64, d1.to_bits(), d0.to_bits());
        let ff = fans as f32;
        let e1 = ((edi_new + 0x10) as *const f32).read() + 0.2f32;
        let pC2 = f64s(obj2);
        let denom = (d0 * d0 + d1 * d1).sqrt();
        let scaled = (pC2 as *const f32).read() * 2.4f32 + e1;
        let v = scaled / denom;
        let mut x1 = ff - (outA as *const f32).read();
        while x1 < -PI {
            x1 += TAU;
        }
        while x1 > PI {
            x1 -= TAU;
        }
        if 0.0 > x1 {
            x1 = -x1;
        }
        let v2 = v * 0.5f32;
        // NOTE: no early exit here: when v2 <= x1 the original skips the
        // outA store and falls into the outB block below.
        if v2 > x1 {
            let mut xa = ff - v2;
            (outA as *mut f32).write(xa);
            while xa < -PI {
                xa += TAU;
            }
            (outA as *mut f32).write(xa);
        }
        let mut x2 = ff - (outB as *const f32).read();
        while x2 < -PI {
            x2 += TAU;
        }
        while x2 > PI {
            x2 -= TAU;
        }
        if 0.0 > x2 {
            x2 = -x2;
        }
        if !(v2 > x2) {
            return outB;
        }
        let mut xb = v2 + ff;
        (outB as *mut f32).write(xb);
        while xb > PI {
            xb -= TAU;
        }
        (outB as *mut f32).write(xb);
        outB
    }
});
