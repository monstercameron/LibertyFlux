// original: 0x008A6EB0 audio_weight_and_mix_slots (proposed)

/// Select, weight and mix up to five audio slots into the voice object.
///
/// `this` is the audio manager, `obj` the voice object being filled,
/// `array` five per-slot weights, `count` a mode flag and `buffer` five
/// 0xE0-byte slot blocks. The thread's audio state index `t` comes from the
/// TLS slot named by the global `TLS_INDEX`, field `TLS_THREAD_FIELD`.
/// (thiscall, four stack words; the return value is caller-ignored scratch.)
///
/// When `count` is 1 and some weight is nonzero, the early path
/// runs: slot block `k` (the first such weight) is installed into `obj`
/// through callee 2, then copied over `obj[0..0xE0]` by memcpy, which runs
/// natively on the original side and as an identical copy here.
///
/// Otherwise the main path runs. Each slot `i` is eligible when the gate
/// dword at `this + t*4 + GATE_BASE + i*GATE_STRIDE` is nonzero, bit `i` of
/// `obj[OBJ_FLAGS]` is set, and `array[i]` is nonzero. Phase 1 keeps
/// the maximum over eligible slots of `buffer[i][0xA8] + buffer[i][0xA4]`
/// (seeded with `MAX_INIT`, strictly ordered greater-than, NaN never wins)
/// and its index. Phase 2 re-checks eligibility, clamps the same sum from
/// below at `MAX_INIT` (NaN passes through), subtracts the best, and unless the
/// remainder is below `-MAX_INIT` calls the double helper (callee 1) with
/// (10.0, (remainder - MAX_INIT) * STEP); the float-downconverted answer
/// scales `array[i]` into per-slot output `g[i]` and accumulates `acc`.
/// Unless `acc` is above zero or NaN, the function ends here.
///
/// The heavy path zeroes `obj[0..0x60]`, `obj[0x60..0x70]`,
/// `obj[0xB0..0xB8]`, `obj[0xBC]` and copies the winning block's words at
/// 0xA4/0xA8, then for each slot `j` with a nonzero dword at
/// `this + (t + IDX_BASE + j*4)*4`, bit `j` of `obj[OBJ_FLAGS]` set and a
/// nonzero `array[j]` mixes
/// `h = g[j]/acc` of block `j` into `obj[0x60..0x6C]`, `obj[0xBC]` and the
/// truncated integer accumulators `obj[0xB0]`/`obj[0xB4]` (x87 truncate
/// semantics, indefinite on overflow/NaN), and max-merges scaled block
/// words into `obj[0..0x5C]` (a current value wins only on strictly ordered
/// greater-than, so NaN candidates always take).
///
/// Float operation order is the original's, pinned with black_box,
/// including the two reversed products (`obj[0x64]` uses h*word and the
/// second max-lane uses q*word where the others use word*q).
lf_checker_rt::export!(thiscall, rw_008A6EB0(this: u32, obj: u32, array: u32, count: u32, buffer: u32) -> u32 {
    unsafe {
        const TLS_INDEX_GLOBAL: u32 = 0x017ABA14;
        const TLS_THREAD_FIELD: u32 = 0x70;
        const GATE_BASE: u32 = 0x1580;
        const GATE_STRIDE: u32 = 0x10;
        const IDX_BASE: u32 = 0x560;
        const BLOCK: u32 = 0xE0;
        const N_SLOTS: u32 = 5;
        const OBJ_FLAGS: u32 = 0xD3;
        const W_HI: u32 = 0xA8;
        const W_LO: u32 = 0xA4;
        const MAX_INIT: f32 = -100.0;
        const STEP: f32 = 0.05;
        const TEN_BITS: u64 = 0x4024000000000000;
        const F64_LIMIT_BITS: u32 = 0x5F000000; // 2^63 as f32
        const DBL_HELPER: u32 = 1;
        const INSTALL_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        /// Nonzero test matching ucomiss+lahf/test/jp (jp iff ZF==PF iff the
        /// values differ or are unordered; ±0.0 skips, NaN takes).
        #[inline(always)]
        fn nonzero(v: f32) -> bool {
            v != 0.0
        }
        /// Ordered max-merge: the current value wins only on strictly
        /// ordered greater-than (comiss/ja), so a NaN candidate takes.
        #[inline(always)]
        fn max_merge(cur: f32, m: f32) -> f32 {
            if cur > m { cur } else { m }
        }
        /// x87 fistp-qword with truncate control: out-of-range or NaN
        /// stores the indefinite 0x8000000000000000, else truncates.
        fn fistp_trunc(p: f32) -> i64 {
            let lim = f32::from_bits(F64_LIMIT_BITS);
            if p.is_nan() || p >= lim || p < -lim {
                i64::MIN
            } else {
                p as i64
            }
        }
        /// Eligibility gate shared by phases 1 and 2.
        unsafe fn gate_open(this: u32, t: u32, obj: u32, array: u32, i: u32) -> bool {
            unsafe {
                let gate = this
                    .wrapping_add(t.wrapping_mul(4))
                    .wrapping_add(GATE_BASE + i.wrapping_mul(GATE_STRIDE));
                if rd32(gate) == 0 {
                    return false;
                }
                let flags = ((obj.wrapping_add(OBJ_FLAGS)) as *const u8).read();
                if flags & (1u8 << i) == 0 {
                    return false;
                }
                nonzero(rdf(array.wrapping_add(i.wrapping_mul(4))))
            }
        }

        // Pre-scan: count==1 with a nonzero weight takes the early path.
        if count == 1 {
            for k in 0..N_SLOTS {
                if nonzero(rdf(array.wrapping_add(k.wrapping_mul(4)))) {
                    let blk = buffer.wrapping_add(k.wrapping_mul(BLOCK));
                    lf_checker_rt::callee_thiscall!(INSTALL_CALLEE, u32, obj, blk);
                    let mut off = 0u32;
                    while off < BLOCK {
                        wr32(obj.wrapping_add(off), rd32(blk.wrapping_add(off)));
                        off = off.wrapping_add(4);
                    }
                    return 0;
                }
            }
        }

        let slot = rd32(lf_checker_rt::relocated(TLS_INDEX_GLOBAL));
        let thread = lf_checker_rt::tls_slot(slot as usize);
        let t = rd32(thread.wrapping_add(TLS_THREAD_FIELD));

        // Phase 1: maximum weight sum and its index.
        let mut best = MAX_INIT;
        let mut best_idx = 0u32;
        for i in 0..N_SLOTS {
            let gate = this
                .wrapping_add(t.wrapping_mul(4))
                .wrapping_add(GATE_BASE + i.wrapping_mul(GATE_STRIDE));
            if rd32(gate) == 0 {
                continue;
            }
            let flags = ((obj.wrapping_add(OBJ_FLAGS)) as *const u8).read();
            if flags & (1u8 << i) == 0 {
                continue;
            }
            if !nonzero(rdf(array.wrapping_add(i.wrapping_mul(4)))) {
                continue;
            }
            let blk = buffer.wrapping_add(i.wrapping_mul(BLOCK));
            let cand = add(rdf(blk.wrapping_add(W_HI)), rdf(blk.wrapping_add(W_LO)));
            if cand > best {
                best = cand;
                best_idx = i;
            }
        }

        // Phase 2: scaled outputs and accumulator.
        let mut g = [0.0f32; 5];
        let mut acc = 0.0f32;
        for i in 0..N_SLOTS {
            if !gate_open(this, t, obj, array, i) {
                continue;
            }
            let blk = buffer.wrapping_add(i.wrapping_mul(BLOCK));
            let mut sum = add(rdf(blk.wrapping_add(W_HI)), rdf(blk.wrapping_add(W_LO)));
            // comiss+jbe keeps the sum unless strictly below the floor: NaN
            // is unordered, so jbe is taken and NaN passes through unclamped.
            if sum < MAX_INIT {
                sum = MAX_INIT;
            }
            let rem = sub(sum, best);
            let u = sub(rem, MAX_INIT);
            let f = if !(u >= 0.0) {
                0.0f32
            } else {
                let k = mul(rem, STEP);
                let dk = (k as f64).to_bits();
                let ans = lf_checker_rt::callee_cdecl!(
                    DBL_HELPER,
                    u64,
                    (TEN_BITS & 0xFFFF_FFFF) as u32,
                    (TEN_BITS >> 32) as u32,
                    (dk & 0xFFFF_FFFF) as u32,
                    (dk >> 32) as u32
                );
                f64::from_bits(ans) as f32
            };
            let w = rdf(array.wrapping_add(i.wrapping_mul(4)));
            let gi = mul(w, f);
            g[i as usize] = gi;
            acc = add(acc, gi);
        }

        // Unless the accumulator is above zero or NaN, the function ends here.
        if 0.0 >= acc {
            return 0;
        }

        // Heavy path: reset the voice object around the winning block.
        let wblk = buffer.wrapping_add(best_idx.wrapping_mul(BLOCK));
        wr32(obj.wrapping_add(0xBC), 0);
        wr32(obj.wrapping_add(W_LO), rd32(wblk.wrapping_add(W_LO)));
        wr32(obj.wrapping_add(W_HI), rd32(wblk.wrapping_add(W_HI)));
        for off in [0x6Cu32, 0x60, 0x64, 0x68, 0xB0, 0xB4] {
            wr32(obj.wrapping_add(off), 0);
        }
        let mut off = 0u32;
        while off < 0x30 {
            wr32(obj.wrapping_add(off), 0);
            off = off.wrapping_add(4);
        }
        while off < 0x60 {
            wr32(obj.wrapping_add(off), 0);
            wr32(obj.wrapping_add(off.wrapping_add(4)), 0);
            off = off.wrapping_add(8);
        }

        for j in 0..N_SLOTS {
            let idx = t.wrapping_add(IDX_BASE + j.wrapping_mul(4));
            if rd32(this.wrapping_add(idx.wrapping_mul(4))) == 0 {
                continue;
            }
            let flags = ((obj.wrapping_add(OBJ_FLAGS)) as *const u8).read();
            if flags & (1u8 << j) == 0 {
                continue;
            }
            if !nonzero(rdf(array.wrapping_add(j.wrapping_mul(4)))) {
                continue;
            }
            let q = g[j as usize];
            let h = div(q, acc);
            let blk = buffer.wrapping_add(j.wrapping_mul(BLOCK));
            wrf(obj.wrapping_add(0xBC), add(rdf(obj.wrapping_add(0xBC)), mul(rdf(blk.wrapping_add(0xBC)), h)));
            wrf(obj.wrapping_add(0x6C), add(rdf(obj.wrapping_add(0x6C)), mul(rdf(blk.wrapping_add(0x6C)), h)));
            wrf(obj.wrapping_add(0x60), add(rdf(obj.wrapping_add(0x60)), mul(rdf(blk.wrapping_add(0x60)), h)));
            wrf(obj.wrapping_add(0x64), add(rdf(obj.wrapping_add(0x64)), mul(h, rdf(blk.wrapping_add(0x64)))));
            wrf(obj.wrapping_add(0x68), add(rdf(obj.wrapping_add(0x68)), mul(rdf(blk.wrapping_add(0x68)), h)));
            let n1 = rd32(blk.wrapping_add(0xB0));
            let p1 = mul(n1 as f32, h);
            wr32(obj.wrapping_add(0xB0), rd32(obj.wrapping_add(0xB0)).wrapping_add(fistp_trunc(p1) as u32));
            let n2 = rd32(blk.wrapping_add(0xB4));
            let p2 = mul(n2 as f32, h);
            wr32(obj.wrapping_add(0xB4), rd32(obj.wrapping_add(0xB4)).wrapping_add(fistp_trunc(p2) as u32));
            // Max-merge scaled block words into obj[0..0x5C].
            let mut cx = obj.wrapping_add(0x34);
            let mut dx = blk.wrapping_add(0x34);
            let mut e3 = blk.wrapping_add(0x18);
            for a in 0..6u32 {
                let m1 = mul(rdf(e3.wrapping_sub(0x18)), q);
                let c1 = rdf(obj.wrapping_add(a.wrapping_mul(4)));
                wrf(obj.wrapping_add(a.wrapping_mul(4)), max_merge(c1, m1));
                let m2 = mul(q, rdf(e3));
                let c2 = rdf(obj.wrapping_add(a.wrapping_mul(4).wrapping_add(0x18)));
                wrf(obj.wrapping_add(a.wrapping_mul(4).wrapping_add(0x18)), max_merge(c2, m2));
                if a != 2 && a != 3 {
                    let d0 = rdf(dx.wrapping_sub(4));
                    let c0 = rdf(cx.wrapping_sub(4));
                    wrf(cx.wrapping_sub(4), max_merge(c0, mul(d0, q)));
                    let d1 = rdf(dx);
                    let c3 = rdf(cx);
                    wrf(cx, max_merge(c3, mul(d1, q)));
                    let d4 = rdf(dx.wrapping_add(4));
                    let c4 = rdf(cx.wrapping_add(4));
                    wrf(cx.wrapping_add(4), max_merge(c4, mul(d4, q)));
                    cx = cx.wrapping_add(0xC);
                    dx = dx.wrapping_add(0xC);
                }
                e3 = e3.wrapping_add(4);
            }
        }
        0
    }
});
