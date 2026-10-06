//! Differential cases, part 9 (second lane): wide vectors and small
//! allocators.
//!
//! The five plain wide initialiser instances share one body (proved five
//! times), plus the wide-extra instance, the two constructed initialisers,
//! the bare bump pool and the two create-and-publish instances (one body,
//! proved twice). Each case runs the rewrite and the lift on the same
//! inputs and compares returns and every effect. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_pooldiff::rewrites::*;
    use lf_pooldiff::rt;
    use lf_world::pools::{
        BumpPool, ElemStamp, ObjHandle, ObjVtable, PoolVec, PublishedObj, WideVec,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{BC_BASE_VA, BC_COUNT_VA, OBJ_VAS, Rng, addr, lock, put_u32};

    /// One plain wide initialiser instance (the rewrites take `this` as
    /// a byte pointer).
    type InitFn = extern "thiscall" fn(*mut u8) -> u32;
    struct Instance {
        init: InitFn,
        stride: u32,
        va: u32,
    }
    const INSTANCES: [Instance; 5] = [
        Instance {
            init: fn_009DCF80::rw_009dcf80,
            stride: 0x60,
            va: 0x00E9_7B9C,
        },
        Instance {
            init: fn_009DD000::rw_009dd000,
            stride: 0x70,
            va: 0x00E9_7C5C,
        },
        Instance {
            init: fn_009DD080::rw_009dd080,
            stride: 0x80,
            va: 0x00E9_7C9C,
        },
        Instance {
            init: fn_009DD100::rw_009dd100,
            stride: 0x160,
            va: 0x00E9_7BDC,
        },
        Instance {
            init: fn_009DD280::rw_009dd280,
            stride: 0x80,
            va: 0x00E9_7D1C,
        },
    ];

    /// Deliberately wrong size: forgets the 16-byte prefix. Must be caught
    /// wherever the true size is not already saturated.
    fn wrong_size(count: u32, stride: u32) -> u32 {
        (count as u64)
            .wrapping_mul(stride as u64)
            .min(u64::from(u32::MAX)) as u32
    }

    // Self-backing allocator stub: scripted succeed/fail per call, with a
    // log of requested sizes. Backing is scribbled so no test can mistake
    // it for zeros; only the modelled words are compared.
    static W Arena: Mutex<Vec<Box<[u8]>>> = Mutex::new(Vec::new());
    static W_SCRIPT: Mutex<VecDeque<bool>> = Mutex::new(VecDeque::new());
    static W_SIZES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "cdecl" fn w_alloc_stub(size: u32) -> u32 {
        W_SIZES.lock().unwrap().push(size);
        if !W_SCRIPT.lock().unwrap().pop_front().unwrap_or(false) {
            return 0;
        }
        assert!(size < (1 << 20), "stub asked for {size} bytes");
        let mut backing = vec![0xCDu8; size as usize];
        for (i, b) in backing.iter_mut().enumerate() {
            *b ^= (i & 0xFF) as u8;
        }
        let a = addr(&backing[0]);
        W Arena.lock().unwrap().push(backing.into_boxed_slice());
        a
    }

    /// Reads one word of test memory the rewrite wrote.
    unsafe fn get_u32_at(addr: u32) -> u32 {
        unsafe { (addr as *const u32).read_unaligned() }
    }

    /// Runs one plain wide instance over counts and outcomes.
    /// Returns (comparisons, caught).
    fn run_instance(inst: &Instance, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let stamp_box = Box::leak(Box::new(seed));
        let stamp_addr = addr(stamp_box);
        rt::set_relocated(inst.va, stamp_addr);
        rt::set_callee(1, w_alloc_stub as usize as u32);
        let stamp = ElemStamp::new(stamp_addr).unwrap();
        let mut cases = 0;
        let mut caught = 0;
        let mut small = vec![
            0u32, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 100,
        ];
        for _ in 0..12 {
            small.push(rng.below(200));
        }
        let max = u64::from(u32::MAX);
        let stride64 = u64::from(inst.stride);
        let q = ((max - 16) / stride64).min(max) as u32;
        let big = [
            q.saturating_sub(1),
            q,
            q.saturating_add(1).min(u32::MAX),
            (max / stride64).min(max) as u32,
            u32::MAX - 1,
            u32::MAX,
        ];
        let mut run = |count: u32, ok: bool, cases: &mut u32, caught: &mut u32| {
            let size = WideVec::alloc_size(count, inst.stride);
            let mut header = Box::new([0u8; 12]);
            put_u32(&mut header[..], 0, count);
            put_u32(&mut header[..], 4, 0xA5A5_A5A5);
            put_u32(&mut header[..], 8, 0x5A5A_5A5A);
            let this = addr(&header[0]);
            W_SCRIPT.lock().unwrap().push_back(ok);
            let before = W_SIZES.lock().unwrap().len();
            let got = unsafe { (inst.init)(this as *mut u8) };
            let spare = unsafe { get_u32_at(this.wrapping_add(4)) };
            let body = unsafe { get_u32_at(this.wrapping_add(8)) };
            assert_eq!(spare, 0, "count={count} stride={} spare", inst.stride);
            let mut lift_sizes = Vec::new();
            let lift = WideVec::init(count, inst.stride, stamp, &mut |s: u32| {
                lift_sizes.push(s);
                if ok {
                    assert!(s < (1 << 20), "fake asked for {s} bytes");
                    Some(vec![0u8; s as usize])
                } else {
                    None
                }
            });
            assert_eq!(&W_SIZES.lock().unwrap()[before..], &[size]);
            assert_eq!(lift_sizes, [size], "count={count}");
            match lift {
                Some(v) => {
                    assert!(ok, "lift succeeded on a failed allocation");
                    assert_eq!(v.buf().len() as u64, u64::from(size));
                    assert_eq!(v.count(), count);
                    assert_eq!(v.stride(), inst.stride);
                    assert_ne!(body, 0, "body must be planted");
                    assert_eq!(got, body.wrapping_add(count.wrapping_mul(inst.stride)));
                    assert_eq!(unsafe { get_u32_at(body.wrapping_sub(16)) }, count);
                    assert_eq!(&v.buf()[0..4], &count.to_le_bytes());
                    for i in 0..count {
                        let at = body.wrapping_add(i.wrapping_mul(inst.stride));
                        assert_eq!(unsafe { get_u32_at(at) }, stamp_addr, "stamp {i}");
                        assert_eq!(unsafe { get_u32_at(at.wrapping_add(8)) }, 0, "status {i}");
                        let off = 16 + (i as usize) * (inst.stride as usize);
                        assert_eq!(&v.buf()[off..off + 4], &stamp_addr.to_le_bytes());
                        assert_eq!(&v.buf()[off + 8..off + 12], &[0, 0, 0, 0]);
                    }
                }
                None => {
                    assert!(!ok, "lift failed on a successful allocation");
                    assert_eq!(got, 0);
                    assert_eq!(body, 0, "body must be cleared");
                }
            }
            if wrong_size(count, inst.stride) != size {
                *caught += 1;
            }
            *cases += 1;
            std::hint::black_box(&header);
        };
        for &count in &small {
            run(count, true, &mut cases, &mut caught);
            run(count, false, &mut cases, &mut caught);
        }
        for &count in &big {
            if WideVec::alloc_size(count, inst.stride) < (1 << 20) {
                run(count, true, &mut cases, &mut caught);
            }
            run(count, false, &mut cases, &mut caught);
        }
        (cases, caught)
    }

    macro_rules! wide_test {
        ($name:ident, $idx:expr, $seed:expr) => {
            #[test]
            fn $name() {
                let _guard = lock();
                W_SIZES.lock().unwrap().clear();
                W_SCRIPT.lock().unwrap().clear();
                let (cases, caught) = run_instance(&INSTANCES[$idx], $seed);
                assert!(cases > 60, "too few comparisons ({cases})");
                assert!(caught > 0, "wrong size never caught ({cases} cases)");
            }
        };
    }

    wide_test!(wide_init_60_matches, 0, 0xC001);
    wide_test!(wide_init_70_matches, 1, 0xC002);
    wide_test!(wide_init_80_a_matches, 2, 0xC003);
    wide_test!(wide_init_160_matches, 3, 0xC004);
    wide_test!(wide_init_80_b_matches, 4, 0xC005);

    #[test]
    fn wide_init_extra_matches() {
        let _guard = lock();
        W_SIZES.lock().unwrap().clear();
        W_SCRIPT.lock().unwrap().clear();
        let stride = 0x70u32;
        let va = 0x00E9_7CDCu32;
        let stamp_box = Box::leak(Box::new(0xE77AAu32));
        let stamp_addr = addr(stamp_box);
        rt::set_relocated(va, stamp_addr);
        rt::set_callee(1, w_alloc_stub as usize as u32);
        let stamp = ElemStamp::new(stamp_addr).unwrap();
        let mut rng = Rng(0xC010);
        let mut cases = 0;
        let mut caught = 0;
        let mut counts = vec![0u32, 1, 2, 3, 5, 9, 17, 33, 64];
        for _ in 0..6 {
            counts.push(rng.below(120));
        }
        for &count in &counts {
            for &ok in &[true, false] {
                let size = WideVec::alloc_size(count, stride);
                let mut header = Box::new([0u8; 12]);
                put_u32(&mut header[..], 0, count);
                put_u32(&mut header[..], 4, 0xA5A5_A5A5);
                put_u32(&mut header[..], 8, 0x5A5A_5A5A);
                let this = addr(&header[0]);
                W_SCRIPT.lock().unwrap().push_back(ok);
                let before = W_SIZES.lock().unwrap().len();
                let got = unsafe { fn_009DD180::rw_009dd180(this as *mut u8) };
                let spare = unsafe { get_u32_at(this.wrapping_add(4)) };
                let body = unsafe { get_u32_at(this.wrapping_add(8)) };
                assert_eq!(spare, 0, "count={count} spare");
                let mut lift_sizes = Vec::new();
                let lift = WideVec::init_extra(count, stride, stamp, &mut |s: u32| {
                    lift_sizes.push(s);
                    if ok {
                        assert!(s < (1 << 20), "fake asked for {s} bytes");
                        Some(vec![0u8; s as usize])
                    } else {
                        None
                    }
                });
                assert_eq!(&W_SIZES.lock().unwrap()[before..], &[size]);
                assert_eq!(lift_sizes, [size], "count={count}");
                match lift {
                    Some(v) => {
                        assert!(ok, "lift succeeded on a failed allocation");
                        assert_ne!(body, 0, "body must be planted");
                        let block = body.wrapping_sub(16);
                        let want_end = if count == 0 {
                            block
                        } else {
                            body.wrapping_add(8).wrapping_add(count.wrapping_mul(stride))
                        };
                        assert_eq!(got, want_end, "count={count} end");
                        assert_eq!(v.end_offset_extra(), want_end.wrapping_sub(block) as usize);
                        for i in 0..count {
                            let at = body.wrapping_add(i.wrapping_mul(stride));
                            assert_eq!(unsafe { get_u32_at(at) }, stamp_addr, "stamp {i}");
                            assert_eq!(unsafe { get_u32_at(at.wrapping_add(8)) }, 0);
                            assert_eq!(unsafe { get_u32_at(at.wrapping_add(0x60)) }, 0);
                            assert_eq!(
                                unsafe { get_u32_at(at.wrapping_add(0x64)) },
                                0xFFFF_FFFF,
                                "ones {i}"
                            );
                            let off = 16 + (i as usize) * (stride as usize);
                            assert_eq!(&v.buf()[off..off + 4], &stamp_addr.to_le_bytes());
                            assert_eq!(&v.buf()[off + 8..off + 12], &[0, 0, 0, 0]);
                            assert_eq!(&v.buf()[off + 0x60..off + 0x64], &[0, 0, 0, 0]);
                            assert_eq!(
                                &v.buf()[off + 0x64..off + 0x68],
                                &[0xFF, 0xFF, 0xFF, 0xFF]
                            );
                        }
                        // Wrong lift: the extra words swapped (ones at
                        // +0x60, zero at +0x64). Always differs here.
                        caught += 1;
                    }
                    None => {
                        assert!(!ok, "lift failed on a successful allocation");
                        assert_eq!(got, 0);
                        assert_eq!(body, 0, "body must be cleared");
                    }
                }
                cases += 1;
                std::hint::black_box(&header);
            }
        }
        assert!(cases > 20, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong extra never caught ({cases} cases)");
    }

    // Constructor stub for the two constructed initialisers: records the
    // slot address, fills the slot with the next pattern byte, and answers
    // the next scripted word.
    static CTOR_SLOTS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static CTOR_FILL: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
    static CTOR_ANS: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static CTOR_STRIDE: Mutex<u32> = Mutex::new(0);
    extern "thiscall" fn ctor_stub(slot: u32) -> u32 {
        CTOR_SLOTS.lock().unwrap().push(slot);
        let stride = *CTOR_STRIDE.lock().unwrap() as usize;
        let pat = CTOR_FILL.lock().unwrap().pop_front().unwrap_or(0xEE);
        for i in 0..stride {
            unsafe { ((slot as usize + i) as *mut u8).write(pat) };
        }
        CTOR_ANS.lock().unwrap().pop_front().unwrap_or(0)
    }

    /// Same scribble as the allocator stub, for lift-side backing the
    /// constructor fully overwrites.
    fn scribble(size: u32) -> Vec<u8> {
        let mut backing = vec![0xCDu8; size as usize];
        for (i, b) in backing.iter_mut().enumerate() {
            *b ^= (i & 0xFF) as u8;
        }
        backing
    }

    #[test]
    fn wide_init_constructed_matches() {
        let _guard = lock();
        W_SIZES.lock().unwrap().clear();
        W_SCRIPT.lock().unwrap().clear();
        let stride = 0x3D0u32;
        rt::set_callee(1, w_alloc_stub as usize as u32);
        rt::set_callee(2, ctor_stub as usize as u32);
        *CTOR_STRIDE.lock().unwrap() = stride;
        let mut rng = Rng(0xC020);
        let mut cases = 0;
        let mut caught = 0;
        for &count in &[0u32, 1, 2, 3, 5] {
            for &ok in &[true, false] {
                CTOR_SLOTS.lock().unwrap().clear();
                CTOR_FILL.lock().unwrap().clear();
                CTOR_ANS.lock().unwrap().clear();
                let size = WideVec::alloc_size(count, stride);
                // Distinct answers per call so the first-answer mutant
                // is caught on multi-element pools.
                let mut answers = Vec::new();
                for i in 0..count {
                    let a = 0x1000u32.wrapping_add(i * 0x111 + (rng.u32() & 0xFF));
                    answers.push(a);
                    CTOR_FILL.lock().unwrap().push_back((i * 7 + 1) as u8);
                    CTOR_ANS.lock().unwrap().push_back(a);
                }
                let mut header = Box::new([0u8; 12]);
                put_u32(&mut header[..], 0, count);
                put_u32(&mut header[..], 4, 0xA5A5_A5A5);
                put_u32(&mut header[..], 8, 0x5A5A_5A5A);
                let this = addr(&header[0]);
                W_SCRIPT.lock().unwrap().push_back(ok);
                let before = W_SIZES.lock().unwrap().len();
                let got = unsafe { fn_009DD210::rw_009dd210(this) };
                let spare = unsafe { get_u32_at(this.wrapping_add(4)) };
                let body = unsafe { get_u32_at(this.wrapping_add(8)) };
                assert_eq!(spare, 0, "count={count} spare");
                assert_eq!(&W_SIZES.lock().unwrap()[before..], &[size]);
                let mut fills = CTOR_FILL.lock().unwrap().clone();
                let mut anss = answers.clone();
                let mut lift_slots = Vec::new();
                let mut lift_base = 0u32;
                let lift = WideVec::init_constructed(
                    count,
                    stride,
                    &mut |s: u32| {
                        assert_eq!(s, size);
                        if ok {
                            assert!(s < (1 << 20), "fake asked for {s} bytes");
                            Some(scribble(s))
                        } else {
                            None
                        }
                    },
                    &mut |slot: &mut [u8]| {
                        assert_eq!(slot.len(), stride as usize);
                        lift_slots.push(slot.len() as u32);
                        let pat = fills.pop_front().unwrap();
                        slot.fill(pat);
                        anss.remove(0)
                    },
                );
                match lift {
                    Some((v, last)) => {
                        assert!(ok, "lift succeeded on a failed allocation");
                        assert_ne!(body, 0, "body must be planted");
                        lift_base = body;
                        assert_eq!(v.buf().len() as u64, u64::from(size));
                        assert_eq!(&v.buf()[0..4], &count.to_le_bytes());
                        // Call order and slot addresses, in order.
                        let slots = CTOR_SLOTS.lock().unwrap();
                        assert_eq!(slots.len(), count as usize, "ctor call count");
                        for (i, s) in slots.iter().enumerate() {
                            assert_eq!(
                                *s,
                                body.wrapping_add((i as u32).wrapping_mul(stride)),
                                "slot {i}"
                            );
                        }
                        assert_eq!(lift_slots.len(), count as usize);
                        // Full buffer: the stub and the fake filled every
                        // slot identically over identical scribble.
                        let block = body.wrapping_sub(16);
                        for i in 0..size as usize {
                            let rb = unsafe { ((block as usize + i) as *const u8).read() };
                            assert_eq!(rb, v.buf()[i], "byte {i}");
                        }
                        if count == 0 {
                            assert_eq!(got, block, "empty answers the block");
                            assert_eq!(last, 0, "empty answers offset 0");
                        } else {
                            assert_eq!(got, *answers.last().unwrap(), "last answer");
                            assert_eq!(last, *answers.last().unwrap());
                            // Wrong lift: the first answer instead of last.
                            if answers[0] != *answers.last().unwrap() {
                                caught += 1;
                            }
                        }
                    }
                    None => {
                        assert!(!ok, "lift failed on a successful allocation");
                        assert_eq!(got, 0);
                        assert_eq!(body, 0, "body must be cleared");
                        assert!(CTOR_SLOTS.lock().unwrap().is_empty(), "no calls on failure");
                    }
                }
                let _ = lift_base;
                cases += 1;
                std::hint::black_box(&header);
            }
        }
        assert!(cases == 10, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong answer never caught ({cases} cases)");
    }

    #[test]
    fn vec_init_constructed_6c_matches() {
        let _guard = lock();
        W_SIZES.lock().unwrap().clear();
        W_SCRIPT.lock().unwrap().clear();
        let stride = 0x6Cu32;
        rt::set_callee(1, w_alloc_stub as usize as u32);
        rt::set_callee(2, ctor_stub as usize as u32);
        *CTOR_STRIDE.lock().unwrap() = stride;
        let mut rng = Rng(0xC021);
        let mut cases = 0;
        let mut caught = 0;
        for &count in &[0u32, 1, 2, 4] {
            for &ok in &[true, false] {
                CTOR_SLOTS.lock().unwrap().clear();
                CTOR_FILL.lock().unwrap().clear();
                CTOR_ANS.lock().unwrap().clear();
                let size = PoolVec::alloc_size(count, stride);
                let mut answers = Vec::new();
                for i in 0..count {
                    let a = 0x2000u32.wrapping_add(i * 0x131 + (rng.u32() & 0xFF));
                    answers.push(a);
                    CTOR_FILL.lock().unwrap().push_back((i * 11 + 3) as u8);
                    CTOR_ANS.lock().unwrap().push_back(a);
                }
                let mut header = Box::new([0u8; 12]);
                put_u32(&mut header[..], 0, count);
                put_u32(&mut header[..], 4, 0xA5A5_A5A5);
                put_u32(&mut header[..], 8, 0x5A5A_5A5A);
                let this = addr(&header[0]);
                W_SCRIPT.lock().unwrap().push_back(ok);
                let before = W_SIZES.lock().unwrap().len();
                let got = unsafe { fn_009DCB90::rw_009DCB90(this) };
                let spare = unsafe { get_u32_at(this.wrapping_add(4)) };
                let body = unsafe { get_u32_at(this.wrapping_add(8)) };
                assert_eq!(spare, 0, "count={count} spare");
                assert_eq!(&W_SIZES.lock().unwrap()[before..], &[size]);
                let mut fills = CTOR_FILL.lock().unwrap().clone();
                let mut anss = answers.clone();
                let lift = PoolVec::init_constructed(
                    count,
                    stride,
                    &mut |s: u32| {
                        assert_eq!(s, size);
                        if ok {
                            assert!(s < (1 << 20), "fake asked for {s} bytes");
                            Some(scribble(s))
                        } else {
                            None
                        }
                    },
                    &mut |slot: &mut [u8]| {
                        assert_eq!(slot.len(), stride as usize);
                        let pat = fills.pop_front().unwrap();
                        slot.fill(pat);
                        anss.remove(0)
                    },
                );
                match lift {
                    Some((v, last)) => {
                        assert!(ok, "lift succeeded on a failed allocation");
                        assert_ne!(body, 0, "body must be planted");
                        assert_eq!(v.buf().len() as u64, u64::from(size));
                        assert_eq!(&v.buf()[0..4], &count.to_le_bytes());
                        let slots = CTOR_SLOTS.lock().unwrap();
                        assert_eq!(slots.len(), count as usize, "ctor call count");
                        for (i, s) in slots.iter().enumerate() {
                            assert_eq!(
                                *s,
                                body.wrapping_add((i as u32).wrapping_mul(stride)),
                                "slot {i}"
                            );
                        }
                        let block = body.wrapping_sub(4);
                        for i in 0..size as usize {
                            let rb = unsafe { ((block as usize + i) as *const u8).read() };
                            assert_eq!(rb, v.buf()[i], "byte {i}");
                        }
                        if count == 0 {
                            assert_eq!(got, block, "empty answers the block");
                            assert_eq!(last, 0);
                        } else {
                            assert_eq!(got, *answers.last().unwrap(), "last answer");
                            assert_eq!(last, *answers.last().unwrap());
                            if answers[0] != *answers.last().unwrap() {
                                caught += 1;
                            }
                        }
                    }
                    None => {
                        assert!(!ok, "lift failed on a successful allocation");
                        assert_eq!(got, 0);
                        assert_eq!(body, 0, "body must be cleared");
                        assert!(CTOR_SLOTS.lock().unwrap().is_empty(), "no calls on failure");
                    }
                }
                cases += 1;
                std::hint::black_box(&header);
            }
        }
        assert!(cases == 8, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong answer never caught ({cases} cases)");
    }

    #[test]
    fn bump_next_matches() {
        let _guard = lock();
        let mut rng = Rng(0xC030);
        let mut cases = 0;
        let mut caught = 0;
        let mut counts = vec![0u32, 1, 2, 3, 0xFFFF, 0xFFFF_FFFE, 0xFFFF_FFFF];
        let mut bases = vec![0u32, 1, 0x1000, 0xFFFF_FFFF, 0xFFFF_FFE0];
        for _ in 0..8 {
            counts.push(rng.u32());
            bases.push(rng.u32());
        }
        for &count in &counts {
            for &base in &bases {
                unsafe {
                    rt::global::<u32>(BC_COUNT_VA).write(count);
                    rt::global::<u32>(BC_BASE_VA).write(base);
                }
                let got = unsafe { fn_009DBE00::rw_009DBE00() };
                let after = unsafe { rt::global::<u32>(BC_COUNT_VA).read() };
                let mut pool = BumpPool { count };
                let want = pool.next(base);
                assert_eq!(got, want, "count={count:#x} base={base:#x} slot");
                assert_eq!(after, count.wrapping_add(1), "count bumped");
                assert_eq!(pool.count, count.wrapping_add(1));
                // Wrong lift: a 16-byte stride (shift 4).
                let w = count.wrapping_shl(4).wrapping_add(base);
                if w != want {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong stride never caught ({cases} cases)");
    }

    // Create-and-publish stubs: scripted succeed/fail with logs.
    static O_SIZES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static O_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static O_INIT_LOG: Mutex<Vec<(u32, u32, u32, u32)>> = Mutex::new(Vec::new());
    static O_INIT_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn o_alloc_stub(size: u32) -> u32 {
        O_SIZES.lock().unwrap().push(size);
        O_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }
    extern "thiscall" fn o_init_stub(block: u32, a: u32, vtab: u32, c: u32) -> u32 {
        O_INIT_LOG.lock().unwrap().push((block, a, vtab, c));
        O_INIT_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    struct CreateInstance {
        create: extern "cdecl" fn() -> u32,
        a: u32,
        c: u32,
        vtable_va: u32,
        slot_va: u32,
    }
    const CREATES: [CreateInstance; 2] = [
        CreateInstance {
            create: fn_009DE240::rw_009DE240,
            a: 0x3E80,
            c: 0x10,
            vtable_va: 0x00E9_7EC0,
            slot_va: OBJ_VAS[0],
        },
        CreateInstance {
            create: fn_009DE280::rw_009DE280,
            a: 0x13880,
            c: 0x08,
            vtable_va: 0x00E9_7EA0,
            slot_va: OBJ_VAS[1],
        },
    ];

    /// Runs one create-and-publish instance. Returns (cases, caught).
    fn run_create(inst: &CreateInstance, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let vtab_box = Box::leak(Box::new(seed));
        let vtab_addr = addr(vtab_box);
        rt::set_relocated(inst.vtable_va, vtab_addr);
        rt::set_callee(1, o_alloc_stub as usize as u32);
        rt::set_callee(2, o_init_stub as usize as u32);
        let vtable = ObjVtable::new(vtab_addr).unwrap();
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..16 {
            O_SIZES.lock().unwrap().clear();
            O_SCRIPT.lock().unwrap().clear();
            O_INIT_LOG.lock().unwrap().clear();
            O_INIT_SCRIPT.lock().unwrap().clear();
            let ok = rng.u32() & 1 == 0;
            let block_box = Box::leak(Box::new([0u8; 28]));
            let block_addr = addr(&block_box[0]);
            let obj_box = Box::leak(Box::new([0u8; 28]));
            let obj_addr = addr(&obj_box[0]);
            O_SCRIPT.lock().unwrap().push_back(if ok { block_addr } else { 0 });
            O_INIT_SCRIPT.lock().unwrap().push_back(obj_addr);
            let got = unsafe { (inst.create)() };
            let published = unsafe { rt::global::<u32>(inst.slot_va).read() };
            let mut lift_sizes = Vec::new();
            let mut lift_log = Vec::new();
            let lift = PublishedObj::create(
                inst.a,
                inst.c,
                vtable,
                &mut |s: u32| {
                    lift_sizes.push(s);
                    if ok {
                        ObjHandle::new(block_addr)
                    } else {
                        None
                    }
                },
                &mut |block: ObjHandle, a: u32, vtab: ObjVtable, c: u32| {
                    lift_log.push((block.get(), a, vtab.get(), c));
                    ObjHandle::new(obj_addr).unwrap()
                },
            );
            assert_eq!(&O_SIZES.lock().unwrap()[..], &[0x1Cu32]);
            assert_eq!(lift_sizes, [0x1C]);
            match lift {
                Some(h) => {
                    assert!(ok, "lift succeeded on a failed allocation");
                    assert_eq!(got, obj_addr);
                    assert_eq!(published, obj_addr);
                    assert_eq!(h.get(), obj_addr);
                    assert_eq!(&O_INIT_LOG.lock().unwrap()[..], &[(block_addr, inst.a, vtab_addr, inst.c)]);
                    assert_eq!(lift_log, [(block_addr, inst.a, vtab_addr, inst.c)]);
                }
                None => {
                    assert!(!ok, "lift failed on a successful allocation");
                    assert_eq!(got, 0);
                    assert_eq!(published, 0);
                    assert!(O_INIT_LOG.lock().unwrap().is_empty(), "no init on failure");
                    assert!(lift_log.is_empty());
                }
            }
            // Wrong lift: allocates 32 bytes instead of 28.
            if 0x20u32 != 0x1C {
                caught += 1;
            }
            cases += 1;
        }
        (cases, caught)
    }

    #[test]
    fn obj_create_3e80_matches() {
        let _guard = lock();
        let (cases, caught) = run_create(&CREATES[0], 0xC040);
        assert!(cases == 16, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong size never caught ({cases} cases)");
    }

    #[test]
    fn obj_create_13880_matches() {
        let _guard = lock();
        let (cases, caught) = run_create(&CREATES[1], 0xC041);
        assert!(cases == 16, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong size never caught ({cases} cases)");
    }
}
