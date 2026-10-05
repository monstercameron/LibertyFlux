// original: 0x009EC8D0 ped_pose_solver (proposed)

/// Ped pose solver: samples a probe helper into eight frame slots,
/// optionally refines them through a target lookup and a 3x3 transform,
/// blends toward a goal, converts through two scalar helpers, validates
/// every lane for NaN, then commits through the ped's virtual table.
///
/// `thiscall`: `this` in ECX, no stack arguments, no return value. When
/// `this+0x1BC` is null or its flag word masked with 0x3C0 is not 0x80,
/// the function delegates to virtual slot +0x114 with argument 1 and
/// returns. Otherwise it mirrors the original's 0xB0-byte frame in a local
/// array (offsets identical, so call-time snapshots observe the same
/// words), zero-initialised like the contract's stack fill.
///
/// Stages: the probe call fills eight slots (a float quartet, a dword
/// trio and one dead slot); a non-negative word at +0xD54 selects a global
/// table entry that may trigger a direct notifier; a 12-argument solver
/// call yields a float; the blend triple is seeded from the owner's row
/// and, when the seventh probe slot reads below 1.0, refined by a 3x3
/// matrix-vector transform driven by a second lookup; a cdecl post-scale
/// call refreshes the angle slot; a blend (or, for a positive fourth
/// probe slot, a replay of the stored triple) feeds two xmm0 scalar
/// helpers whose results are fanned out to a 3x3 layout; nine NaN checks
/// divert to a fallback chain (two conditional constructor calls and a
/// matrix install) while the clean path runs a validate/prepare pair; the
/// commit call carries the blended triple (observed via snapshots), a
/// zero word and the distance flag byte; a clear flag runs one last
/// notifier before the final virtual call.
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

const G_SOLVE_TABLE: u32 = 0x01295CD8;
const G_ONE_F: u32 = 0x00FE88E8; // 1.0
const G_ZERO_F: u32 = 0x00FE8628; // 0.0
const G_DIST2: u32 = 0x00FE8B40; // 25.0
const G_SIGN_F: u32 = 0x00FE8FA0; // 0x80000000
const F_ONE_BITS: u32 = 0x3F800000;
const ID9_ARG: u32 = 0x3CA3D70A;

unsafe fn fallback(edi: u32, f: &mut [u32; 44]) {
    unsafe {
        if rd32(edi.wrapping_add(0x20)) == 0 {
            lf_checker_rt::callee_thiscall!(11, u32, edi);
            lf_checker_rt::callee_thiscall!(
                12, u32, edi.wrapping_add(0x10), rd32(edi.wrapping_add(0x20))
            );
        }
        let p = f.as_mut_ptr().add(28) as u32;
        lf_checker_rt::callee_thiscall!(13, u32, p, rd32(edi.wrapping_add(0x20)));
    }
}

