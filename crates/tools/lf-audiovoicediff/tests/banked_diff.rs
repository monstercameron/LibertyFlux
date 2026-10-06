//! Differential cases, part 2: the banked voice table.
//!
//! Each case plants the scale/table globals, builds a bank table and its
//! record stores, runs the rewrite and the lift on the same selector, and
//! compares the return and every store byte. Record addresses are rebuilt
//! from the lifted indexes per case, so the translation is proven, not
//! assumed. Each method has a deliberately wrong lift that must be
//! caught. 32-bit target only.
//!
//! Proven scales keep records disjoint (0xEC and up); aliasing scales
//! cannot survive the address-to-index narrowing and are out of domain
//! (see the registry).

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_voice::banked::{BankRecord, BankedVoices, VoiceSel};
    use lf_audiovoicediff::rewrites::*;
    use lf_audiovoicediff::rt::{self};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, SCALE_VA, TABLE_VA, addr, lock};

    /// Bank-table stride and biases, as the rewrites hold them.
    const STRIDE: u32 = 0x6F40;
    /// First bank entry's bias from the table base.
    const TABLE_BIAS: u32 = 0x6F14;
    /// Slot word offset from the record start.
    const SLOT_OFF: u32 = 0xE0;
    /// Flag byte offset from the record start.
    const FLAG_OFF: u32 = 0xE8;

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    /// One planted banked table: the table image plus its record stores.
    struct Fixture {
        table: Box<[u8]>,
        stores: Vec<Box<[u8]>>,
        scale: u32,
    }

    impl Fixture {
        /// Base address of the table image.
        fn base(&self) -> u32 {
            addr(&self.table[0])
        }

        /// Base address of bank `b`'s record store.
        fn bank_base(&self, b: usize) -> u32 {
            addr(&self.stores[b][0])
        }
    }

    /// Plants a table of `nb` banks with `ns` records each at stride
    /// `scale`, all bytes random, and installs the globals.
    fn plant(rng: &mut Rng, nb: usize, ns: usize, scale: u32) -> Fixture {
        let store_len = ns * scale as usize + FLAG_OFF as usize + 1;
        let mut stores = Vec::with_capacity(nb);
        for _ in 0..nb {
            let mut s = vec![0u8; store_len];
            rng.bytes(&mut s);
            stores.push(s.into_boxed_slice());
        }
        let table_len = (nb - 1) * STRIDE as usize + TABLE_BIAS as usize + 4;
        let mut table = vec![0u8; table_len];
        rng.bytes(&mut table);
        let mut fx = Fixture {
            table: table.into_boxed_slice(),
            stores,
            scale,
        };
        let base = fx.base();
        for b in 0..nb {
            let at = base
                .wrapping_add(b as u32 * STRIDE)
                .wrapping_add(TABLE_BIAS);
            let bank = fx.bank_base(b);
            unsafe {
                std::ptr::write_unaligned(at as *mut u32, bank);
            }
        }
        rt::set_global(SCALE_VA, scale);
        rt::set_global(TABLE_VA, base);
        fx
    }

    /// Decodes the lifted table from a planted fixture.
    fn decode(fx: &Fixture, ns: usize) -> BankedVoices {
        let mut banks = Vec::with_capacity(fx.stores.len());
        for b in 0..fx.stores.len() {
            let base = fx.bank_base(b);
            let img = unsafe { image(base, fx.stores[b].len()) };
            let mut recs = Vec::with_capacity(ns);
            for s in 0..ns {
                let o = s * fx.scale as usize;
                let slot = u32::from_le_bytes(
                    img[o + SLOT_OFF as usize..o + SLOT_OFF as usize + 4]
                        .try_into()
                        .unwrap(),
                );
                recs.push(BankRecord {
                    slot,
                    flags: img[o + FLAG_OFF as usize],
                });
            }
            banks.push(recs);
        }
        BankedVoices {
            scale: fx.scale,
            banks,
        }
    }

    /// The record address the rewrite must compute for a selector.
    fn target_of(fx: &Fixture, at: VoiceSel) -> u32 {
        fx.bank_base(at.sub as usize)
            .wrapping_add(fx.scale.wrapping_mul(at.sel as u32))
    }

    /// Plants the `this` object carrying a selector.
    fn plant_sel(rng: &mut Rng, at: VoiceSel) -> (Box<[u8]>, u32) {
        let mut v = vec![0u8; 0x41];
        rng.bytes(&mut v);
        v[4] = at.sel;
        v[0x40] = at.sub;
        let obj: Box<[u8]> = v.into_boxed_slice();
        let this = addr(&obj[0]);
        (obj, this)
    }

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_audio::audio_voice::banked::{BankedVoices, VoiceSel};

        /// Sets bit 2 instead of bit 3.
        pub fn set_flag4(t: &mut BankedVoices, at: VoiceSel) {
            let r = &mut t.banks[at.sub as usize][at.sel as usize];
            r.flags |= 4;
        }

        /// Stores the complemented value.
        pub fn store_not(t: &mut BankedVoices, at: VoiceSel, value: u32) {
            t.banks[at.sub as usize][at.sel as usize].slot = !value;
        }

        /// Takes bit 1 of the argument instead of bit 0.
        pub fn set_bit1_from_bit1(t: &mut BankedVoices, at: VoiceSel, arg: u32) -> bool {
            let slot = &mut t.banks[at.sub as usize][at.sel as usize].flags;
            let doubled = ((arg as u8) >> 1).wrapping_mul(2);
            let adjust = (doubled ^ *slot) & 2;
            *slot ^= adjust;
            adjust != 0
        }
    }

    /// Scales the trials rotate through (all keep records disjoint).
    const SCALES: [u32; 4] = [0xEC, 0x200, 0x1000, 0xEC];

    #[test]
    fn set_flag8_matches() {
        let _guard = lock();
        let mut rng = Rng(0x9EB0);
        let mut caught = 0;
        for trial in 0..48u32 {
            let scale = SCALES[trial as usize % SCALES.len()];
            let nb = 1 + rng.below(3) as usize;
            let ns = 1 + rng.below(6) as usize;
            let fx = plant(&mut rng, nb, ns, scale);
            let at = VoiceSel {
                sel: rng.below(ns as u32) as u8,
                sub: rng.below(nb as u32) as u8,
            };
            let (_sel_obj, this) = plant_sel(&mut rng, at);
            // The rewrite must resolve exactly the planted record address.
            let target = target_of(&fx, at);
            debug_assert!(target >= fx.bank_base(at.sub as usize));
            let mut lift = decode(&fx, ns);
            let pre = lift.clone();
            let ret = unsafe { fn_008A9EB0::rw_008a9eb0(this as *const u8) };
            assert_eq!(
                ret,
                fx.base(),
                "trial {trial}: the rewrite answers the table base"
            );
            lift.set_flag8(at);
            // Every store byte must match the lift's record.
            for b in 0..nb {
                let base = fx.bank_base(b);
                let img = unsafe { image(base, fx.stores[b].len()) };
                for s in 0..ns {
                    let o = s * scale as usize;
                    let want = lift.banks[b][s];
                    let got_slot = u32::from_le_bytes(
                        img[o + SLOT_OFF as usize..o + SLOT_OFF as usize + 4]
                            .try_into()
                            .unwrap(),
                    );
                    assert_eq!(
                        got_slot, want.slot,
                        "trial {trial}: bank {b} record {s} slot"
                    );
                    assert_eq!(
                        img[o + FLAG_OFF as usize],
                        want.flags,
                        "trial {trial}: bank {b} record {s} flags"
                    );
                }
            }
            // The flag byte the rewrite touched is the lifted record's.
            let touched = unsafe { image(target.wrapping_add(FLAG_OFF), 1)[0] };
            assert_eq!(touched, lift.banks[at.sub as usize][at.sel as usize].flags);
            assert_eq!(touched & 8, 8, "trial {trial}: bit 3 is set");
            let mut w = pre.clone();
            wrong::set_flag4(&mut w, at);
            let wflag = w.banks[at.sub as usize][at.sel as usize].flags;
            if wflag != touched {
                caught += 1;
            }
        }
        assert!(caught > 0, "flag4 mutant was never caught");
    }

    #[test]
    fn store_slot_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA3A0);
        let mut caught = 0;
        for trial in 0..48u32 {
            let scale = SCALES[(trial as usize + 1) % SCALES.len()];
            let nb = 1 + rng.below(3) as usize;
            let ns = 1 + rng.below(6) as usize;
            let fx = plant(&mut rng, nb, ns, scale);
            let at = VoiceSel {
                sel: rng.below(ns as u32) as u8,
                sub: rng.below(nb as u32) as u8,
            };
            let value = if trial == 0 { 0xA5A5_A5A5 } else { rng.u32() };
            let (_sel_obj, this) = plant_sel(&mut rng, at);
            let target = target_of(&fx, at);
            let mut lift = decode(&fx, ns);
            let pre = lift.clone();
            let ret = unsafe { fn_008AA3A0::rw_008aa3a0(this as *const u8, value) };
            assert_eq!(ret, value, "trial {trial}: the rewrite echoes the value");
            lift.store_slot(at, value);
            for b in 0..nb {
                let base = fx.bank_base(b);
                let img = unsafe { image(base, fx.stores[b].len()) };
                for s in 0..ns {
                    let o = s * scale as usize;
                    let want = lift.banks[b][s];
                    let got_slot = u32::from_le_bytes(
                        img[o + SLOT_OFF as usize..o + SLOT_OFF as usize + 4]
                            .try_into()
                            .unwrap(),
                    );
                    assert_eq!(
                        got_slot, want.slot,
                        "trial {trial}: bank {b} record {s} slot"
                    );
                    assert_eq!(
                        img[o + FLAG_OFF as usize],
                        want.flags,
                        "trial {trial}: bank {b} record {s} flags"
                    );
                }
            }
            let touched = u32::from_le_bytes(
                unsafe { image(target.wrapping_add(SLOT_OFF), 4) }
                    .try_into()
                    .unwrap(),
            );
            assert_eq!(touched, value, "trial {trial}: the stored word");
            let mut w = pre.clone();
            wrong::store_not(&mut w, at, value);
            if w.banks[at.sub as usize][at.sel as usize].slot != touched {
                caught += 1;
            }
        }
        assert!(caught > 0, "store-not mutant was never caught");
    }

    #[test]
    fn set_bit1_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA3F0);
        let mut caught = 0;
        for trial in 0..64u32 {
            let scale = SCALES[(trial as usize + 2) % SCALES.len()];
            let nb = 1 + rng.below(3) as usize;
            let ns = 1 + rng.below(6) as usize;
            let fx = plant(&mut rng, nb, ns, scale);
            let at = VoiceSel {
                sel: rng.below(ns as u32) as u8,
                sub: rng.below(nb as u32) as u8,
            };
            // Trial 0 pins low-bit 1 against flag-bit 0.
            let arg = if trial == 0 { 0x01 } else { rng.u32() };
            let (_sel_obj, this) = plant_sel(&mut rng, at);
            let target = target_of(&fx, at);
            let mut lift = decode(&fx, ns);
            if trial == 0 {
                lift.banks[at.sub as usize][at.sel as usize].flags &= !2;
                unsafe {
                    image(target.wrapping_add(FLAG_OFF), 1)[0] &= !2;
                }
            }
            let before = unsafe { image(target.wrapping_add(FLAG_OFF), 1)[0] };
            let pre = lift.clone();
            let ret = unsafe { fn_008AA3F0::rw_008aa3f0(this as *const u8, arg) };
            let changed = lift.set_bit1(at, arg);
            // The table_hi|adjust answer is rebuilt from the lifted bit.
            let expect = (fx.base() & 0xFFFF_FF00) | u32::from(changed) * 2;
            assert_eq!(
                ret, expect,
                "trial {trial}: answer rebuilds from the lifted change"
            );
            // The adjust mask itself is pinned against the pre-state.
            let doubled = (arg as u8).wrapping_mul(2);
            let adjust = (doubled ^ before) & 2;
            assert_eq!(
                u32::from(changed) * 2,
                adjust as u32,
                "trial {trial}: mask matches the sequence"
            );
            let touched = unsafe { image(target.wrapping_add(FLAG_OFF), 1)[0] };
            assert_eq!(touched, lift.banks[at.sub as usize][at.sel as usize].flags);
            assert_eq!(
                touched & 2,
                ((arg as u8) & 1) << 1,
                "trial {trial}: bit 1 follows the argument's low bit"
            );
            let mut w = pre.clone();
            let wchanged = wrong::set_bit1_from_bit1(&mut w, at, arg);
            let wflag = w.banks[at.sub as usize][at.sel as usize].flags;
            if wflag != touched || wchanged != changed {
                caught += 1;
            }
        }
        assert!(caught > 0, "bit1-from-bit1 mutant was never caught");
    }
}
