// original: 0x00A70CC0 CTaskComplexPlayerInCover::vf19

/// Refresh an in-cover task: measure the target offset, then either attach
/// a fresh cover task or take the near branch.
///
/// `this` is the task, `arg` a ped (never null here). A null word at
/// `arg + 0xD68` returns null at once. Otherwise a ducking predicate
/// (thiscall on `arg`) is stored as a byte at `this + 0x19`, and three
/// scaled vectors are built from the position block at `[arg + 0x20]`
/// (`+0x10`/`+0x14`/`+0x18` direction, `+0x30`/`+0x34`/`+0x38` base) with
/// the scale `10.0` and three zero globals: base-plus-scaled-global,
/// its delta back to base, and base-plus-scaled-direction.
///
/// A probe call (cdecl: cover word, `arg`, delta vector, out vector, 0)
/// fills the out vector; the middle lane plus `1.0`, and the scaled
/// vector minus base, feed a second probe (stdcall: out, in), and a third
/// (cdecl: `arg`, second out, flag word, returning its float in `st0`).
/// The planar distance of the out vector from base is compared against
/// `0.7` with an unordered-or-below jump: not-a-number also takes it.
///
/// The near branch allocates a block, builds a cover task on it (thiscall:
/// 0, first-vector pointer, 0), folds the flag byte into the block's byte
/// at `+0x35` (bit 2 set, bit 1 set when the flag is zero, other bits kept)
/// and returns the block.
///
/// The far branch runs an optional second stage (only when byte
/// `arg + 0x218` is clear, byte `arg + 0x219` set and the cover word equals
/// `arg + 0xE70`): the second probe again, then a combiner (cdecl: `arg`,
/// zero slot, out-derived vector, flag-is-zero, 0). A classifier (cdecl:
/// `arg`, distance bits, flag word, cover-bits-are-not-3) yields a handle;
/// a sequence object is built (or null), a slide task is built on a second
/// block unless null (thiscall: `0x2A`, handle, out vector, third-probe
/// float, `[this + 0x18]`-is-zero, flag-is-nonzero-plus-one) and appended,
/// and a cover task is built on a third block and appended the same way
/// (a null third block appends null instead). Returns the sequence.
///
/// Original: 0x00A70CC0 (thiscall, one stack word). Returns a pointer or
/// null in `eax`. All float operations keep the original's operand order.
lf_checker_rt::export!(thiscall, rw_00A70CC0(this: u32, arg: u32) -> u32 {
    unsafe {
        const POS: u32 = 0x20;
        const COVER: u32 = 0xd68;
        const SELF_OFF: u32 = 0xe70;
        const B218: u32 = 0x218;
        const B219: u32 = 0x219;
        const DUCKB: u32 = 0x19;
        const THIS18: u32 = 0x18;
        const FARB: u32 = 0x35;
        const K10: u32 = 0xfe8b08;
        const K1: u32 = 0xfe88e8;
        const THRESH: u32 = 0x103cea8;
        const GX: u32 = 0x128e320;
        const GY: u32 = 0x128e324;
        const GZ: u32 = 0x128e328;
        const POOL_GLOBAL: u32 = 0x167e2a0;
        const C_PROBE1: u32 = 1;
        const C_PROBE2: u32 = 2;
        const C_PROBE3: u32 = 3;
        const C_COMB: u32 = 4;
        const C_CLASS: u32 = 5;
        const C_ALLOC_S: u32 = 6;
        const C_SEQ: u32 = 7;
        const C_ALLOC_SL: u32 = 8;
        const C_SLIDE: u32 = 9;
        const C_DUCK: u32 = 10;
        const C_APPEND: u32 = 11;
        const C_ALLOC_C: u32 = 12;
        const C_COVER: u32 = 13;
        const C_ALLOC_F: u32 = 14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
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

        let d68 = rd32(arg + COVER);
        if d68 == 0 {
            return 0;
        }
        let duck = lf_checker_rt::callee_thiscall!(C_DUCK, u32, arg);
        wr8(this + DUCKB, duck as u8);
        let k10 = rdf(lf_checker_rt::relocated(K10));
        let k1 = rdf(lf_checker_rt::relocated(K1));
        let thresh = rdf(lf_checker_rt::relocated(THRESH));
        let gx = rdf(lf_checker_rt::relocated(GX));
        let gy = rdf(lf_checker_rt::relocated(GY));
        let gz = rdf(lf_checker_rt::relocated(GZ));
        let p = rd32(arg + POS);
        let px = rdf(p + 0x30);
        let py = rdf(p + 0x34);
        let pz = rdf(p + 0x38);
        let s50 = add(px, mul(gx, k10));
        let s4c = add(py, mul(gy, k10));
        let s48 = add(pz, mul(gz, k10));
        let d0 = sub(s50, px);
        let d1 = sub(s4c, py);
        let d2 = sub(s48, pz);
        let t0 = add(px, mul(rdf(p + 0x10), k10));
        let t1 = add(py, mul(rdf(p + 0x14), k10));
        let t2 = add(pz, mul(rdf(p + 0x18), k10));
        let in_b = [d0.to_bits(), d1.to_bits(), d2.to_bits()];
        let mut out_a = [0u32; 3];
        lf_checker_rt::callee_cdecl!(
            C_PROBE1,
            u32,
            d68,
            arg,
            in_b.as_ptr() as u32,
            out_a.as_mut_ptr() as u32,
            0,
        );
        let o0 = f32::from_bits(out_a[0]);
        let o1 = f32::from_bits(out_a[1]);
        let o2 = f32::from_bits(out_a[2]);
        let s58 = add(o2, k1);
        out_a[2] = s58.to_bits();
        let u1 = sub(t1, py);
        let u2 = sub(t0, px);
        let u0 = sub(t2, pz);
        let in_c = [u2.to_bits(), u1.to_bits(), u0.to_bits()];
        let mut out_c = [0u32; 3];
        lf_checker_rt::callee_stdcall!(
            C_PROBE2,
            u32,
            out_c.as_mut_ptr() as u32,
            in_c.as_ptr() as u32,
        );
        let mut flag_w = [0u32; 1];
        let st0: f32 = lf_checker_rt::callee_cdecl!(
            C_PROBE3,
            f32,
            arg,
            out_c.as_ptr() as u32,
            flag_w.as_mut_ptr() as u32,
        );
        let v1 = sub(o0, px);
        let v0 = sub(o1, py);
        let dist = add(mul(v0, v0), mul(v1, v1));
        let dist = dist.sqrt();
        if !(dist >= thresh) {
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let block = lf_checker_rt::callee_thiscall!(C_ALLOC_F, u32, pool);
            let vec_a = [s50.to_bits(), s4c.to_bits(), s48.to_bits()];
            // A null block skips the build; the flag-byte read below then
            // faults exactly as the original's does.
            let got = if block == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    C_COVER,
                    u32,
                    block,
                    0,
                    vec_a.as_ptr() as u32,
                    0,
                )
            };
            let flag_is_zero = (flag_w[0] as u8) == 0;
            let mut cl = ((flag_is_zero as u8) & 1) | 2;
            let old = rd8(got + FARB);
            cl = cl.wrapping_add(cl);
            wr8(got + FARB, cl | (old & 0xfd));
            return got;
        }
        let run2 = rd8(arg + B218) == 0
            && rd8(arg + B219) != 0
            && arg.wrapping_add(SELF_OFF) == d68;
        if run2 {
            // Second stage reads the first stage's out-vector back as input.
            let mut out2 = [0u32; 2];
            lf_checker_rt::callee_stdcall!(
                C_PROBE2,
                u32,
                out2.as_mut_ptr() as u32,
                out_c.as_ptr() as u32,
            );
            let eq = ((flag_w[0] as u8) == 0) as u32;
            let dvec = [o0.to_bits(), o1.to_bits(), sub(s58, k1).to_bits()];
            let zero1 = [0u32; 1];
            lf_checker_rt::callee_cdecl!(
                C_COMB,
                u32,
                arg,
                zero1.as_ptr() as u32,
                dvec.as_ptr() as u32,
                eq,
                0,
            );
        }
        let d68v = rd32(d68);
        let bitflag = (((d68v >> 3) & 3) != 3) as u32;
        let h = lf_checker_rt::callee_cdecl!(
            C_CLASS,
            u32,
            arg,
            dist.to_bits(),
            flag_w[0],
            bitflag,
        );
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let b = lf_checker_rt::callee_thiscall!(C_ALLOC_S, u32, pool);
        let seq = if b == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(C_SEQ, u32, b)
        };
        let ed = lf_checker_rt::callee_thiscall!(C_ALLOC_SL, u32, pool);
        let mut e = 0u32;
        if ed != 0 {
            let flag_nz = (flag_w[0] as u8) != 0;
            let cl = (flag_nz as u32) + 1;
            let al = (rd8(this + THIS18) == 0) as u32;
            e = lf_checker_rt::callee_thiscall!(
                C_SLIDE,
                u32,
                ed,
                0x2a,
                h,
                out_a.as_ptr() as u32,
                st0.to_bits(),
                al,
                cl,
            );
        }
        lf_checker_rt::callee_thiscall!(C_APPEND, u32, seq, e);
        let c = lf_checker_rt::callee_thiscall!(C_ALLOC_C, u32, pool);
        if c == 0 {
            lf_checker_rt::callee_thiscall!(C_APPEND, u32, seq, 0);
            return seq;
        }
        let vec_a = [s50.to_bits(), s4c.to_bits(), s48.to_bits()];
        let c2 = lf_checker_rt::callee_thiscall!(
            C_COVER,
            u32,
            c,
            0,
            vec_a.as_ptr() as u32,
            0,
        );
        lf_checker_rt::callee_thiscall!(C_APPEND, u32, seq, c2);
        seq
    }
});
