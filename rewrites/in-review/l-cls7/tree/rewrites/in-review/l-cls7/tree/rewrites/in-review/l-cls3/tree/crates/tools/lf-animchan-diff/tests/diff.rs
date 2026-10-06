//! Differential cases, part 1: static and raw scalar channels.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value and every written byte, floats bit
//! for bit. Each case also runs a deliberately wrong lift, which must be
//! caught at least once. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe; out-boxes mutate
// through those addresses, so their bindings stay non-mut.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_animation::channel::frame::{AnimChannel, round_half_up};
    use lf_animation::channel::{RawBool, RawFloat, RawInt, StaticFloat, StaticInt};
    use lf_animchan_diff::rewrites::*;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{F32_EDGE, RawObj, Rng, StaticObj, U32_EDGE, addr};

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_animation::channel::frame::AnimChannel;
        use lf_animation::channel::{RawBool, RawFloat, RawInt, StaticFloat};

        pub fn static_eval_neg(c: &StaticFloat) -> f32 {
            -c.get()
        }

        pub fn static_copy_plus_one(c: &StaticFloat) -> f32 {
            c.get() + 1.0
        }

        pub fn static_copy_zero(_c: &StaticFloat) -> f32 {
            0.0
        }

        /// Off-by-one comparison: fails samples exactly on the boundary.
        pub fn static_uniform_ge(
            c: &mut StaticFloat,
            samples: &[f32],
            count: i32,
            stride: u32,
            tol: f32,
        ) -> bool {
            let first = samples[0];
            *c = StaticFloat::new(first);
            if count <= 1 {
                return true;
            }
            let step = stride.wrapping_add(1) as usize;
            let limit = tol * tol;
            let mut k: i32 = 1;
            while k < count {
                let x = samples[(k as usize).wrapping_mul(step)];
                let diff = first - x;
                if diff * diff >= limit {
                    return false;
                }
                k += 1;
            }
            true
        }

        /// Skips the uniformity loop entirely.
        pub fn static_int_always(_samples: &[u32], _count: i32) -> bool {
            true
        }

        pub fn raw_lerp_flip_t(a: f32, b: f32, t: f32) -> f32 {
            (b - a) * (1.0 - t) + a
        }

        pub fn raw_lerp_f64_math(a: f32, b: f32, t: f32) -> f64 {
            f64::from(b - a).mul_add(f64::from(t), f64::from(a))
        }

        /// Strict snap-up comparison: exact-boundary fractions go up early.
        pub fn raw_sample_strict(c: &RawFloat, frame: f32) -> f32 {
            use lf_animation::channel::frame::{SNAP_HI, SNAP_LO, clamp_index, key_below};
            let (index, frac) = key_below(frame);
            let last = c.key_count() as i32 - 1;
            if frac < SNAP_HI {
                if !(frac > SNAP_LO) {
                    c.keys()[clamp_index(index, last) as usize]
                } else {
                    let j = clamp_index(index.wrapping_add(1), last);
                    let i = clamp_index(index, last);
                    let a = c.keys()[i as usize];
                    let b = c.keys()[j as usize];
                    (b - a) * frac + a
                }
            } else {
                let k = clamp_index(index.wrapping_add(1), last);
                c.keys()[k as usize]
            }
        }

        pub fn raw_int_next_key(c: &RawInt, idx: u32) -> u32 {
            c.keys()[(idx as usize).wrapping_add(1) % c.key_count()]
        }

        pub fn raw_int_trunc(c: &RawInt, frame: f32) -> u32 {
            use lf_animation::channel::frame::{clamp_index, truncate_to_i32};
            let last = c.key_count() as i32 - 1;
            c.keys()[clamp_index(truncate_to_i32(frame), last) as usize]
        }

        pub fn raw_bool_lo_two(c: &RawBool, frame: f32) -> u8 {
            use lf_animation::channel::frame::round_half_up;
            let raw = round_half_up(frame);
            let index = if raw < 0 { 0u32 } else { raw as u32 };
            let masked = c.bytes()[(index >> 3) as usize] & (index as u8);
            u8::from((masked & 3) != 0)
        }
    }

    fn static_obj(value_bits: u32) -> Box<StaticObj> {
        Box::new(StaticObj {
            vtable: 0x1111_1111,
            hdr: [0xAA, 0xBB, 0xCC, 0xDD],
            value: value_bits,
        })
    }

    fn raw_obj(keys_addr: u32, count: u16) -> Box<RawObj> {
        Box::new(RawObj {
            vtable: 0x2222_2222,
            hdr: [0x11, 0x22, 0x33, 0x44],
            keys: keys_addr,
            count,
            gate: 0x55AA,
        })
    }

    #[test]
    fn static_float_eval_matches() {
        let mut rng = Rng(0xE1A4);
        let mut caught = 0;
        for i in 0..80u32 {
            let bits = if i < 24 {
                F32_EDGE[i as usize].to_bits()
            } else {
                rng.u32()
            };
            let obj = static_obj(bits);
            let (a, b, c) = (rng.u32(), rng.u32(), rng.u32());
            let got = unsafe { fn_0069A710::rw_0069A710(addr(&*obj), a, b, c) };
            let lift = StaticFloat::new(f32::from_bits(bits));
            assert_eq!(got.to_bits(), lift.eval().to_bits(), "value {bits:#x}");
            if wrong::static_eval_neg(&lift).to_bits() != got.to_bits() {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong eval never caught");
    }

    #[test]
    fn static_float_copies_match() {
        let mut rng = Rng(0xC09E);
        let mut caught9 = 0;
        let mut caught4 = 0;
        for i in 0..80u32 {
            let bits = if i < 24 {
                F32_EDGE[i as usize].to_bits()
            } else {
                rng.u32()
            };
            let obj = static_obj(bits);
            let lift = StaticFloat::new(f32::from_bits(bits));
            // vf9: copy ignoring the trailing words.
            let mut out9 = Box::new(0xDEAD_BEEFu32);
            let r9 = unsafe {
                fn_0069A700::rw_0069a700(addr(&*obj), rng.u32(), rng.u32(), addr(&*out9))
            };
            assert_eq!(r9, addr(&*out9), "vf9 answers out");
            assert_eq!(*out9, lift.copy_key().to_bits(), "vf9 value {bits:#x}");
            if wrong::static_copy_plus_one(&lift).to_bits() != *out9 {
                caught9 += 1;
            }
            // vf4: copy ignoring the frame.
            let mut out4 = Box::new(0xDEAD_BEEFu32);
            let fbits = rng.u32();
            let r4 = unsafe { fn_0069B360::rw_0069b360(addr(&*obj), fbits, addr(&*out4)) };
            let mut s = 0.0f32;
            lift.sample_into(f32::from_bits(fbits), &mut s);
            assert_eq!(r4, addr(&*out4), "vf4 answers out");
            assert_eq!(*out4, s.to_bits(), "vf4 value {bits:#x}");
            if wrong::static_copy_zero(&lift).to_bits() != *out4 {
                caught4 += 1;
            }
        }
        assert!(caught9 > 0, "wrong vf9 never caught");
        assert!(caught4 > 0, "wrong vf4 never caught");
    }

    #[test]
    fn static_float_uniformity_matches() {
        let mut rng = Rng(0xF140);
        let mut caught = 0;
        let mut cases = 0;
        // (samples, count, stride, tol) grid plus targeted shapes.
        let run = |samples: Vec<f32>,
                   count: u32,
                   stride: u32,
                   tol: f32,
                   objval: u32,
                   caught: &mut u32| {
            let boxed: Box<[f32]> = samples.clone().into_boxed_slice();
            let base = addr(&boxed[0]);
            let obj = static_obj(objval);
            let obj_addr = addr(&*obj);
            let r = unsafe { fn_0069B370::rw_0069b370(obj_addr, base, count, stride, tol) };
            let mut lift = StaticFloat::new(f32::from_bits(obj.value ^ 0xFFFF_FFFF));
            let before = obj.value;
            let ok = lift.adopt_if_uniform(&samples, count as i32, stride, tol);
            assert_eq!(r, u32::from(ok), "count {count} stride {stride} tol {tol}");
            assert_eq!(obj.value, lift.get().to_bits(), "stored first sample");
            assert_eq!(obj.vtable, 0x1111_1111);
            assert_eq!(obj.hdr, [0xAA, 0xBB, 0xCC, 0xDD]);
            let _ = (boxed, before);
            let mut w = StaticFloat::new(0.0);
            let wok = wrong::static_uniform_ge(&mut w, &samples, count as i32, stride, tol);
            if wok != ok || w.get().to_bits() != obj.value {
                *caught += 1;
            }
        };
        for &count in &[0u32, 1, 2, 3, 5, 33, 0x8000_0000, 0xFFFF_FFFF] {
            for &stride in &[0u32, 1, 3] {
                let n = if (count as i32) > 1 {
                    (count as usize) * (stride as usize + 1)
                } else {
                    1
                };
                // Uniform.
                run(vec![2.0; n], count, stride, 0.5, rng.u32(), &mut caught);
                cases += 1;
                // Outlier at the second element.
                let mut v = vec![2.0; n];
                if n > 1 + stride as usize {
                    v[1 + stride as usize] = 9.0;
                }
                run(v, count, stride, 0.5, rng.u32(), &mut caught);
                cases += 1;
                // Outlier at the last element.
                let mut v = vec![2.0; n];
                if n > 1 {
                    v[n - 1] = -9.0;
                }
                run(v, count, stride, 0.5, rng.u32(), &mut caught);
                cases += 1;
                // Exact boundary: diff^2 == tol^2 passes.
                let v: Vec<f32> = (0..n)
                    .map(|k| {
                        if k % (stride as usize + 1) == 0 && k > 0 {
                            1.5
                        } else {
                            1.0
                        }
                    })
                    .collect();
                run(v, count, stride, 0.5, rng.u32(), &mut caught);
                cases += 1;
                // NaN samples: unordered never exceeds.
                let mut v = vec![1.0; n];
                if n > 1 {
                    v[n - 1] = f32::NAN;
                }
                run(v, count, stride, 0.0, rng.u32(), &mut caught);
                cases += 1;
            }
        }
        // Random shapes.
        for _ in 0..40 {
            let count = 2 + rng.u32() % 12;
            let stride = rng.u32() % 3;
            let n = (count as usize) * (stride as usize + 1);
            let v: Vec<f32> = (0..n).map(|_| rng.f32_bits()).collect();
            let tol = if rng.u32() % 2 == 0 {
                rng.f32_bits()
            } else {
                rng.small(0, 4)
            };
            run(v, count, stride, tol, rng.u32(), &mut caught);
            cases += 1;
        }
        assert!(cases > 100, "case grid too small: {cases}");
        assert!(caught > 0, "wrong uniformity never caught");
    }

    #[test]
    fn static_int_uniformity_matches() {
        let mut rng = Rng(0x17u64);
        let mut caught = 0;
        let run = |samples: Vec<u32>, count: i32, objval: u32, caught: &mut u32| {
            let boxed: Box<[u32]> = samples.clone().into_boxed_slice();
            let mut obj = static_obj(objval);
            let r = unsafe {
                fn_0069B690::rw_0069b690(
                    (&mut *obj) as *mut StaticObj as *mut u8,
                    boxed.as_ptr(),
                    count,
                )
            };
            let mut lift = StaticInt::new(obj.value ^ 0xFFFF_FFFF);
            let ok = lift.adopt_if_uniform(&samples, count);
            assert_eq!(r, u32::from(ok), "count {count} samples {samples:?}");
            // On success the first sample is stored; on mismatch the word is untouched.
            if ok {
                assert_eq!(obj.value, lift.get());
            }
            assert_eq!(obj.vtable, 0x1111_1111);
            let _ = boxed;
            if wrong::static_int_always(&samples, count) != ok {
                *caught += 1;
            }
        };
        for &count in &[-3i32, -1, 0, 1, 2, 3, 9] {
            let n = (count.max(1) as usize).min(9).max(1);
            run(vec![7; n], count, rng.u32(), &mut caught);
            let mut v = vec![7; n];
            if n > 1 {
                v[1] = 8;
            }
            run(v, count, rng.u32(), &mut caught);
            let mut v = vec![7; n];
            if n > 1 {
                v[n - 1] = 8;
            }
            run(v, count, rng.u32(), &mut caught);
            for &e in &U32_EDGE {
                run(vec![e; n], count, rng.u32(), &mut caught);
            }
        }
        for _ in 0..30 {
            let count = 2 + (rng.u32() % 8) as i32;
            let v: Vec<u32> = (0..count as usize).map(|_| rng.u32()).collect();
            run(v, count, rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong uniformity never caught");
    }

    #[test]
    fn raw_float_lerps_match() {
        let mut rng = Rng(0x9E99);
        let mut caught9 = 0;
        let mut caught13 = 0;
        for len in [2usize, 3, 8] {
            for trial in 0..60u32 {
                let keys: Vec<f32> = (0..len)
                    .map(|k| {
                        if trial == 0 {
                            F32_EDGE[k % F32_EDGE.len()]
                        } else if trial == 1 {
                            U32_EDGE[k % U32_EDGE.len()] as f32
                        } else {
                            rng.f32_bits()
                        }
                    })
                    .collect();
                let boxed: Box<[f32]> = keys.clone().into_boxed_slice();
                let obj = raw_obj(addr(&boxed[0]), len as u16);
                let lift = RawFloat::new(keys.clone());
                let idx = if trial % 3 == 0 {
                    0
                } else if trial % 3 == 1 {
                    (len - 2) as u32
                } else {
                    rng.u32() % (len as u32 - 1)
                };
                let t = if trial < 24 {
                    F32_EDGE[(trial as usize) % F32_EDGE.len()]
                } else {
                    rng.f32_bits()
                };
                // vf9: blend into out.
                let mut out9 = Box::new(0u32);
                let r9 = unsafe { fn_0069A680::rw_0069a680(addr(&*obj), idx, t, addr(&*out9)) };
                let l9 = lift.lerp_at(idx, t);
                assert_eq!(r9, addr(&*out9), "vf9 answers out");
                assert_eq!(*out9, l9.to_bits(), "vf9 len {len} idx {idx} t {t}");
                let a = keys[idx as usize];
                let b = keys[idx as usize + 1];
                if wrong::raw_lerp_flip_t(a, b, t).to_bits() != *out9 {
                    caught9 += 1;
                }
                // vf13: same blend at double width.
                let r13 = unsafe { fn_0069A6B0::rw_0069a6b0(addr(&*obj), idx, t, rng.u32()) };
                assert_eq!(r13.to_bits(), lift.lerp_at_f64(idx, t).to_bits(), "vf13");
                if wrong::raw_lerp_f64_math(a, b, t).to_bits() != r13.to_bits() {
                    caught13 += 1;
                }
                let _ = boxed;
            }
        }
        assert!(caught9 > 0, "wrong lerp never caught");
        assert!(caught13 > 0, "wrong f64 lerp never caught");
    }

    #[test]
    fn raw_float_sample_matches() {
        let mut rng = Rng(0xA4F4);
        let mut caught = 0;
        for len in [1usize, 2, 3, 8] {
            // Distinct keys so neighbouring selections differ.
            let keys: Vec<f32> = (0..len).map(|k| 10.0 + k as f32 * 7.0).collect();
            let boxed: Box<[f32]> = keys.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), len as u16);
            let lift = RawFloat::new(keys);
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
                let out = Box::new(0xBAAD_F00Du32);
                let r = unsafe { fn_0069A7F0::rw_0069a7f0(addr(&*obj), f, addr(&*out)) };
                let mut s = 0.0f32;
                lift.sample_into(f, &mut s);
                assert_eq!(r, addr(&*out), "vf4 answers out (frame {f})");
                assert_eq!(*out, s.to_bits(), "vf4 len {len} frame {f}");
                if wrong::raw_sample_strict(&lift, f).to_bits() != *out {
                    caught += 1;
                }
            }
            let _ = boxed;
        }
        // Random keys add NaN/inf coverage.
        for _ in 0..60 {
            let len = 2 + (rng.u32() % 6) as usize;
            let keys: Vec<f32> = (0..len).map(|_| rng.f32_bits()).collect();
            let boxed: Box<[f32]> = keys.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), len as u16);
            let lift = RawFloat::new(keys);
            let f = rng.frame(len as f32 - 1.0);
            let out = Box::new(0u32);
            let r = unsafe { fn_0069A7F0::rw_0069a7f0(addr(&*obj), f, addr(&*out)) };
            let mut s = 0.0f32;
            lift.sample_into(f, &mut s);
            assert_eq!(r, addr(&*out));
            assert_eq!(*out, s.to_bits(), "random keys frame {f}");
            let _ = boxed;
        }
        assert!(caught > 0, "wrong sampler never caught");
    }

    #[test]
    fn raw_int_key_and_sample_match() {
        let mut rng = Rng(0x10u64);
        let mut caught_key = 0;
        let mut caught_sample = 0;
        for len in [1usize, 2, 5] {
            let keys: Vec<u32> = (0..len).map(|k| 1000 + k as u32 * 101).collect();
            let boxed: Box<[u32]> = keys.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), len as u16);
            let lift = RawInt::new(keys.clone());
            // Key copy at every index.
            for idx in 0..len as u32 {
                let out = Box::new(0u32);
                let r =
                    unsafe { fn_0069A6E0::rw_0069a6e0(addr(&*obj), idx, rng.u32(), addr(&*out)) };
                assert_eq!(r, addr(&*out), "vf10 answers out");
                assert_eq!(*out, lift.key_at(idx), "vf10 idx {idx}");
                if wrong::raw_int_next_key(&lift, idx) != *out {
                    caught_key += 1;
                }
            }
            // Sampler over edge and random frames.
            let mut frames: Vec<f32> = F32_EDGE.to_vec();
            for _ in 0..40 {
                frames.push(rng.frame(len as f32 - 1.0));
            }
            for f in frames {
                let out = Box::new(0xBAAD_F00Du32);
                let r = unsafe { fn_0069B070::rw_0069b070(addr(&*obj), f, addr(&*out)) };
                let mut s = 0u32;
                lift.sample_into(f, &mut s);
                assert_eq!(r, addr(&*out), "vf5 answers out (frame {f})");
                assert_eq!(*out, s, "vf5 len {len} frame {f}");
                if wrong::raw_int_trunc(&lift, f) != *out {
                    caught_sample += 1;
                }
            }
            let _ = boxed;
        }
        assert!(caught_key > 0, "wrong key copy never caught");
        assert!(caught_sample > 0, "wrong sampler never caught");
    }

    #[test]
    fn raw_int_size_matches() {
        for &count in &[0u16, 1, 2, 3, 255, 0xFFFF] {
            let keys = vec![0u32; count as usize];
            let boxed: Box<[u32]> = keys.into_boxed_slice();
            let keys_addr = if count == 0 { 0 } else { addr(&boxed[0]) };
            let obj = raw_obj(keys_addr, count);
            let r = unsafe { fn_0069A9A0::rw_0069a9a0(addr(&*obj)) };
            let lift = RawInt::new(vec![0u32; count as usize]);
            assert_eq!(r, lift.storage_size(), "count {count}");
            assert_ne!(r, (count as u32).wrapping_mul(4).wrapping_add(0x14));
            let _ = boxed;
        }
    }

    #[test]
    fn raw_bool_sample_and_size_match() {
        let mut rng = Rng(0xB001);
        let mut caught = 0;
        // Targeted wrong-catch: bit 2 set, low two clear.
        {
            let bytes = vec![0u8, 0b0111];
            let boxed: Box<[u8]> = bytes.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), 2);
            let lift = RawBool::new(bytes);
            let out = Box::new(0xFFu8);
            let r = unsafe { fn_0069B120::rw_0069b120(addr(&*obj), 12.0, addr(&*out)) };
            let mut s = 0u8;
            lift.sample_into(12.0, &mut s);
            assert_eq!(r, addr(&*out));
            assert_eq!(*out, s);
            assert_eq!(s, 1);
            assert_ne!(wrong::raw_bool_lo_two(&lift, 12.0), *out);
            caught += 1;
            let _ = boxed;
        }
        for len in [1usize, 2, 8] {
            let bytes: Vec<u8> = (0..len).map(|_| rng.u32() as u8).collect();
            let boxed: Box<[u8]> = bytes.clone().into_boxed_slice();
            let obj = raw_obj(addr(&boxed[0]), len as u16);
            let lift = RawBool::new(bytes);
            // Every selectable index, plus edges and random in-domain frames.
            let mut frames: Vec<f32> = (0..8 * len).map(|i| i as f32).collect();
            frames.extend_from_slice(&F32_EDGE);
            for _ in 0..40 {
                frames.push(rng.frame((8 * len) as f32));
            }
            for f in frames {
                // Domain filter: the original over-reads past the last byte;
                // the lift panics there instead, so only in-domain frames run.
                let raw = round_half_up(f);
                let index = if raw < 0 { 0u32 } else { raw as u32 };
                if index >> 3 >= len as u32 {
                    continue;
                }
                let out = Box::new(0xFFu8);
                let r = unsafe { fn_0069B120::rw_0069b120(addr(&*obj), f, addr(&*out)) };
                let mut s = 0u8;
                lift.sample_into(f, &mut s);
                assert_eq!(r, addr(&*out), "vf6 answers out (frame {f})");
                assert_eq!(*out, s, "vf6 len {len} frame {f}");
                if wrong::raw_bool_lo_two(&lift, f) != *out {
                    caught += 1;
                }
            }
            let _ = boxed;
        }
        for &count in &[0u16, 1, 2, 0xFFFF] {
            let bytes = vec![0u8; count as usize];
            let boxed: Box<[u8]> = bytes.into_boxed_slice();
            let keys_addr = if count == 0 { 0 } else { addr(&boxed[0]) };
            let obj = raw_obj(keys_addr, count);
            let r = unsafe { fn_0069B2D0::rw_0069B2D0(addr(&*obj)) };
            let lift = RawBool::new(vec![0u8; count as usize]);
            assert_eq!(r, lift.storage_size(), "count {count}");
            assert_ne!(r, (count as u32).wrapping_add(0x14));
            let _ = boxed;
        }
        assert!(caught > 0, "wrong sampler never caught");
    }
}

#[cfg(not(target_arch = "x86"))]
#[test]
fn no_rewrite_cases_on_host() {}
