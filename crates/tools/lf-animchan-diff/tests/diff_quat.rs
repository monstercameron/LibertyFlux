//! Differential cases, part 5: the raw-quaternion indexed blend.
//!
//! The lifted blend-then-normalize against its verified rewrite on the
//! same generated inputs, comparing every written word, floats bit for
//! bit. The rewrite calls the square-root helper through a callee: the
//! registered stub runs the host square root, the same operation the
//! lift uses, so the root itself is shared and this case proves the
//! blend, the length fold, the zero test and the scaling. A deliberately
//! wrong lift must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_animation::channel::RawQuat;
    use lf_animchan_diff::rewrites::*;
    use lf_animchan_diff::rt::set_callee1;
    use lf_math::Quat;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{F32_EDGE, RawObj, Rng, addr};

    /// Callee-1 stub for the blend: the host square root, the same
    /// operation the lift uses.
    extern "cdecl" fn sqrt_stub(bits: u32) -> f32 {
        f32::from_bits(bits).sqrt()
    }

    /// The blend without the normalize: wrong whenever the length is
    /// neither zero nor one.
    fn wrong_no_normalize(c: &RawQuat, idx: u32, t: f32) -> Quat {
        let s = 1.0 - t;
        let lo = c.keys()[idx as usize];
        let hi = c.keys()[(idx as usize).wrapping_add(1)];
        Quat {
            x: lo.x * s + hi.x * t,
            y: lo.y * s + hi.y * t,
            z: lo.z * s + hi.z * t,
            w: lo.w * s + hi.w * t,
        }
    }

    fn quat_bits(q: &Quat) -> [u32; 4] {
        [q.x.to_bits(), q.y.to_bits(), q.z.to_bits(), q.w.to_bits()]
    }

    /// Random keys mixing edge floats and random bits (NaN payloads and
    /// subnormals included, so the operation order stays pinned).
    fn random_keys(rng: &mut Rng, n: usize) -> Vec<Quat> {
        (0..n)
            .map(|_| {
                let pick = |rng: &mut Rng| {
                    if rng.u32() % 3 == 0 {
                        F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()]
                    } else {
                        rng.f32_bits()
                    }
                };
                Quat {
                    x: pick(rng),
                    y: pick(rng),
                    z: pick(rng),
                    w: pick(rng),
                }
            })
            .collect()
    }

    fn raw_obj(keys_addr: u32, count: u16) -> Box<RawObj> {
        Box::new(RawObj {
            vtable: 0x5555_5555,
            hdr: [0x66, 0x77, 0x88, 0x99],
            keys: keys_addr,
            count,
            gate: 0x33CC,
        })
    }

    #[test]
    fn raw_quat_lerp_normalized_matches() {
        set_callee1(sqrt_stub as usize as u32);
        let mut rng = Rng(0x9A7D);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |keys: &[Quat], idx: u32, t: f32, caught: &mut u32| {
            // Flat words: four per key, as the 16-byte records lie.
            let flat: Box<[f32]> = keys
                .iter()
                .flat_map(|q| [q.x, q.y, q.z, q.w])
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let obj = raw_obj(addr(&flat[0]), keys.len() as u16);
            let mut out = Box::new([0xDEAD_BEEFu32; 4]);
            let r =
                unsafe { fn_00696510::rw_00696510(addr(&*obj), idx, t.to_bits(), addr(&out[0])) };
            assert_eq!(r, 0, "blend has no answer");
            let lift = RawQuat::new(keys.to_vec()).lerp_normalized(idx, t);
            assert_eq!(*out, quat_bits(&lift), "idx {idx} t {t:?}");
            if quat_bits(&wrong_no_normalize(&RawQuat::new(keys.to_vec()), idx, t)) != *out {
                *caught += 1;
            }
            cases += 1;
        };
        // Zero blend: the zero test skips the root and answers as is.
        let zeros = vec![
            Quat {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0
            };
            2
        ];
        for &t in &[0.0f32, 0.5, 1.0, f32::NAN] {
            run(&zeros, 0, t, &mut caught);
        }
        // Unit blends: exact fractions of exact keys.
        let units = vec![
            Quat {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            Quat {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            },
            Quat {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            },
        ];
        for &idx in &[0u32, 1] {
            for &t in &F32_EDGE {
                run(&units, idx, t, &mut caught);
            }
        }
        // Random keys, every pair, edge and random times.
        for _ in 0..40 {
            let n = 2 + (rng.u32() % 3) as usize;
            let keys = random_keys(&mut rng, n);
            for idx in 0..(n as u32 - 1) {
                for _ in 0..4 {
                    let t = if rng.u32() % 2 == 0 {
                        F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()]
                    } else {
                        rng.f32_bits()
                    };
                    run(&keys, idx, t, &mut caught);
                }
            }
        }
        assert!(caught > 0, "wrong quat blend never caught ({cases} cases)");
    }
}