lf_checker_rt::export!(thiscall, rw_009EC8D0(this: u32) -> u32 {
    unsafe {
        // Entry gate / tail delegation.
        let head = rd32(this.wrapping_add(0x1BC));
        if head == 0 || rd32(head.wrapping_add(0x28)) & 0x3C0 != 0x80 {
            let vt = rd32(this);
            let tail: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(0x114)) as usize);
            tail(this, 1);
            return 0;
        }
        let edi = head;
        // Frame mirror: word i holds original [E+i*4]. Zero-initialised:
        // the original reads three slots before writing them and the
        // contract defines those reads as 0.
        let mut f = [0u32; 44];
        f[6] = F_ONE_BITS;
        f[7] = F_ONE_BITS;
        f[8] = F_ONE_BITS;
        f[16] = F_ONE_BITS;
        let base = f.as_mut_ptr();
        let fptr = |i: usize| -> u32 { unsafe { base.add(i) as u32 } };
        let r1: u32 = lf_checker_rt::callee_thiscall!(
            1, u32, this,
            fptr(6), fptr(7), fptr(8), fptr(16),
            fptr(24), fptr(22), fptr(14), fptr(21)
        );
        let bl = (r1 & 0xFF) as u8;
        // Target lookup.
        let edx = rd32(this.wrapping_add(0xD54)) as i32;
        let mut ecxv = 0u32;
        if edx > -1 {
            let idx = (rd16(edi.wrapping_add(0x2E)) as i16) as i32;
            let entry = rd32(
                lf_checker_rt::relocated(G_SOLVE_TABLE)
                    .wrapping_add((idx as u32).wrapping_mul(4)),
            );
            let edx3 = (edx as u32).wrapping_add((edx as u32).wrapping_mul(2));
            ecxv = entry
                .wrapping_add(edx3.wrapping_mul(4))
                .wrapping_add(0x350);
        }
        if ecxv != 0 && bl == 0 {
            lf_checker_rt::callee_thiscall!(2, u32, ecxv, edi);
        }
        // Solver call (12 args, float result on the x87 stack). The
        // accumulator is zeroed after the target lookup, so the fourth
        // word argument is always 0 (only its low byte survives, in bl).
        let r3: f32 = lf_checker_rt::callee_thiscall!(
            3, f32, edi, fptr(27), this, rd32(this.wrapping_add(0xD50)),
            0, f[7], f[8], f[16], fptr(24), f[22], f[14], f[21], 1
        );
        f[23] = r3.to_bits();
        // Seed the blend triple from the owner's row.
        let mrow = rd32(this.wrapping_add(0x20));
        f[8] = rd32(mrow.wrapping_add(0x30));
        f[7] = rd32(mrow.wrapping_add(0x34));
        f[14] = rd32(mrow.wrapping_add(0x38));
        f[21] = rd32(this.wrapping_add(0xAA0));
        // Optional 3x3 refinement.
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(G_ONE_F)));
        if one > f32::from_bits(f[6]) {
            let r4: u32 = lf_checker_rt::callee_thiscall!(
                4, u32, rd32(this.wrapping_add(0x224)).wrapping_add(0x44), 0x321
            );
            if r4 != 0 {
                let v3 = f32::from_bits(rd32(r4.wrapping_add(0x40)));
                let v1 = f32::from_bits(rd32(r4.wrapping_add(0x44)));
                let v2 = f32::from_bits(rd32(r4.wrapping_add(0x48)));
                f[21] = rd32(r4.wrapping_add(0x50));
                let mx = rd32(edi.wrapping_add(0x20));
                let m = |o: u32| f32::from_bits(rd32(mx.wrapping_add(o)));
                let mut x0;
                let mut x4 = fmul(m(0x10), v1);
                x0 = fmul(m(0), v3);
                x4 = fadd(x4, x0);
                x0 = fmul(m(0x20), v2);
                x4 = fadd(x4, x0);
                x0 = fmul(m(4), v3);
                x4 = fadd(x4, m(0x30));
                f[8] = x4.to_bits();
                x4 = fmul(m(0x14), v1);
                x4 = fadd(x4, x0);
                x0 = fmul(m(0x24), v2);
                x4 = fadd(x4, x0);
                x0 = fmul(m(8), v3);
                x4 = fadd(x4, m(0x34));
                f[7] = x4.to_bits();
                x4 = fmul(m(0x18), v1);
                x4 = fadd(x4, x0);
                x0 = fmul(m(0x28), v2);
                x4 = fadd(x4, x0);
                x4 = fadd(x4, m(0x38));
                f[14] = x4.to_bits();
            }
        }
        // Post-scale through the cdecl helper (float result on x87). The
        // three words are stored deepest-first, so the last stored word is
        // argument 0.
        let r5: f32 =
            lf_checker_rt::callee_cdecl!(5, f32, f[21], f[23], f[6]);
        f[21] = r5.to_bits();
        // Blend toward the goal when the fourth probe slot is not
        // positive, else replay the stored triple.
        let zero = f32::from_bits(rd32(lf_checker_rt::relocated(G_ZERO_F)));
        let mut x7v: f32;
        let mut x3v: f32;
        let mut x5v: f32;
        let mut x6v: f32;
        let mut x4v: f32;
        let mut d0: f32;
        let mut d1: f32;
        let mut d2: f32;
        if f32::from_bits(f[16]) > zero {
            x6v = f32::from_bits(f[28]);
            d0 = f32::from_bits(f[41]);
            d1 = f32::from_bits(f[40]);
            x4v = f32::from_bits(f[38]);
            x5v = f32::from_bits(f[32]);
            x3v = f32::from_bits(f[30]);
            x7v = f32::from_bits(f[29]);
            f[16] = x6v.to_bits();
            d2 = 0.0;
        } else {
            let x6 = f32::from_bits(f[6]);
            let mut x0 = fsub(one, x6);
            let mut x1 = fmul(f32::from_bits(f[41]), x6);
            let mut x2 = fmul(f32::from_bits(f[42]), x6);
            let mut x4 = fmul(f32::from_bits(f[8]), x0);
            let mut x3 = fmul(f32::from_bits(f[7]), x0);
            let mut x5 = fmul(f32::from_bits(f[14]), x0);
            x0 = fmul(f32::from_bits(f[40]), x6);
            x3 = fadd(x3, x1);
            x5 = fadd(x5, x2);
            x4 = fadd(x4, x0);
            f[43] = f[27];
            f[7] = x3.to_bits();
            f[8] = x4.to_bits();
            f[14] = x5.to_bits();
            f[40] = x4.to_bits();
            f[41] = x3.to_bits();
            f[42] = x5.to_bits();
            // Scalar pair through the xmm0 helpers. Both take the
            // post-scale slot: the blend's product in xmm0 is overwritten
            // by two reloads before the first call.
            let r6: u32 = lf_checker_rt::callee_cdecl!(6, u32, f[21]);
            f[16] = r6;
            let r7: u32 = lf_checker_rt::callee_cdecl!(7, u32, f[21]);
            x7v = f32::from_bits(r7);
            let neg7 = f32::from_bits(
                r7 ^ rd32(lf_checker_rt::relocated(G_SIGN_F)),
            );
            x5v = neg7;
            x3v = 0.0;
            x6v = 0.0;
            x4v = one;
            f[28] = f[16];
            f[33] = f[16];
            f[29] = r7;
            f[30] = 0;
            f[32] = neg7.to_bits();
            f[34] = 0;
            f[36] = 0;
            f[37] = 0;
            f[38] = one.to_bits();
            d0 = f32::from_bits(f[7]);
            d1 = f32::from_bits(f[8]);
            d2 = f32::from_bits(f[14]);
        }
        // Commit the solved angle pair (full float), then merge the bl
        // byte into the frame word (always, even when the distance check
        // is skipped).
        wr32(this.wrapping_add(0xAA4), f[21]);
        wr32(this.wrapping_add(0xAA0), f[21]);
        let mut bl2 = 0u8;
        let eaxd = rd32(this.wrapping_add(0x7B4));
        if eaxd != 0 {
            let s0 = fsub(d0, f32::from_bits(rd32(eaxd.wrapping_add(0x44))));
            let s1 = fsub(d1, f32::from_bits(rd32(eaxd.wrapping_add(0x40))));
            let s2 = fsub(d2, f32::from_bits(rd32(eaxd.wrapping_add(0x48))));
            let dist = fadd(fadd(fmul(s0, s0), fmul(s1, s1)), fmul(s2, s2));
            let lim = f32::from_bits(rd32(lf_checker_rt::relocated(G_DIST2)));
            if dist > lim {
                bl2 = 1;
            }
        }
        f[21] = (f[21] & !0xFF) | bl2 as u32;
        // NaN validation cascade: any NaN diverts to the fallback.
        let c0 = f32::from_bits(f[16]).is_nan();
        let c1 = x7v.is_nan();
        let c2 = x3v.is_nan();
        let c3 = x5v.is_nan();
        let c4 = f32::from_bits(f[33]).is_nan();
        let c5 = f32::from_bits(f[34]).is_nan();
        let c6 = f32::from_bits(f[36]).is_nan();
        let c7 = x6v.is_nan();
        let c8 = x4v.is_nan();
        if !(c0 || c1 || c2 || c3 || c4 || c5 || c6 || c7 || c8) {
            let r8: u32 =
                lf_checker_rt::callee_thiscall!(8, u32, fptr(40), fptr(40));
            if (r8 & 0xFF) == 0 {
                let r9: u32 =
                    lf_checker_rt::callee_thiscall!(9, u32, fptr(28), ID9_ARG);
                if (r9 & 0xFF) == 0 {
                    lf_checker_rt::callee_thiscall!(10, u32, fptr(28));
                }
            } else {
                fallback(edi, &mut f);
            }
        } else {
            fallback(edi, &mut f);
        }
        // Commit through the ped's table.
        let vt = rd32(this);
        let commit: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(4)) as usize);
        commit(this, fptr(28), 0, f[21]);
        if bl2 == 0 {
            lf_checker_rt::callee_thiscall!(15, u32, this, 0);
        }
        lf_checker_rt::callee_thiscall!(16, u32, this);
        0
    }
});
