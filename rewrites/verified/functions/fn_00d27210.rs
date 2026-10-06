// original: 0x00d27210 ped_attack_aim_update (proposed)
/// Update a ped's attack/aim state for one tick (proposed name).
///
/// `a0` is the ped, `a1` a small mode (0 takes the tail path), `a2`/`a3`
/// are integers forwarded to a helper. After three global gates the
/// function hashes a name, runs a setup helper, resolves two aim values
/// through a virtual slot (falling back to a stored pointer when the slot
/// answers null), derives two matrices from a matrix helper, and runs a
/// float block whose outputs feed a chain of buffer helpers (snapshotted
/// pointer by pointer), a fire-weapon call and, when the ped's flag byte
/// allows and the mode is 0, a random-spread tail that advances two
/// writable global words with a multiply-add and fires a final 16-word
/// helper. Returns the final helper's answer, the fire-weapon answer on
/// the guard exits, a gate global on the gate exits, or 0.
///
/// Float rules: ordered IEEE comparisons throughout (NaN takes the
/// not-taken branch); the `lahf; (an instruction of the original); jnp` idiom means "not
/// equal", so the normaliser is `(x != 0.0) ? c / sqrt(x) : 0.0`; the
/// length-squared sums keep the original's operand orders; the min slot
/// keeps `(c > x) ? x : c` (NaN yields `c`); int-to-float widens exactly.
///
/// The first gate's exit returns the caller's entry EAX, which no Rust
/// rewrite can observe; the contract pins that gate shut (see narrowed).
/// Original: 0x00d27210 (stdcall, four stack words).
lf_checker_rt::export!(stdcall, rw_00d27210(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe { inner1(a0, a1, a2, a3) }
});

const G_GATE_RUN: u32 = 0x011f7060;
const G_GATE_A: u32 = 0x012088b4;
const G_GATE_B: u32 = 0x00f1c040;
const G_GATE_C: u32 = 0x01037720;
const G_AIM_K: u32 = 0x00fe8830;
const G_NORM2: u32 = 0x00fe88e8;
const G_MIN_K: u32 = 0x00fe87e4;
const G_STASH_F: u32 = 0x0139c25c;
const G_RNG_LO: u32 = 0x011101a0;
const G_RNG_HI: u32 = 0x011101a4;
const G_RNG_C1: u32 = 0x00fe864c;
const G_RNG_C2: u32 = 0x00fe8a24;
const G_PAIR_D2B: u32 = 0x00fe8ab8;
const G_AB_ARG: u32 = 0x0103eed4;
const STR_JO_A: u32 = 0x00ee18fc;
const STR_JO_B: u32 = 0x00ee190c;
const STR_EV_A: u32 = 0x00ee191c;
const STR_EV_B: u32 = 0x00ee1928;
const OBJ_MAIN: u32 = 0x01394d60;
const PED_TASK: u32 = 0x48;
const PED_F54: u32 = 0x54;
const PED_FLAG: u32 = 0xdc;
const E1_STASH: u32 = 0x1f0;
const VT_AIM: u32 = 0xa0;
const VT_USE: u32 = 0xe0;
const VT_CONF: u32 = 0xec;
const RNG_MUL: u64 = 0x5cdcfaa7;

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

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
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

#[inline(always)]
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    unsafe { rd32(lf_checker_rt::relocated(va)) }
}

#[inline(always)]
unsafe fn gf(va: u32) -> f32 {
    unsafe { f32::from_bits(g32(va)) }
}

#[inline(always)]
fn black(x: f32) -> f32 {
    core::hint::black_box(x)
}

/// Read a word the checker stub may have written through a pointer we
/// passed as a bare u32. A plain read can be folded: the compiler sees a
/// zero-initialised array whose address only escapes as an integer and
/// concludes nothing changed it. Volatile forces the memory read.
#[inline(always)]
unsafe fn vread(s: &[u32], i: usize) -> u32 {
    unsafe { core::ptr::read_volatile(&s[i]) }
}

