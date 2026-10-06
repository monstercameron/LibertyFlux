//! Differential cases, part 4: the curve-float decoder.
//!
//! The lifted segment evaluator and sampler against their verified
//! rewrites on the same generated inputs, comparing every written word,
//! floats bit for bit. The sampler's found path calls the segment
//! evaluator through a callee: the registered stub runs the lifted
//! evaluator (proven by its own case below), so this case proves the
//! search, the clamps and the scale. Each case also runs a deliberately
//! wrong lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_animation::channel::frame::truncate_to_i32;
    use lf_animation::channel::{CurveFloat, CurveKey};
    use lf_animchan_diff::rewrites::*;
    use lf_animchan_diff::rt::{set_callee1, set_xmm0_lo};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{CurveKeyRec, CurveObj, F32_EDGE, Rng, addr};

    /// Callee-1 stub for the sampler: runs the lifted segment evaluator
    /// over the callee's coefficient array and writes the output.
    extern "fastcall" fn eval_stub(coeff: u32, order: u32, out: u32, tlbits: u32) -> u32 {
        let t = f32::from_bits(tlbits);
        let n = order.wrapping_add(1) as usize;
        let slice = unsafe { core::slice::from_raw_parts(coeff as *const f32, n) };
        let r = CurveFloat::eval_segment(slice, order, t);
        unsafe {
            (out as *mut u32).write(r.to_bits());
        }
        0
    }

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_animation::channel::CurveFloat;

        /// Negates the time: wrong on every curved segment.
        pub fn eval_neg_t(coeffs: &[f32], order: u32, t: f32) -> f32 {
            CurveFloat::eval_segment(coeffs, order, -t)
        }

        /// Strict search comparison: a target exactly on a key skips it.
        pub fn sample_strict(c: &CurveFloat, t: f32) -> f32 {
            use lf_animation::channel::frame::truncate_to_i32;
            let mut t = t;
            if t < 0.0 {
                t = 0.0;
            }
            let count = c.keys().len() as u32;
            let isi = truncate_to_i32(t).wrapping_add(1);
            let mut ebp: u32 = 0;
            let mut found: Option<usize> = None;
            if count > 1 {
                for (eax, k) in c.keys()[..(count - 1) as usize].iter().enumerate() {
                    if (k.key() as i32) > isi {
                        found = Some(eax);
                        break;
                    }
                    ebp = u32::from(k.key());
                }
            }
            // The only difference from the lift: the strict search above.
            let r = if let Some(fi) = found {
                let seg = &c.keys()[fi];
                let tl = t - ebp as f32;
                CurveFloat::eval_segment(seg.coeff(), u32::from(seg.order()), tl)
            } else {
                let last = &c.keys()[(count - 1) as usize];
                let lk = f32::from(last.key());
                if !(lk > t) {
                    t = lk;
                }
                let tl = t - ebp as f32;
                let d = last.coeff();
                match u32::from(last.order()) {
                    0 => d[0],
                    1 => tl * d[0] + d[1],
                    2 => {
                        let s1 = tl * d[0] + d[1];
                        s1 * tl + d[2]
                    }
                    3 => {
                        let s1 = tl * d[0] + d[1];
                        let s2 = s1 * tl + d[2];
                        s2 * tl + d[3]
                    }
                    typ => {
                        let mut s = d[0];
                        for k in 1..=typ {
                            s = s * tl;
                            s += d[k as usize];
                        }
                        s
                    }
                }
            };
            c.scale() * r + c.bias()
        }
    }

    /// Coefficients mixing edge floats and random bits (NaN payloads
    /// included, so the operand order stays pinned).
    fn coeff_block(rng: &mut Rng, order: u32) -> Box<[f32]> {
        let n = order as usize + 1;
        (0..n)
            .map(|i| {
                if i < F32_EDGE.len() && rng.u32() % 3 == 0 {
                    F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()]
                } else {
                    rng.f32_bits()
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn run_segment(rng: &mut Rng, order: u32, t: f32, caught: &mut u32) {
        let block = coeff_block(rng, order);
        let out = Box::new(0xDEAD_BEEFu32);
        set_xmm0_lo(t.to_bits());
        let r = unsafe { fn_0069BBD0::rw_0069BBD0(addr(&block[0]), order, addr(&*out)) };
        assert_eq!(r, 0, "order {order} has no answer");
        let lift = CurveFloat::eval_segment(&block, order, t);
        assert_eq!(*out, lift.to_bits(), "order {order} t {t:?}");
        if wrong::eval_neg_t(&block, order, t).to_bits() != *out {
            *caught += 1;
        }
    }

    #[test]
    fn curve_eval_segment_matches() {
        let mut rng = Rng(0xE8A1);
        let mut caught = 0;
        let mut cases = 0;
        for order in 0..=8u32 {
            // Edge times on every order: zeros, ones, snap shapes,
            // extremes, infinities, quiet and signalling NaNs.
            for &t in &F32_EDGE {
                run_segment(&mut rng, order, t, &mut caught);
                cases += 1;
            }
            for _ in 0..24 {
                let t = rng.f32_bits();
                run_segment(&mut rng, order, t, &mut caught);
                cases += 1;
            }
        }
        assert!(caught > 0, "wrong segment eval never caught ({cases} cases)");
    }

    /// A 32-bit curve channel plus its lift, kept alive together.
    struct Fixture {
        _recs: Box<[CurveKeyRec]>,
        _coeffs: Vec<Box<[f32]>>,
        obj: Box<CurveObj>,
        lift: CurveFloat,
    }

    /// Builds both forms over the same keys. `orders` selects each key's
    /// degree; coefficients come from `coeff_of` per key index.
    fn fixture(
        keys: &[u16],
        orders: &[u8],
        scale: f32,
        bias: f32,
        coeff_of: &mut dyn FnMut(usize, u8) -> Vec<f32>,
    ) -> Fixture {
        let mut blocks: Vec<Box<[f32]>> = Vec::new();
        let mut recs: Vec<CurveKeyRec> = Vec::new();
        let mut lift_keys: Vec<CurveKey> = Vec::new();
        for (i, (k, o)) in keys.iter().zip(orders.iter()).enumerate() {
            let coeff = coeff_of(i, *o);
            debug_assert!(((*o as usize) + 1) <= coeff.len());
            let block: Box<[f32]> = coeff.into_boxed_slice();
            let ptr = addr(&block[0]);
            blocks.push(block);
            recs.push(CurveKeyRec {
                key: *k,
                order: *o,
                pad: 0,
                coeff: ptr,
            });
            let back: Vec<f32> = blocks[i].to_vec();
            lift_keys.push(CurveKey::new(*k, *o, back));
        }
        let recs = recs.into_boxed_slice();
        let obj = Box::new(CurveObj {
            vtable: 0x4444_4444,
            hdr: [0x11, 0x22, 0x33, 0x44],
            base: addr(&recs[0]),
            count: keys.len() as u16,
            cap: keys.len() as u16,
            scale: scale.to_bits(),
            bias: bias.to_bits(),
        });
        let lift = CurveFloat::new(lift_keys, scale, bias);
        Fixture {
            _recs: recs,
            _coeffs: blocks,
            obj,
            lift,
        }
    }

    /// Times exercising the search: exact keys, key neighbourhoods,
    /// negatives, and the float edges.
    fn sample_times(rng: &mut Rng, keys: &[u16]) -> Vec<f32> {
        let mut v: Vec<f32> = F32_EDGE.to_vec();
        v.extend([-3.5, -1.0, -0.0, 0.5, 2.0]);
        for &k in keys {
            let f = f32::from(k);
            // isi lands exactly on the key for t in [k-1, k).
            v.extend([f - 1.0, f - 0.5, f - 0.001, f, f + 0.001, f + 0.5]);
            // isi lands one past the key.
            v.extend([f + 1.0, f + 1.5]);
        }
        for _ in 0..32 {
            v.push(rng.f32_bits());
            v.push(rng.small(-8, 40));
        }
        v
    }

    #[test]
    fn curve_sample_matches() {
        set_callee1(eval_stub as usize as u32);
        let mut rng = Rng(0x5A4D);
        let mut caught = 0;
        let mut cases = 0;
        // Shapes: single keys of each small order, short ascending runs,
        // repeated keys, and wide orders.
        let shapes: Vec<(Vec<u16>, Vec<u8>)> = vec![
            (vec![0], vec![0]),
            (vec![5], vec![1]),
            (vec![10], vec![2]),
            (vec![3], vec![3]),
            (vec![7], vec![4]),
            (vec![0, 10], vec![0, 0]),
            (vec![0, 10], vec![1, 2]),
            (vec![5, 5], vec![1, 1]),
            (vec![0, 4, 9], vec![2, 1, 3]),
            (vec![1, 2, 3, 4], vec![3, 2, 1, 0]),
            (vec![0, 100, 1000], vec![4, 5, 1]),
            (vec![0xFFFF], vec![2]),
            (vec![0, 0xFFFF], vec![1, 1]),
        ];
        for (keys, orders) in &shapes {
            for &(scale, bias) in &[(1.0f32, 0.0f32), (2.5, -1.0), (0.0, 3.0)] {
                let fx = fixture(keys, orders, scale, bias, &mut |i, o| {
                    // Deterministic per shape: edge-heavy on key 0, dense
                    // after, so both paths meet NaNs and extremes.
                    let n = (o as usize) + 1;
                    (0..n)
                        .map(|j| {
                            if (i + j) % 4 == 0 {
                                F32_EDGE[(i * 7 + j * 3) % F32_EDGE.len()]
                            } else {
                                rng.f32_bits()
                            }
                        })
                        .collect()
                });
                for &t in &sample_times(&mut rng, keys) {
                    let out = Box::new(0xDEAD_BEEFu32);
                    let obj_addr = addr(&*fx.obj);
                    let r = unsafe { fn_0069BD70::rw_0069BD70(obj_addr, t.to_bits(), addr(&*out)) };
                    assert_eq!(r, 0, "sample has no answer");
                    let lift = fx.lift.sample(t);
                    assert_eq!(*out, lift.to_bits(), "t {t:?} keys {keys:?}");
                    // The wrong search only differs on the found path;
                    // run it there (a target exactly on a scanned key).
                    let isi = truncate_to_i32(if t < 0.0 { 0.0 } else { t }).wrapping_add(1);
                    let scanned = &keys[..keys.len() - 1];
                    if keys.len() > 1 && scanned.iter().any(|&k| (k as i32) >= isi) {
                        if wrong::sample_strict(&fx.lift, t).to_bits() != *out {
                            caught += 1;
                        }
                    }
                    cases += 1;
                }
            }
        }
        // Targeted exact-key case: isi sits exactly on the first key, so
        // the strict search skips it and decodes the wrong segment.
        let fx = fixture(&[10, 20], &[0, 0], 1.0, 0.0, &mut |i, _| vec![i as f32 + 1.0]);
        let out = Box::new(0xDEAD_BEEFu32);
        let r = unsafe { fn_0069BD70::rw_0069BD70(addr(&*fx.obj), 9.0f32.to_bits(), addr(&*out)) };
        assert_eq!(r, 0);
        assert_eq!(*out, 1.0f32.to_bits(), "exact key decodes segment 0");
        assert_eq!(fx.lift.sample(9.0).to_bits(), 1.0f32.to_bits());
        if wrong::sample_strict(&fx.lift, 9.0).to_bits() != *out {
            caught += 1;
        }
        cases += 1;
        assert!(caught > 0, "wrong sample never caught ({cases} cases)");
    }
}
