//! Differential cases, part 6: rebuilding the raw-bool bit array.
//!
//! The lifted packer against its verified rewrite on the same generated
//! inputs, comparing the return value, the pair words and every packed
//! byte. The rewrite frees its old buffer through the thread allocator
//! and installs the new one through an allocator helper: the harness
//! plants a slot-0 chain answering a recording free stub and registers
//! an allocating stub that mirrors the checker's model (writes the pair
//! pointer only), so this case proves the zeroing, the size, the packing
//! order and the last-sample clamp. A deliberately wrong lift must be
//! caught at least once. 32-bit target only.
//!
//! One test function: the planted chain and the stubs are per-binary
//! state.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_animation::channel::RawBool;
    use lf_animchan_diff::rewrites::*;
    use lf_animchan_diff::rt::{plant_tls0, set_callee1};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{RawObj, Rng, addr};

    /// Last pointer handed to the free stub, and its call count.
    static FREED: AtomicU32 = AtomicU32::new(0);
    static FREE_CALLS: AtomicU32 = AtomicU32::new(0);
    /// Last buffer the allocator stub installed, its size, its call
    /// count, and a fill counter so no two buffers start alike.
    static LAST_BUF: AtomicU32 = AtomicU32::new(0);
    static LAST_SIZE: AtomicU32 = AtomicU32::new(0);
    static ALLOC_CALLS: AtomicU32 = AtomicU32::new(0);
    static ALLOC_FILL: AtomicU32 = AtomicU32::new(0);

    /// Callee-2 stand-in, reached through the planted chain: records the
    /// released buffer. The test's boxes own the memory either way.
    extern "thiscall" fn free_stub(_heap: u32, old: u32) -> u32 {
        FREE_CALLS.fetch_add(1, Ordering::SeqCst);
        FREED.store(old, Ordering::SeqCst);
        0
    }

    /// Callee-1 stand-in: installs a fresh buffer at the pair, exactly
    /// the checker's model (the pointer word only), and records it.
    extern "thiscall" fn alloc_stub(pair: u32, size: u32) -> u32 {
        ALLOC_CALLS.fetch_add(1, Ordering::SeqCst);
        let fill = ALLOC_FILL.fetch_add(1, Ordering::SeqCst) as u8;
        let buf = vec![fill; size as usize].leak();
        let ptr = buf.as_ptr() as usize as u32;
        LAST_BUF.store(ptr, Ordering::SeqCst);
        LAST_SIZE.store(size, Ordering::SeqCst);
        unsafe {
            (pair as *mut u32).write(ptr);
        }
        ptr
    }

    /// Packs most-significant-bit first: wrong on every asymmetric byte.
    fn wrong_msb_first(samples: &[u8]) -> Vec<u8> {
        let n = samples.len();
        let size = (n >> 3) + usize::from((n & 7) != 0);
        if n == 0 {
            return Vec::new();
        }
        let last = n - 1;
        let mut edi = 0usize;
        let mut out = vec![0u8; size];
        for b in out.iter_mut() {
            let mut cur = 0u8;
            for bit in 0..8u32 {
                if samples[edi] != 0 {
                    cur |= 1 << (7 - bit);
                }
                let nx = edi + 1;
                edi = if nx < last { nx } else { last };
            }
            *b = cur;
        }
        out
    }

    #[test]
    fn raw_bool_build_matches() {
        plant_tls0(free_stub as usize as u32);
        set_callee1(alloc_stub as usize as u32);
        let mut rng = Rng(0xB017);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |samples: &[u8], with_old: bool, caught: &mut u32| {
            let boxed: Box<[u8]> = samples.to_vec().into_boxed_slice();
            let samples_ptr = if boxed.is_empty() {
                // Unread when the count is zero.
                0xDEAD_0001
            } else {
                addr(&boxed[0])
            };
            let old_box: Option<Box<[u8; 16]>> =
                with_old.then(|| Box::new([0x5A; 16]));
            let old_ptr = old_box.as_ref().map(|b| addr(&b[0])).unwrap_or(0);
            let obj = Box::new(RawObj {
                vtable: 0x6666_6666,
                hdr: [0x77, 0x88, 0x99, 0xAA],
                keys: old_ptr,
                count: 0xBEEF,
                gate: 0x1234,
            });
            FREE_CALLS.store(0, Ordering::SeqCst);
            ALLOC_CALLS.store(0, Ordering::SeqCst);
            FREED.store(0xDEAD_DEAD, Ordering::SeqCst);
            let r = unsafe {
                fn_0069B170::rw_0069B170(addr(&*obj), samples_ptr, samples.len() as u32)
            };
            assert_eq!(r, 1, "build always answers 1");
            // The old buffer is released exactly when non-null.
            assert_eq!(
                FREE_CALLS.load(Ordering::SeqCst),
                u32::from(with_old),
                "free calls"
            );
            if with_old {
                assert_eq!(FREED.load(Ordering::SeqCst), old_ptr, "freed pointer");
            }
            // Untouched words keep their sentinels.
            assert_eq!(obj.vtable, 0x6666_6666);
            assert_eq!(obj.hdr, [0x77, 0x88, 0x99, 0xAA]);
            // The pair zeroing is a whole word at +12: count and gate.
            assert_eq!(obj.count, 0, "pair count stays zeroed");
            assert_eq!(obj.gate, 0, "gate zeroed with the pair");
            let lift = RawBool::build_from_samples(samples);
            if samples.is_empty() {
                assert_eq!(ALLOC_CALLS.load(Ordering::SeqCst), 0, "no buffer for none");
                assert_eq!(obj.keys, 0, "pair pointer stays null");
                assert!(lift.bytes().is_empty());
            } else {
                assert_eq!(ALLOC_CALLS.load(Ordering::SeqCst), 1, "one install");
                assert_eq!(obj.keys, LAST_BUF.load(Ordering::SeqCst), "pair takes buffer");
                let size = LAST_SIZE.load(Ordering::SeqCst) as usize;
                assert_eq!(size, lift.bytes().len(), "installed size");
                let got =
                    unsafe { core::slice::from_raw_parts(LAST_BUF.load(Ordering::SeqCst) as *const u8, size) };
                assert_eq!(got, lift.bytes(), "packed bytes");
            }
            if wrong_msb_first(samples) != lift.bytes() {
                *caught += 1;
            }
            cases += 1;
        };
        // Deterministic asymmetric case: the wrong order is caught here
        // whatever the generator does.
        run(&[1, 0, 0, 0, 0, 0, 0, 0], true, &mut caught);
        // Every small count in full, shapes around the byte edges.
        for count in 0..=40usize {
            let zeros = vec![0u8; count];
            let ones = vec![0xFFu8; count];
            let alt: Vec<u8> = (0..count).map(|i| (i % 2) as u8).collect();
            let mut first_one = vec![0u8; count];
            if count > 0 {
                first_one[0] = 1;
            }
            let mut last_one = vec![0u8; count];
            if count > 0 {
                last_one[count - 1] = 7;
            }
            let rnd: Vec<u8> = (0..count).map(|_| (rng.u32() & 0xFF) as u8).collect();
            for (i, shape) in [zeros, ones, alt, first_one, last_one, rnd]
                .iter()
                .enumerate()
            {
                run(shape, i % 2 == 0, &mut caught);
            }
        }
        // Wider counts, random bytes.
        for &count in &[63usize, 64, 65, 100, 255, 256] {
            let rnd: Vec<u8> = (0..count).map(|_| (rng.u32() & 0xFF) as u8).collect();
            run(&rnd, true, &mut caught);
            run(&rnd, false, &mut caught);
        }
        assert!(caught > 0, "wrong pack order never caught ({cases} cases)");
    }
}