/// Shared body of the attack/aim update.
unsafe fn inner1(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        // Gate 1's exit returns entry EAX, unobservable here; the
        // contract pins the gate shut, so this is unreachable there.
        if g32(G_GATE_RUN) == 1 {
            return 0;
        }
        let g1 = g32(G_GATE_A);
        if g1 != g32(G_GATE_B) {
            return g1;
        }
        if g32(G_GATE_C) == 0x12 {
            return g1;
        }
        let esi0 = a1.wrapping_add(2).wrapping_add(a0);
        // Pushed immediates carry relocation entries, so the original
        // pushes relocated addresses; match it through relocated().
        let h1: u32 = lf_checker_rt::callee_cdecl!(
            1, u32,
            lf_checker_rt::relocated(if a1 == 0 { STR_JO_A } else { STR_JO_B }),
            0
        );
        let mut b17: [u8; 8] = [0; 8];
        b17[1..5].copy_from_slice(&h1.to_le_bytes());
        let e1: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, lf_checker_rt::relocated(OBJ_MAIN), esi0, h1,
            b17.as_ptr() as u32, 0, 0
        );
        if e1 == 0 {
            return 0;
        }
        let esi = rd32(a0 + PED_TASK);
        let va: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(esi) + VT_AIM) as usize);
        let t1 = va(esi);
        let xa = if t1 == 0 {
            rd32(esi + 0x100)
        } else {
            let r = va(esi);
            let vb: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(r) + VT_USE) as usize);
            vb(r)
        };
        let ret1: u32 =
            lf_checker_rt::callee_cdecl!(5, u32, rd32(xa + 4), a2);
        let t2 = va(esi);
        let xb = if t2 == 0 {
            rd32(esi + 0x100)
        } else {
            let r = va(esi);
            let vb: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(r) + VT_USE) as usize);
            vb(r)
        };
        let ret2: u32 =
            lf_checker_rt::callee_cdecl!(5, u32, rd32(xb + 4), a3);
        let m1a: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi, ret1);
        let m2: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi, ret2);
        // Aim-float block.
        let dx = sub(rdf(m2 + 0x30), rdf(m1a + 0x30));
        let dy = sub(rdf(m2 + 0x34), rdf(m1a + 0x34));
        let dz = sub(rdf(m2 + 0x38), rdf(m1a + 0x38));
        let k0 = gf(G_AIM_K);
        let ex = add(mul(dx, k0), rdf(m1a + 0x30));
        let ez = add(rdf(m1a + 0x38), mul(dz, k0));
        let ey = add(rdf(m1a + 0x34), mul(dy, k0));
        let len2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let kk = if len2 != 0.0 {
            div(gf(G_NORM2), black(len2).sqrt())
        } else {
            0.0
        };
        let b30: [u32; 3] = [mul(dx, kk).to_bits(), mul(kk, dy).to_bits(), mul(kk, dz).to_bits()];
        let mut b70: [u32; 16] = [0; 16];
        b70[0] = ex.to_bits();
        b70[1] = ey.to_bits();
        b70[2] = ez.to_bits();
        let mut bc0: [u32; 15] = [0; 15];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            7, u32, bc0.as_ptr() as u32, b70.as_ptr() as u32,
            b30.as_ptr() as u32, 1
        );
        let b70p4 = b70.as_ptr().wrapping_add(4) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, b70p4, m1a);
        // 3x4 copies: every fourth source and dest word is skipped,
        // leaving the zero fill behind. (The three words past the end
        // are dead stores and omitted.)
        let mut b140: [u32; 12] = [0; 12];
        for i in 0..3 {
            b140[i] = vread(&b70, 4 + i);
            b140[4 + i] = vread(&b70, 8 + i);
            b140[8 + i] = vread(&b70, 12 + i);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            9, u32, b70p4, b140.as_ptr() as u32
        );
        let mut b100: [u32; 12] = [0; 12];
        for i in 0..3 {
            b100[i] = vread(&bc0, i);
            b100[4 + i] = vread(&bc0, 4 + i);
            b100[8 + i] = vread(&bc0, 8 + i);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            10, u32, b100.as_ptr() as u32, b70p4
        );
        let _: u32 =
            lf_checker_rt::callee_thiscall!(11, u32, e1, b100.as_ptr() as u32);
        let f54 = rd32(a0 + PED_F54);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            12, u32, e1, lf_checker_rt::relocated(STR_EV_A), f54
        );
        let vc: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(esi) + VT_CONF) as usize);
        let b40: [u32; 4] = [0; 4];
        let fr: u32 = vc(esi, b40.as_ptr() as u32);
        let fx = rdf(fr);
        let fy = rdf(fr + 4);
        let fz = rdf(fr + 8);
        let l2 = add(add(mul(fx, fx), mul(fy, fy)), mul(fz, fz));
        let x = mul(black(l2).sqrt(), gf(G_MIN_K));
        let cc = gf(G_NORM2);
        let minv = if cc > x { x } else { cc };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            12, u32, e1, lf_checker_rt::relocated(STR_EV_B), minv.to_bits()
        );
        wr32(e1 + E1_STASH, g32(G_STASH_F));
        let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, e1, m1a);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            15, u32, lf_checker_rt::relocated(OBJ_MAIN), e1, esi, 0
        );
        // Callee 16 is gated on a stack byte tested after the id15 call.
        // A probe that scripted the setup helper's buffer byte 7 nonzero
        // fired it on the original only 7 times in 60 trials while the
        // rewrite fired on every scripted trial, so the tested byte is not
        // (only) that buffer byte; the slot is likely reused by a later
        // buffer on some paths. With zeroed stack the original never fires
        // it (0 calls in 1000 trials), so the rewrite never calls it; the
        // branch is listed in `narrowed`.
        let flagb = rd8(a0 + PED_FLAG);
        let albit = ((flagb >> 2) & 1) as u32;
        let r95: u32 = lf_checker_rt::callee_cdecl!(
            17, u32, h1, esi, f54, a1, 0, 0, minv.to_bits(), ret1, ret2, 0,
            albit
        );
        if flagb & 4 == 0 {
            return r95;
        }
        if a1 != 0 {
            return r95;
        }
        // Random-spread tail: advance the two global words with a
        // multiply-add (mul then add/adc), then scale a masked sample.
        let glo = g32(G_RNG_LO);
        let ghi = g32(G_RNG_HI);
        let acc = glo as u64 * RNG_MUL + ghi as u64;
        let nlo = acc as u32;
        wr32(lf_checker_rt::relocated(G_RNG_LO), nlo);
        wr32(lf_checker_rt::relocated(G_RNG_HI), (acc >> 32) as u32);
        let sample = (nlo & 0x7fffff) as f32;
        let rngf = add(mul(mul(sample, gf(G_RNG_C1)), gf(G_RNG_C2)), gf(G_PAIR_D2B));
        let al1: u32 = lf_checker_rt::callee_cdecl!(18, u32,);
        let spread = if al1 & 0xff != 0 {
            gf(G_PAIR_D2B)
        } else {
            let al2: u32 = lf_checker_rt::callee_cdecl!(19, u32,);
            if al2 & 0xff != 0 {
                gf(G_PAIR_D2B)
            } else {
                rngf
            }
        };
        let b40t: [u32; 4] = [0, 0, 0x3f800000, 0];
        let b50t: [u32; 4] = [0, 0x3f800000, 0, 0];
        let b60t: [u32; 3] = [0x3f800000, 0x3e99999a, 0];
        let bf0t: [u32; 4] = [0; 4];
        lf_checker_rt::callee_cdecl!(
            20, u32, 0, 0, 0x201, b40t.as_ptr() as u32, b50t.as_ptr() as u32,
            bf0t.as_ptr() as u32, b60t.as_ptr() as u32, spread.to_bits(), 0,
            g32(G_AB_ARG), 0x40200000, 0, 0, 0xffffffff, 0, 0
        )
    }
}

