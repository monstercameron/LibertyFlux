//! Differential cases, part 3: the pool-vector initialiser against its
//! thirteen instances.
//!
//! Each case plants one stamp address, runs the rewrite and
//! [`PoolVec::init`] on the same count, and compares the return (the end
//! address against buffer base plus length), the allocation size, the
//! count prefix, every stamp word, and the header spare/body words. A
//! deliberately wrong lift (forgetting the count prefix in the size) must
//! be caught. 32-bit target only.

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
    use lf_world::pools::{ElemStamp, PoolVec};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock, put_u32};

    /// One initialiser instance.
    type InitFn = extern "thiscall" fn(u32) -> u32;
    struct Instance {
        init: InitFn,
        stride: u32,
        va: u32,
    }
    const INSTANCES: [Instance; 13] = [
        Instance { init: fn_009DC960::rw_009DC960, stride: 0x24, va: 0x00E9_791C },
        Instance { init: fn_009DC9D0::rw_009DC9D0, stride: 0x10, va: 0x00E9_769C },
        Instance { init: fn_009DCA40::rw_009DCA40, stride: 0x38, va: 0x00E9_771C },
        Instance { init: fn_009DCAB0::rw_009DCAB0, stride: 0x30, va: 0x00E9_759C },
        Instance { init: fn_009DCB20::rw_009DCB20, stride: 0x38, va: 0x00E9_789C },
        Instance { init: fn_009DCC00::rw_009DCC00, stride: 0x4C, va: 0x00E9_7A1C },
        Instance { init: fn_009DCC70::rw_009DCC70, stride: 0x3C, va: 0x00E9_751C },
        Instance { init: fn_009DCCE0::rw_009DCCE0, stride: 0x3C, va: 0x00E9_779C },
        Instance { init: fn_009DCD50::rw_009DCD50, stride: 0xA8, va: 0x00E9_7A9C },
        Instance { init: fn_009DCDC0::rw_009DCDC0, stride: 0x20, va: 0x00E9_799C },
        Instance { init: fn_009DCE30::rw_009DCE30, stride: 0x24, va: 0x00E9_761C },
        Instance { init: fn_009DCEA0::rw_009DCEA0, stride: 0x1C, va: 0x00E9_7B1C },
        Instance { init: fn_009DCF10::rw_009DCF10, stride: 0x78, va: 0x00E9_781C },
    ];

    /// Deliberately wrong size: forgets the count prefix. Must be caught
    /// wherever the true size is not already saturated.
    fn wrong_size(count: u32, stride: u32) -> u32 {
        (count as u64)
            .wrapping_mul(stride as u64)
            .min(u64::from(u32::MAX)) as u32
    }

    // Self-backing allocator stub: scripted succeed/fail per call, with a
    // log of requested sizes. Backing is scribbled so no test can mistake
    // it for zeros; only the count prefix and the stamps are compared.
    static VEC_ARENA: Mutex<Vec<Box<[u8]>>> = Mutex::new(Vec::new());
    static VEC_SCRIPT: Mutex<VecDeque<bool>> = Mutex::new(VecDeque::new());
    static VEC_SIZES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "cdecl" fn vec_alloc_stub(size: u32) -> u32 {
        VEC_SIZES.lock().unwrap().push(size);
        if !VEC_SCRIPT.lock().unwrap().pop_front().unwrap_or(false) {
            return 0;
        }
        assert!(size < (1 << 20), "stub asked for {size} bytes");
        let mut backing = vec![0xCDu8; size as usize];
        for (i, b) in backing.iter_mut().enumerate() {
            *b ^= (i & 0xFF) as u8;
        }
        let a = addr(&backing[0]);
        VEC_ARENA.lock().unwrap().push(backing.into_boxed_slice());
        a
    }

    /// Reads one word of test memory the rewrite wrote.
    unsafe fn get_u32_at(addr: u32) -> u32 {
        unsafe { (addr as *const u32).read_unaligned() }
    }

    /// Runs one instance over counts and outcomes. Returns (comparisons, caught).
    fn run_instance(inst: &Instance, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        // A distinct stamp address for this instance.
        let stamp_box = Box::leak(Box::new(seed));
        let stamp_addr = addr(stamp_box);
        rt::set_relocated(inst.va, stamp_addr);
        rt::set_callee(1, vec_alloc_stub as usize as u32);
        let stamp = ElemStamp::new(stamp_addr).unwrap();
        let mut cases = 0;
        let mut caught = 0;
        // Small counts run with both outcomes; saturating counts only
        // fail (no test backs gigabytes).
        let mut small = vec![0u32, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 100];
        for _ in 0..12 {
            small.push(rng.below(200));
        }
        let max = u64::from(u32::MAX);
        let stride64 = u64::from(inst.stride);
        let q = ((max - 4) / stride64).min(max) as u32;
        let big = [
            q.saturating_sub(1),
            q,
            q.saturating_add(1).min(u32::MAX),
            (max / stride64).min(max) as u32,
            u32::MAX - 1,
            u32::MAX,
        ];
        let mut run = |count: u32, ok: bool, cases: &mut u32, caught: &mut u32| {
            let size = PoolVec::alloc_size(count, inst.stride);
            let mut header = Box::new([0u8; 12]);
            put_u32(&mut header[..], 0, count);
            // Spare and body start as garbage the rewrite must overwrite.
            put_u32(&mut header[..], 4, 0xA5A5_A5A5);
            put_u32(&mut header[..], 8, 0x5A5A_5A5A);
            let this = addr(&header[0]);
            VEC_SCRIPT.lock().unwrap().push_back(ok);
            let before = VEC_SIZES.lock().unwrap().len();
            let got = unsafe { (inst.init)(this) };
            // Raw reads: the rewrite wrote through the address, invisibly
            // to the borrow checker.
            let spare = unsafe { get_u32_at(this.wrapping_add(4)) };
            let body = unsafe { get_u32_at(this.wrapping_add(8)) };
            assert_eq!(spare, 0, "count={count} stride={} spare", inst.stride);
            let mut lift_sizes = Vec::new();
            let lift = PoolVec::init(count, inst.stride, stamp, &mut |s: u32| {
                lift_sizes.push(s);
                if ok {
                    assert!(s < (1 << 20), "fake asked for {s} bytes");
                    Some(vec![0u8; s as usize])
                } else {
                    None
                }
            });
            assert_eq!(&VEC_SIZES.lock().unwrap()[before..], &[size]);
            assert_eq!(lift_sizes, [size], "count={count}");
            match lift {
                Some(v) => {
                    assert!(ok, "lift succeeded on a failed allocation");
                    assert_eq!(v.buf().len() as u64, u64::from(size));
                    assert_eq!(v.count(), count);
                    assert_eq!(v.stride(), inst.stride);
                    assert_ne!(body, 0, "body must be planted");
                    assert_eq!(got, body.wrapping_add((v.end_offset() - 4) as u32));
                    // Count prefix and every stamp on both sides.
                    assert_eq!(unsafe { get_u32_at(body.wrapping_sub(4)) }, count);
                    assert_eq!(&v.buf()[0..4], &count.to_le_bytes());
                    for i in 0..count {
                        let at = body.wrapping_add(i.wrapping_mul(inst.stride));
                        assert_eq!(unsafe { get_u32_at(at) }, stamp_addr, "stamp {i}");
                        let off = 4 + (i as usize) * (inst.stride as usize);
                        assert_eq!(&v.buf()[off..off + 4], &stamp_addr.to_le_bytes());
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
            // Big sizes only fail; skip any that would fit (none do, but
            // the formula is what is pinned, not the backing).
            if PoolVec::alloc_size(count, inst.stride) < (1 << 20) {
                run(count, true, &mut cases, &mut caught);
            }
            run(count, false, &mut cases, &mut caught);
        }
        (cases, caught)
    }

    macro_rules! vec_test {
        ($name:ident, $idx:expr, $seed:expr) => {
            #[test]
            fn $name() {
                let _guard = lock();
                VEC_SIZES.lock().unwrap().clear();
                VEC_SCRIPT.lock().unwrap().clear();
                let (cases, caught) = run_instance(&INSTANCES[$idx], $seed);
                assert!(cases > 60, "too few comparisons ({cases})");
                assert!(caught > 0, "wrong size never caught ({cases} cases)");
            }
        };
    }

    vec_test!(vec_init_0x24_791c_matches, 0, 0xB001);
    vec_test!(vec_init_0x10_769c_matches, 1, 0xB002);
    vec_test!(vec_init_0x38_771c_matches, 2, 0xB003);
    vec_test!(vec_init_0x30_759c_matches, 3, 0xB004);
    vec_test!(vec_init_0x38_789c_matches, 4, 0xB005);
    vec_test!(vec_init_0x4c_7a1c_matches, 5, 0xB006);
    vec_test!(vec_init_0x3c_751c_matches, 6, 0xB007);
    vec_test!(vec_init_0x3c_779c_matches, 7, 0xB008);
    vec_test!(vec_init_0xa8_7a9c_matches, 8, 0xB009);
    vec_test!(vec_init_0x20_799c_matches, 9, 0xB00A);
    vec_test!(vec_init_0x24_761c_matches, 10, 0xB00B);
    vec_test!(vec_init_0x1c_7b1c_matches, 11, 0xB00C);
    vec_test!(vec_init_0x78_781c_matches, 12, 0xB00D);
}




