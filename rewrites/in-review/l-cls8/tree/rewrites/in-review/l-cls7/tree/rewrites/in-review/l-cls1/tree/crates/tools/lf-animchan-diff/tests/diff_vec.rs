//! Differential cases, part 2: vector, quaternion and quantized channels.
//!
//! Same contract as `diff.rs`: return value and every written byte, floats
//! bit for bit, plus a deliberately wrong lift caught per method. 32-bit
//! target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe; out-boxes mutate
// through those addresses, so their bindings stay non-mut.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_animchan_diff::rewrites::*;
    use lf_animation::channel::frame::AnimChannel;
    use lf_animation::channel::{QuantizeFloat, RawVec3, StaticQuat, StaticVec3};
    use lf_math::{Quat, Vec4};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{F32_EDGE, Rng, RawObj, SlotObj, U32_EDGE, addr};

    mod wrong {
        use lf_animation::channel::RawVec3;
        use lf_math::Vec4;

        /// Blends toward the wrong end: `1 - t` instead of `t`.
        pub fn vec_lerp_flip_t(lo: Vec4, hi: Vec4, t: f32) -> [f32; 3] {
            let t = 1.0 - t;
            [
                (hi.x - lo.x) * t + lo.x,
                (hi.y - lo.y) * t + lo.y,
                (hi.z - lo.z) * t + lo.z,
            ]
        }

        /// Strict snap-up comparison, mirroring the float wrong version.
        pub fn vec_sample_strict(c: &RawVec3, frame: f32, out: &mut Vec4) {
            use lf_animation::channel::frame::{SNAP_HI, SNAP_LO, clamp_index, key_below};
            use lf_animation::channel::frame::AnimChannel;
            let (index, frac) = key_below(frame);
            let last = c.key_count() as i32 - 1;
            if frac < SNAP_HI {
                if !(frac > SNAP_LO) {
                    *out = c.keys()[clamp_index(index, last) as usize];
                } else {
                    let j = clamp_index(index.wrapping_add(1), last);
                    let i = clamp_index(index, last);
                    let a = c.keys()[i as usize];
                    let b = c.keys()[j as usize];
                    out.x = (b.x - a.x) * frac + a.x;
                    out.y = (b.y - a.y) * frac + a.y;
                    out.z = (b.z - a.z) * frac + a.z;
                }
            } else {
                let k = clamp_index(index.wrapping_add(1), last);
                *out = c.keys()[k as usize];
            }
        }

        /// Sum of squares instead of the ordered maximum chain.
        pub fn vec3_uniform_sum(
            c: &mut StaticVec3Like,
            records: &[Vec4],
            count: i32,
            tol: f32,
        ) -> bool {
            let first = records[0];
            *c = StaticVec3Like(first);
            if count <= 1 {
                return true;
            }
            let limit = tol * tol;
            let mut k: i32 = 1;
            while k < count {
                let r = records[k as usize];
                let dx = first.x - r.x;
                let dy = first.y - r.y;
                let dz = first.z - r.z;
                if dx * dx + dy * dy + dz * dz > limit {
                    return false;
                }
                k += 1;
            }
            true
        }

        /// Stand-in for the wrong-version value slot.
        pub struct StaticVec3Like(pub Vec4);

        pub fn quant_f64_math(scale: f32, bias: f32, a: u64, b: u64, t: f32) -> f64 {
            let v0 = f64::from(a as f32).mul_add(f64::from(scale), f64::from(bias));
            let v1 = f64::from(b as f32).mul_add(f64::from(scale), f64::from(bias));
            (v1 - v0).mul_add(f64::from(t), v0)
        }
    }

    fn raw_obj(keys_addr: u32, count: u16) -> Box<RawObj> {
        Box::new(RawObj {
            vtable: 0x3333_3333,
            hdr: [0x55, 0x66, 0x77, 0x88],
            keys: keys_addr,
            count,
            gate: 0xAA55,
        })
    }

    fn slot_obj(slot_addr: u32) -> Box<SlotObj> {
        Box::new(SlotObj {
            vtable: 0x4444_4444,
            hdr: [0x99, 0xAA, 0xBB, 0xCC],
            slot: slot_addr,
        })
    }

    fn words_to_vec4(w: [u32; 4]) -> Vec4 {
        Vec4::from_array(w.map(f32::from_bits))
    }

    #[test]
    fn raw_vec3_indexed_lerp_matches() {
        let mut rng = Rng(0x64A0);
        let mut caught = 0;
        for len in [2usize, 3, 6] {
            for trial in 0..60u32 {
                let words: Vec<[u32; 4]> = (0..len)
                    .map(|_| [rng.u32(), rng.u32(), rng.u32(), rng.u32()])
                    .collect();
                let boxed: Box<[[u32; 4]]> = words.clone().into_boxed_slice();
                let obj = raw_obj(addr(&boxed[0]), len as u16);
                let keys: Vec<Vec4> = words.iter().map(|&w| words_to_vec4(w)).collect();
                let lift = RawVec3::new(keys);
                let idx = if trial % 2 == 0 {
                    0
                } else {
                    rng.u32() % (len as u32 - 1)
                };
                let t = if trial < 24 {
                    F32_EDGE[(trial as usize) % F32_EDGE.len()]
                } else {
                    rng.f32_bits()
                };
                let out = Box::new([0xAA55_AA55u32; 4]);
                let r = unsafe {
                    fn_006964A0::rw_006964A0(addr(&*obj), idx, t.to_bits(), addr(&out[0]))
                };
                let l = lift.lerp_at(idx, t);
                assert_eq!(r, 0, "sample_indexed answers 0");
                assert_eq!(out[0], l.x.to_bits(), "x len {len} idx {idx} t {t}");
                assert_eq!(out[1], l.y.to_bits(), "y");
                assert_eq!(out[2], l.z.to_bits(), "z");
                assert_eq!(out[3], 0xAA55_AA55, "fourth word untouched");
                let lo = lift.keys()[idx as usize];
                let hi = lift.keys()[idx as usize + 1];
                let w = wrong::vec_lerp_flip_t(lo, hi, t);
                if w.map(f32::to_bits) != [out[0], out[1], out[2]] {
                    caught += 1;
                }
                let _ = boxed;
            }
        }
        assert!(caught > 0, "wrong lerp never caught");
    }

    #[test]
    fn raw_vec3_sample_matches() {
        let mut rng = Rng(0xAAA0);
        let mut caught = 0;
        for len in [1usize, 2, 3, 6] {
            // Distinct keys with distinct padding so paths differ observably.
            let words: Vec<[u32; 4]> = (0..len)
                .map(|k| {
                    [
                        (10.0 + k as f32 * 7.0).to_bits(),
                        (20.0 + k as f32 * 3.0).to_bits(),
                        (30.0 + k as f32).to_bits(),
                        0xCA00_0000 + k as u32,
                    ]
                })
                .collect();
            let boxed: Box<[[u32; 4]]> = words.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), len as u16);
            let keys: Vec<Vec4> = words.iter().map(|&w| words_to_vec4(w)).collect();
            let lift = RawVec3::new(keys);
            let mut frames: Vec<f32> = F32_EDGE.to_vec();
            for _ in 0..40 {
                frames.push(rng.frame(len as f32 - 1.0));
            }
            for k in 0..len {
                frames.push(k as f32 - 0.0005);
                frames.push(k as f32 + 0.0005);
                frames.push(k as f32 - 0.9995);
                frames.push(k as f32 + 0.9995);
            }
            for f in frames {
                let out = Box::new([0xBAAD_F00Du32; 4]);
                let out_addr = addr(&out[0]);
                let r = unsafe { fn_0069AAA0::rw_0069aaa0(addr(&*obj), f, out_addr) };
                let mut s = Vec4::from_array([0.0, 0.0, 0.0, f32::from_bits(0xBAAD_F00D)]);
                lift.sample_into(f, &mut s);
                let got = s.to_array().map(f32::to_bits);
                assert_eq!(*out, got, "vf2 len {len} frame {f}");
                // The integer answer is narrowed away, but its shape is pinned:
                // the output address on the lerp path, the padding on snap.
                assert!(
                    r == out_addr || r == out[3],
                    "return shape len {len} frame {f}: {r:#x}"
                );
                let mut w = Vec4::from_array([0.0, 0.0, 0.0, f32::from_bits(0xBAAD_F00D)]);
                wrong::vec_sample_strict(&lift, f, &mut w);
                if w.to_array().map(f32::to_bits) != *out {
                    caught += 1;
                }
            }
            let _ = boxed;
        }
        // Random keys add NaN/inf/padding coverage.
        for _ in 0..60 {
            let len = 2 + (rng.u32() % 5) as usize;
            let words: Vec<[u32; 4]> = (0..len)
                .map(|_| [rng.u32(), rng.u32(), rng.u32(), rng.u32()])
                .collect();
            let boxed: Box<[[u32; 4]]> = words.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), len as u16);
            let keys: Vec<Vec4> = words.iter().map(|&w| words_to_vec4(w)).collect();
            let lift = RawVec3::new(keys);
            let f = rng.frame(len as f32 - 1.0);
            let out = Box::new([rng.u32(); 4]);
            let preset = *out;
            let r = unsafe { fn_0069AAA0::rw_0069aaa0(addr(&*obj), f, addr(&out[0])) };
            let mut s = Vec4::from_array(preset.map(f32::from_bits));
            lift.sample_into(f, &mut s);
            assert_eq!(*out, s.to_array().map(f32::to_bits), "random frame {f}");
            assert!(r == addr(&out[0]) || r == out[3]);
            let _ = boxed;
        }
        assert!(caught > 0, "wrong sampler never caught");
    }

    #[test]
    fn raw_vec3_size_matches() {
        for &count in &[0u16, 1, 2, 100, 0xFFFF] {
            let words = vec![[0u32; 4]; count as usize];
            let boxed: Box<[[u32; 4]]> = words.into_boxed_slice();
            let keys_addr = if count == 0 { 0 } else { addr(&boxed[0]) };
            let obj = raw_obj(keys_addr, count);
            let r = unsafe { fn_0069AC60::rw_0069AC60(addr(&*obj)) };
            let lift = RawVec3::new(vec![Vec4::from_array([0.0; 4]); count as usize]);
            assert_eq!(r, lift.storage_size(), "count {count}");
            assert_ne!(r, (count as u32).wrapping_shl(4));
            let _ = boxed;
        }
    }

    #[test]
    fn static_vec3_copy_matches() {
        let mut rng = Rng(0xA740);
        let mut caught = 0;
        for i in 0..60u32 {
            let w = if i < 10 {
                [
                    F32_EDGE[i as usize].to_bits(),
                    U32_EDGE[i as usize % U32_EDGE.len()],
                    rng.u32(),
                    rng.u32(),
                ]
            } else {
                [rng.u32(), rng.u32(), rng.u32(), rng.u32()]
            };
            let slot = Box::new(w);
            let obj = slot_obj(addr(&*slot));
            let lift = StaticVec3::new(words_to_vec4(w));
            let out = Box::new([0xDEAD_BEEFu32; 4]);
            let r = unsafe {
                fn_0069A740::rw_0069a740(addr(&*obj), rng.u32(), rng.u32(), addr(&out[0]))
            };
            let got = lift.get().to_array().map(f32::to_bits);
            assert_eq!(*out, got, "copy words {w:?}");
            assert_eq!(r, w[3], "vf7 answers the padding word");
            if w[3] != 0 {
                // Wrong: padding cleared.
                let wrong_out = [w[0], w[1], w[2], 0];
                if wrong_out != *out {
                    caught += 1;
                }
            }
            let _ = (slot, obj);
        }
        assert!(caught > 0, "wrong copy never caught");
    }

    #[test]
    fn static_vec3_compress_matches() {
        let mut rng = Rng(0xB8A0);
        let mut caught = 0;
        let run = |records: Vec<[u32; 4]>, count: i32, tol: f32, caught: &mut u32| {
            let boxed: Box<[[u32; 4]]> = records.clone().into_boxed_slice();
            let mut dst = Box::new([0xCDCD_CDCdu32; 4]);
            let obj = slot_obj(addr(&dst[0]));
            let r = unsafe {
                fn_0069B8A0::rw_0069B8A0(addr(&*obj), addr(&boxed[0]), count, tol.to_bits())
            };
            let recs: Vec<Vec4> = records.iter().map(|&w| words_to_vec4(w)).collect();
            let mut lift = StaticVec3::new(Vec4::from_array([1.0, 2.0, 3.0, 4.0]));
            let ok = lift.adopt_if_uniform(&recs, count, tol);
            assert_eq!(r, u32::from(ok), "count {count} tol {tol}");
            assert_eq!(*dst, lift.get().to_array().map(f32::to_bits), "copied record 0");
            let _ = (boxed, obj);
            let mut w = wrong::StaticVec3Like(Vec4::from_array([0.0; 4]));
            let wok = wrong::vec3_uniform_sum(&mut w, &recs, count, tol);
            if wok != ok || w.0.to_array().map(f32::to_bits) != *dst {
                *caught += 1;
            }
        };
        let rec = |x: f32, y: f32, z: f32, w: u32| [x.to_bits(), y.to_bits(), z.to_bits(), w];
        for &count in &[-2i32, 0, 1, 2, 3, 9] {
            let n = (count.max(1) as usize).min(9).max(1);
            for &tol in &[0.0f32, 0.01, 0.5] {
                // Uniform (padding varies: ignored).
                run(
                    (0..n).map(|k| rec(1.0, 2.0, 3.0, 100 + k as u32)).collect(),
                    count,
                    tol,
                    &mut caught,
                );
                // Outlier at the last record.
                let mut v: Vec<[u32; 4]> =
                    (0..n).map(|k| rec(1.0, 2.0, 3.0, 100 + k as u32)).collect();
                if n > 1 {
                    v[n - 1] = rec(1.0, 2.0, 9.0, 100);
                }
                run(v, count, tol, &mut caught);
                // Two-component spread: max passes, sum fails at tol 0.5.
                let mut v: Vec<[u32; 4]> =
                    (0..n).map(|k| rec(1.0, 2.0, 3.0, 100 + k as u32)).collect();
                if n > 1 {
                    v[1] = rec(1.4, 2.4, 3.0, 100);
                }
                run(v, count, tol, &mut caught);
                // NaN component: falls through the maximum chain.
                let mut v: Vec<[u32; 4]> =
                    (0..n).map(|k| rec(1.0, 2.0, 3.0, 100 + k as u32)).collect();
                if n > 1 {
                    v[1] = rec(f32::NAN, 2.0, 3.0, 100);
                }
                run(v, count, tol, &mut caught);
            }
        }
        for _ in 0..30 {
            let count = 2 + (rng.u32() % 7) as i32;
            let v: Vec<[u32; 4]> = (0..count as usize)
                .map(|_| [rng.u32(), rng.u32(), rng.u32(), rng.u32()])
                .collect();
            run(v, count, rng.small(0, 2), &mut caught);
        }
        assert!(caught > 0, "wrong uniformity never caught");
    }

    #[test]
    fn static_quat_copy_matches() {
        let mut rng = Rng(0xA720);
        let mut caught = 0;
        for i in 0..60u32 {
            let w: [u32; 4] = if i == 0 {
                [0, 0, 0, 0x3F80_0000] // identity rotation
            } else {
                [rng.u32(), rng.u32(), rng.u32(), rng.u32()]
            };
            let slot = Box::new(w);
            let obj = slot_obj(addr(&*slot));
            let lift = StaticQuat::new(Quat::from_xyzw(
                f32::from_bits(w[0]),
                f32::from_bits(w[1]),
                f32::from_bits(w[2]),
                f32::from_bits(w[3]),
            ));
            let out = Box::new([0xDEAD_BEEFu32; 4]);
            let r = unsafe {
                fn_0069A720::rw_0069a720(addr(&*obj), rng.u32(), rng.u32(), addr(&out[0]))
            };
            let q = lift.get();
            assert_eq!(r, addr(&out[0]), "vf8 answers out");
            assert_eq!(*out, [q.x.to_bits(), q.y.to_bits(), q.z.to_bits(), q.w.to_bits()]);
            // vf3: the same copy through the frame sampler, frame ignored.
            let out3 = Box::new([0xDEAD_BEEFu32; 4]);
            let fbits = rng.u32();
            let r3 = unsafe {
                fn_0069B5A0::rw_0069b5a0(addr(&*obj), fbits, addr(&out3[0]))
            };
            let mut s = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
            lift.sample_into(f32::from_bits(fbits), &mut s);
            assert_eq!(r3, addr(&out3[0]), "vf3 answers out");
            assert_eq!(
                *out3,
                [s.x.to_bits(), s.y.to_bits(), s.z.to_bits(), s.w.to_bits()],
                "vf3 copy"
            );
            // Wrong: z and w swapped.
            if [w[0], w[1], w[3], w[2]] != *out {
                caught += 1;
            }
            let _ = (slot, obj);
        }
        assert!(caught > 0, "wrong copy never caught");
    }

    #[test]
    fn static_quat_compress_matches() {
        let mut rng = Rng(0xB5C0);
        let mut caught = 0;
        let run = |records: Vec<[u32; 4]>, count: i32, tol: f32, caught: &mut u32| {
            let boxed: Box<[[u32; 4]]> = records.clone().into_boxed_slice();
            let mut dst = Box::new([0xCDCD_CDCdu32; 4]);
            let mut obj = slot_obj(addr(&dst[0]));
            let r = unsafe {
                fn_0069B5C0::rw_0069b5c0(
                    (&mut *obj) as *mut SlotObj as *mut u8,
                    addr(&boxed[0]) as usize as *const u8,
                    count,
                    tol.to_bits(),
                )
            };
            let recs: Vec<Quat> = records
                .iter()
                .map(|&w| {
                    Quat::from_xyzw(
                        f32::from_bits(w[0]),
                        f32::from_bits(w[1]),
                        f32::from_bits(w[2]),
                        f32::from_bits(w[3]),
                    )
                })
                .collect();
            let mut lift = StaticQuat::new(Quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
            let ok = lift.adopt_if_uniform(&recs, count, tol);
            assert_eq!(r, u32::from(ok), "count {count} tol {tol}");
            let q = lift.get();
            assert_eq!(
                *dst,
                [q.x.to_bits(), q.y.to_bits(), q.z.to_bits(), q.w.to_bits()],
                "copied record 0"
            );
            let _ = (boxed, obj);
            // Wrong: sum of squares instead of the maximum ladder.
            let mut wok = true;
            if count > 1 {
                let first = recs[0];
                let limit = tol * tol;
                let mut k = 1;
                while k < count {
                    let r = recs[k as usize];
                    let s = (first.x - r.x).powi(2)
                        + (first.y - r.y).powi(2)
                        + (first.z - r.z).powi(2)
                        + (first.w - r.w).powi(2);
                    if s > limit {
                        wok = false;
                        break;
                    }
                    k += 1;
                }
            }
            if wok != ok {
                *caught += 1;
            }
        };
        let rec = |x: f32, y: f32, z: f32, w: f32| {
            [x.to_bits(), y.to_bits(), z.to_bits(), w.to_bits()]
        };
        for &count in &[-2i32, 0, 1, 2, 3, 9] {
            let n = (count.max(1) as usize).min(9).max(1);
            for &tol in &[0.0f32, 0.01, 0.5] {
                run(
                    (0..n).map(|_| rec(0.0, 0.0, 0.0, 1.0)).collect(),
                    count,
                    tol,
                    &mut caught,
                );
                let mut v: Vec<[u32; 4]> =
                    (0..n).map(|_| rec(0.0, 0.0, 0.0, 1.0)).collect();
                if n > 1 {
                    v[n - 1] = rec(0.0, 0.0, 0.5, 1.0);
                }
                run(v, count, tol, &mut caught);
                // Spread over all four: max passes, sum fails at tol 0.5.
                let mut v: Vec<[u32; 4]> =
                    (0..n).map(|_| rec(0.0, 0.0, 0.0, 1.0)).collect();
                if n > 1 {
                    v[1] = rec(0.3, 0.3, 0.3, 1.3);
                }
                run(v, count, tol, &mut caught);
                // NaN component falls through the ladder.
                let mut v: Vec<[u32; 4]> =
                    (0..n).map(|_| rec(0.0, 0.0, 0.0, 1.0)).collect();
                if n > 1 {
                    v[1] = rec(f32::NAN, 0.0, 0.0, 1.0);
                }
                run(v, count, tol, &mut caught);
            }
        }
        for _ in 0..30 {
            let count = 2 + (rng.u32() % 7) as i32;
            let v: Vec<[u32; 4]> = (0..count as usize)
                .map(|_| [rng.u32(), rng.u32(), rng.u32(), rng.u32()])
                .collect();
            run(v, count, rng.small(0, 2), &mut caught);
        }
        assert!(caught > 0, "wrong uniformity never caught");
    }

    #[test]
    fn quantize_float_eval_matches() {
        let mut rng = Rng(0x9970);
        let mut caught = 0;
        const U64_EDGE: [u64; 10] = [
            0,
            1,
            2,
            0xFFFF_FFFF,
            0x1_0000_0000,
            (1 << 53) - 1,
            1 << 53,
            (1 << 53) + 1,
            u64::MAX - 1,
            u64::MAX,
        ];
        let run =
            |scale: f32, bias: f32, a: u64, b: u64, t: f32, ignored: u32, caught: &mut u32| {
            let mut words = Box::new([0xCCCC_CCCCu32; 8]);
            words[5] = scale.to_bits();
            words[6] = bias.to_bits();
            let p = Box::new([
                a as u32,
                (a >> 32) as u32,
                b as u32,
                (b >> 32) as u32,
            ]);
            let r = unsafe {
                fn_00699970::rw_00699970(addr(&words[0]), ignored, t, addr(&p[0]))
            };
            let lift = QuantizeFloat::new(scale, bias);
            assert_eq!(
                r.to_bits(),
                lift.eval_scaled(a, b, t).to_bits(),
                "scale {scale} bias {bias} a {a} b {b} t {t}"
            );
            if wrong::quant_f64_math(scale, bias, a, b, t).to_bits() != r.to_bits() {
                *caught += 1;
            }
            let _ = (words, p);
        };
        for &a in &U64_EDGE {
            for &b in &U64_EDGE {
                run(0.5, 1.0, a, b, 0.5, rng.u32(), &mut caught);
                run(1.0, 0.0, a, b, 0.0, rng.u32(), &mut caught);
            }
        }
        for i in 0..60u32 {
            let scale = if i < 24 { F32_EDGE[i as usize] } else { rng.f32_bits() };
            let bias = rng.f32_bits();
            let t = if i < 24 {
                F32_EDGE[(i as usize * 7) % F32_EDGE.len()]
            } else {
                rng.f32_bits()
            };
            run(scale, bias, rng.u64(), rng.u64(), t, rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong eval never caught");
    }
}

#[cfg(not(target_arch = "x86"))]
#[test]
fn no_rewrite_cases_on_host() {}
