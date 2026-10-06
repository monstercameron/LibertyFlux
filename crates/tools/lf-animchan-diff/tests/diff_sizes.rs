//! Differential cases, part 3: storage sizes of the compressed channels.
//!
//! Each lifted size against its verified rewrite on the same generated
//! inputs, comparing the return value bit for bit. Each case also runs a
//! deliberately wrong lift, which must be caught at least once. 32-bit
//! target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_animation::channel::{CurveFloat, CurveKey, DeltaFloat, QuantizeFloat, RleInt};
    use lf_animchan_diff::rewrites::*;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{CurveKeyRec, CurveObj, Rng, U32_EDGE, addr, blob64, put_u16, put_u32};

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_animation::channel::{CurveFloat, DeltaFloat, RleInt};

        /// Forgets the round-up: drops a partial word.
        pub fn delta_trunc(c: &DeltaFloat) -> u32 {
            (c.count() >> 5).wrapping_mul(4).wrapping_add(0x38)
        }

        /// Swaps the two headers.
        pub fn quant_swap(count: u32, width: u32) -> u32 {
            let n = count.wrapping_mul(width);
            let header = if (n & 31) != 0 { 0x1cu32 } else { 0x20u32 };
            (n >> 5).wrapping_mul(4).wrapping_add(header)
        }

        /// Forgets the per-sample words.
        pub fn rle_no_samples(c: &RleInt) -> u32 {
            let words = (c.bit_len() >> 5) + u32::from((c.bit_len() & 0x1F) != 0);
            words.wrapping_mul(4).wrapping_add(0x1C)
        }

        /// Wrong segment tail per key.
        pub fn curve_tail8(c: &CurveFloat) -> u32 {
            let mut size = 0x18u32;
            for k in c.keys() {
                size = size
                    .wrapping_add((u32::from(k.order())).wrapping_mul(4))
                    .wrapping_add(0x08);
            }
            size
        }
    }

    /// Edge counts around the word boundaries, plus the shared edge words.
    fn count_edges() -> Vec<u32> {
        let mut v = U32_EDGE.to_vec();
        v.extend([
            30, 31, 32, 33, 63, 64, 65, 0x3F, 0x40, 0x7FFF, 0xFFFF, 0x1_0000,
        ]);
        v
    }

    #[test]
    fn delta_storage_size_matches() {
        let mut rng = Rng(0xDE17);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |n: u32, caught: &mut u32| {
            let mut obj = blob64();
            put_u32(&mut obj, 0x24, n);
            let got = unsafe { fn_006996C0::rw_006996c0(addr(&obj[0])) };
            let lift = DeltaFloat::new(n);
            assert_eq!(got, lift.storage_size(), "count {n:#x}");
            if wrong::delta_trunc(&lift) != got {
                *caught += 1;
            }
            cases += 1;
        };
        for &n in &count_edges() {
            run(n, &mut caught);
        }
        for _ in 0..64 {
            run(rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong delta size never caught ({cases} cases)");
    }

    #[test]
    fn quant_storage_size_matches() {
        let mut rng = Rng(0x9A47);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |count: u32, width: u32, caught: &mut u32| {
            let mut obj = blob64();
            put_u32(&mut obj, 0x10, count);
            put_u32(&mut obj, 0x0C, width);
            let got = unsafe { fn_00699BB0::rw_00699bb0(addr(&obj[0])) };
            let lift = QuantizeFloat::new(1.0, 0.0).with_counts(count, width);
            assert_eq!(
                got,
                lift.storage_size(),
                "count {count:#x} width {width:#x}"
            );
            if wrong::quant_swap(count, width) != got {
                *caught += 1;
            }
            cases += 1;
        };
        for &count in &count_edges() {
            for &width in &[0u32, 1, 2, 7, 16, 20, 31, 32] {
                run(count, width, &mut caught);
            }
        }
        for _ in 0..64 {
            run(rng.u32(), rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong quant size never caught ({cases} cases)");
    }

    #[test]
    fn rle_alloc_size_matches() {
        let mut rng = Rng(0x81E0);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |count: u16, bit_len: u32, caught: &mut u32| {
            let mut obj = blob64();
            put_u16(&mut obj, 0x0C, count);
            put_u32(&mut obj, 0x14, bit_len);
            let got = unsafe { fn_0069D980::rw_0069D980(addr(&obj[0])) };
            let lift = RleInt::new(count, bit_len);
            assert_eq!(got, lift.alloc_size(), "count {count:#x} bits {bit_len:#x}");
            if wrong::rle_no_samples(&lift) != got {
                *caught += 1;
            }
            cases += 1;
        };
        for &count in &[0u16, 1, 2, 0x7FFF, 0x8000, 0xFFFE, 0xFFFF] {
            for &bit_len in &count_edges() {
                run(count, bit_len, &mut caught);
            }
        }
        for _ in 0..64 {
            run((rng.u32() & 0xFFFF) as u16, rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong rle size never caught ({cases} cases)");
    }

    /// Builds a 32-bit key array with the given order bytes; returns the
    /// boxed records (which must stay alive) and the lift keys.
    fn curve_keys(
        keys: &[u16],
        orders: &[u8],
    ) -> (Box<[CurveKeyRec]>, Vec<CurveKey>, Vec<Box<[f32]>>) {
        assert_eq!(keys.len(), orders.len());
        let mut coeff_blocks: Vec<Box<[f32]>> = Vec::new();
        let mut recs: Vec<CurveKeyRec> = Vec::new();
        let mut lift_keys: Vec<CurveKey> = Vec::new();
        for (k, o) in keys.iter().zip(orders.iter()) {
            // Coefficients are unread by the size; one zeroed block each
            // keeps the pointers live and distinct.
            let block: Box<[f32]> = vec![0.0; (*o as usize) + 1].into_boxed_slice();
            let ptr = addr(&block[0]);
            coeff_blocks.push(block);
            recs.push(CurveKeyRec {
                key: *k,
                order: *o,
                pad: 0,
                coeff: ptr,
            });
            lift_keys.push(CurveKey::new(*k, *o, vec![0.0; (*o as usize) + 1]));
        }
        (recs.into_boxed_slice(), lift_keys, coeff_blocks)
    }

    #[test]
    fn curve_alloc_size_matches() {
        let mut rng = Rng(0xC428);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |keys: &[u16], orders: &[u8], caught: &mut u32| {
            let (recs, lift_keys, _blocks) = curve_keys(keys, orders);
            let base = if recs.is_empty() { 0 } else { addr(&recs[0]) };
            let obj = Box::new(CurveObj {
                vtable: 0x3333_3333,
                hdr: [0x44, 0x55, 0x66, 0x77],
                base,
                count: keys.len() as u16,
                cap: keys.len() as u16,
                scale: 0x3F80_0000,
                bias: 0,
            });
            let got = unsafe { fn_0069C7D0::rw_0069C7D0(addr(&*obj)) };
            let lift = CurveFloat::new(lift_keys, 1.0, 0.0);
            assert_eq!(got, lift.alloc_size(), "orders {orders:?}");
            if wrong::curve_tail8(&lift) != got {
                *caught += 1;
            }
            cases += 1;
        };
        // Empty channel: the base alone.
        run(&[], &[], &mut caught);
        // One key of every small order plus the widest byte.
        for &o in &[0u8, 1, 2, 3, 4, 5, 8, 16, 254, 255] {
            run(&[7], &[o], &mut caught);
        }
        // Wrapping: enough wide orders to overflow 32 bits.
        run(&[0u16; 32], &[255u8; 32], &mut caught);
        for _ in 0..48 {
            let n = (rng.u32() % 6) as usize;
            let keys: Vec<u16> = (0..n).map(|_| (rng.u32() & 0xFFFF) as u16).collect();
            let orders: Vec<u8> = (0..n).map(|_| (rng.u32() & 0xFF) as u8).collect();
            run(&keys, &orders, &mut caught);
        }
        assert!(caught > 0, "wrong curve size never caught ({cases} cases)");
    }
}
