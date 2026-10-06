//! Differential cases, part 5 (second lane): the fixed tables.
//!
//! Each case plants real 32-bit images, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (full images, header globals, callee logs). Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_pooldiff::rewrites::*;
    use lf_pooldiff::rt;
    use lf_world::pools::{
        ENTRY_COUNT, ENTRY_LEN, FIND_MISS, FIND_SCALE, REVOC_BIT, REVOC_COUNT, REVOC_LEN,
        EntryTable, RevocTable,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_END_VA, E_TAB_VA, R_A_VA, R_B_VA, R_C_VA, R_D_VA, R_END_VA, R_FIRST_VA, R_SLOTS_VA, Rng,
        addr, lock, put_u32,
    };

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_world::pools::{EntryTable, RevocTable};

        /// Zeroes the untouched gap halves too.
        pub fn init_zero_gap(t: &mut EntryTable) {
            for e in &mut t.entries {
                e.ptr = 0;
                e.tag = 0xffff;
                e.gap = 0;
                e.key = 0;
                e.sum = 0;
                e.flag = 0;
                e.flag_hi = 0;
            }
        }

        /// Takes the last match instead of the first.
        pub fn find_last(t: &mut EntryTable, key: u32, base: u32, add: u32) -> Option<usize> {
            let target = base.wrapping_add(add);
            let mut hit = None;
            for (i, e) in t.entries.iter_mut().enumerate() {
                if e.ptr != 0 && e.key == key && e.sum == target {
                    e.flag = 1;
                    hit = Some(i);
                }
            }
            hit
        }

        /// Forgets the second header word.
        pub fn reset_skip_b(t: &mut RevocTable) {
            t.head_a = 0;
            for e in &mut t.entries {
                e.body[7] &= !0x10;
            }
        }

        /// Leaves the four header bytes alone.
        pub fn clear_skip_c(t: &mut RevocTable) {
            t.head_a = 0;
            t.head_b = 0;
            for e in &mut t.entries {
                e.body[7] &= !0x10;
            }
            t.head_d = 0;
        }

        /// Ignores the next-byte condition.
        pub fn revoke_no_next(t: &mut RevocTable, key: u32) {
            let tag = key as u16;
            let mut live = t.head_a;
            for e in &mut t.entries {
                if e.tag == tag && e.body[7] & 0x10 != 0 {
                    e.body[7] &= !0x10;
                    e.tag = 0;
                    live = live.wrapping_sub(1);
                }
            }
            t.head_a = live;
        }
    }

    static SLOTS_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "thiscall" fn slots_stub(slots: u32) -> u32 {
        SLOTS_LOG.lock().unwrap().push(slots);
        0xDEAD
    }

    /// Copies test memory the rewrite wrote back out (raw reads: the
    /// borrow checker cannot see writes through the planted address).
    unsafe fn snapshot(base: u32, len: usize) -> Vec<u8> {
        unsafe {
            let mut out = vec![0u8; len];
            std::ptr::copy_nonoverlapping(base as *const u8, out.as_mut_ptr(), len);
            out
        }
    }

    unsafe fn read_global(va: u32) -> u32 {
        unsafe { rt::global::<u32>(va).read() }
    }

    #[test]
    fn entry_init_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1A0);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..8 {
            let len = ENTRY_COUNT * ENTRY_LEN;
            let mut img = vec![0u8; len];
            rng.bytes(&mut img);
            // Targeted gap/tail shapes: sometimes all zero (mutant hides),
            // usually garbage (mutant caught).
            let boxed = img.into_boxed_slice();
            let base = addr(&boxed[0]);
            assert!(base.checked_add(len as u32).is_some(), "image wraps");
            rt::set_relocated(E_TAB_VA, base);
            rt::set_relocated(E_END_VA, base.wrapping_add(8));
            let before = unsafe { snapshot(base, len) };
            let got = unsafe { fn_00962650::rw_00962650() };
            let after = unsafe { snapshot(base, len) };
            let mut lift = EntryTable::from_bytes(&before);
            let end = lift.init();
            assert_eq!(got, base.wrapping_add(end as u32), "end pointer");
            assert_eq!(end, len + 8, "biased end offset");
            let mut expect = vec![0u8; len];
            lift.to_bytes(&mut expect);
            assert_eq!(after, expect, "full image");
            // Wrong lift: zeroes the gaps.
            let mut w = EntryTable::from_bytes(&before);
            wrong::init_zero_gap(&mut w);
            let mut wimg = vec![0u8; len];
            w.to_bytes(&mut wimg);
            if wimg != after {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&boxed);
            std::mem::forget(boxed);
        }
        assert!(cases == 8, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong init never caught ({cases} cases)");
    }

    /// Plants one entry's modelled words into an image.
    fn plant_entry(img: &mut [u8], i: usize, ptr: u32, key: u32, sum: u32) {
        let off = i * ENTRY_LEN;
        put_u32(img, off, ptr);
        put_u32(img, off + 8, key);
        put_u32(img, off + 12, sum);
    }

    #[test]
    fn entry_find_mark_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1B0);
        let mut cases = 0;
        let mut caught = 0;
        for round in 0..10 {
            let len = ENTRY_COUNT * ENTRY_LEN;
            let mut img = vec![0u8; len];
            rng.bytes(&mut img);
            // Crafted layout: a null-pointer entry with matching words
            // (must be skipped), two matching live entries (first wins),
            // and decoys sharing one word each.
            let key = rng.u32();
            let base = rng.u32();
            let add = rng.u32();
            let target = base.wrapping_add(add);
            let skip = (rng.below(ENTRY_COUNT as u32 - 4) + 1) as usize;
            let first = skip + 1;
            let second = skip + 2;
            plant_entry(&mut img, skip, 0, key, target);
            plant_entry(&mut img, first, 0x1000 + round, key, target);
            plant_entry(&mut img, second, 0x2000 + round, key, target);
            plant_entry(&mut img, skip + 3, 0x3000 + round, key, target ^ 1);
            img[first * ENTRY_LEN + 16] = 0;
            img[second * ENTRY_LEN + 16] = 0;
            // Every other entry gets a null pointer so no stray match
            // hides: only the crafted rows can match.
            for i in 0..ENTRY_COUNT {
                if i != skip && i != first && i != second && i != skip + 3 {
                    put_u32(&mut img, i * ENTRY_LEN, 0);
                }
            }
            let boxed = img.into_boxed_slice();
            let base_addr = addr(&boxed[0]);
            assert!(base_addr.checked_add(len as u32).is_some(), "image wraps");
            rt::set_relocated(E_TAB_VA, base_addr);
            rt::set_relocated(E_END_VA, base_addr.wrapping_add(8));
            let before = unsafe { snapshot(base_addr, len) };
            let got = unsafe { fn_00963120::rw_00963120(key, base, add) };
            let after = unsafe { snapshot(base_addr, len) };
            let mut lift = EntryTable::from_bytes(&before);
            let hit = lift.find_mark(key, base, add);
            let expect_ret = hit.map_or(FIND_MISS, |i| (i as u32) * FIND_SCALE);
            assert_eq!(got, expect_ret, "round {round} return");
            assert_eq!(hit, Some(first), "round {round} first match wins");
            let mut expect = vec![0u8; len];
            lift.to_bytes(&mut expect);
            assert_eq!(after, expect, "round {round} full image");
            assert_eq!(after[first * ENTRY_LEN + 16], 1, "hit flag set");
            // The wrong lift takes the last match; it must differ here.
            let mut w = EntryTable::from_bytes(&before);
            let whit = wrong::find_last(&mut w, key, base, add);
            assert_eq!(whit, Some(second));
            if whit != hit {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&boxed);
            std::mem::forget(boxed);
        }
        // A miss round: no live entry matches.
        {
            let len = ENTRY_COUNT * ENTRY_LEN;
            let img = vec![0xA5u8; len];
            let boxed = img.into_boxed_slice();
            let base_addr = addr(&boxed[0]);
            rt::set_relocated(E_TAB_VA, base_addr);
            rt::set_relocated(E_END_VA, base_addr.wrapping_add(8));
            let before = unsafe { snapshot(base_addr, len) };
            let got = unsafe { fn_00963120::rw_00963120(1, 2, 3) };
            let after = unsafe { snapshot(base_addr, len) };
            let mut lift = EntryTable::from_bytes(&before);
            assert_eq!(lift.find_mark(1, 2, 3), None);
            assert_eq!(got, FIND_MISS, "miss return");
            assert_eq!(after, before, "miss writes nothing");
            cases += 1;
            std::hint::black_box(&boxed);
            std::mem::forget(boxed);
        }
        assert!(cases == 11, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong find never caught ({cases} cases)");
    }

    /// Plants the revocation image and headers; returns (base, first, end).
    fn plant_revoc(img: Vec<u8>, head_a: u32, head_b: u32, head_c: [u8; 4], head_d: u32) -> (u32, u32, u32) {
        let len = REVOC_COUNT * REVOC_LEN;
        assert_eq!(img.len(), len);
        let boxed = img.into_boxed_slice();
        let base = addr(&boxed[0]);
        assert!(base.checked_add((len + 9) as u32).is_some(), "image wraps");
        let first = base.wrapping_add(9);
        let end = first.wrapping_add(len as u32);
        assert!(end < 0x8000_0000, "cursor range must stay below 2^31");
        rt::set_relocated(R_FIRST_VA, first);
        rt::set_relocated(R_END_VA, end);
        unsafe {
            rt::global::<u32>(R_A_VA).write(head_a);
            rt::global::<u32>(R_B_VA).write(head_b);
            rt::global::<u32>(R_C_VA).write(u32::from_le_bytes(head_c));
            rt::global::<u32>(R_D_VA).write(head_d);
        }
        std::mem::forget(boxed);
        (base, first, end)
    }

    #[test]
    fn revoc_reset_matches() {
        let _guard = lock();
        rt::set_callee(1, slots_stub as usize as u32);
        let mut rng = Rng(0xF1C0);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..6 {
            SLOTS_LOG.lock().unwrap().clear();
            let len = REVOC_COUNT * REVOC_LEN;
            let mut img = vec![0u8; len];
            rng.bytes(&mut img);
            let slots_box = Box::leak(Box::new([0u8; 64]));
            let slots_addr = addr(&slots_box[0]);
            rt::set_relocated(R_SLOTS_VA, slots_addr);
            let pre_a = rng.u32();
            let pre_b = rng.u32() | 1;
            let pre_c = rng.u32().to_le_bytes();
            let pre_d = rng.u32();
            let (base, _, end) = plant_revoc(img, pre_a, pre_b, pre_c, pre_d);
            let before = unsafe { snapshot(base, len) };
            let got = unsafe { fn_00B75820::rw_00b75820() };
            let after = unsafe { snapshot(base, len) };
            assert_eq!(got, end, "final cursor");
            assert_eq!(unsafe { read_global(R_A_VA) }, 0, "head_a zeroed");
            assert_eq!(unsafe { read_global(R_B_VA) }, 0, "head_b zeroed");
            assert_eq!(unsafe { read_global(R_D_VA) }, pre_d, "head_d untouched");
            assert_eq!(*SLOTS_LOG.lock().unwrap(), [slots_addr], "slot reset called once");
            let mut lift_calls = 0;
            let mut lift = RevocTable::from_parts(pre_a, pre_b, pre_c, pre_d, &before);
            let lend = lift.reset(&mut | | lift_calls += 1);
            assert_eq!(base.wrapping_add(9).wrapping_add(lend as u32), end, "lift end");
            assert_eq!(lift.head_a, 0);
            assert_eq!(lift.head_b, 0);
            assert_eq!(lift.head_c, pre_c, "head_c untouched");
            assert_eq!(lift.head_d, pre_d, "head_d untouched");
            assert_eq!(lift_calls, 1, "lift slot reset called once");
            let mut expect = vec![0u8; len];
            lift.to_bytes(&mut expect);
            assert_eq!(after, expect, "full image");
            let mut w = RevocTable::from_parts(pre_a, pre_b, pre_c, pre_d, &before);
            wrong::reset_skip_b(&mut w);
            if w.head_b != 0 {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases == 6, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong reset never caught ({cases} cases)");
    }

    #[test]
    fn revoc_clear_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1D0);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..6 {
            let len = REVOC_COUNT * REVOC_LEN;
            let mut img = vec![0u8; len];
            rng.bytes(&mut img);
            let pre_a = rng.u32();
            let pre_b = rng.u32();
            let mut pre_c = rng.u32().to_le_bytes();
            pre_c[0] |= 1;
            let pre_d = rng.u32();
            let (base, _, end) = plant_revoc(img, pre_a, pre_b, pre_c, pre_d);
            let before = unsafe { snapshot(base, len) };
            let got = unsafe { fn_00B75860::rw_00b75860() };
            let after = unsafe { snapshot(base, len) };
            assert_eq!(got, end, "final cursor");
            assert_eq!(unsafe { read_global(R_A_VA) }, 0, "head_a zeroed");
            assert_eq!(unsafe { read_global(R_B_VA) }, 0, "head_b zeroed");
            assert_eq!(unsafe { read_global(R_C_VA) }, 0, "head_c zeroed");
            assert_eq!(unsafe { read_global(R_D_VA) }, 0, "head_d zeroed");
            let mut lift = RevocTable::from_parts(pre_a, pre_b, pre_c, pre_d, &before);
            let lend = lift.clear();
            assert_eq!(base.wrapping_add(9).wrapping_add(lend as u32), end, "lift end");
            assert_eq!(lift.head_a, 0);
            assert_eq!(lift.head_b, 0);
            assert_eq!(lift.head_c, [0; 4]);
            assert_eq!(lift.head_d, 0);
            let mut expect = vec![0u8; len];
            lift.to_bytes(&mut expect);
            assert_eq!(after, expect, "full image");
            let mut w = RevocTable::from_parts(pre_a, pre_b, pre_c, pre_d, &before);
            wrong::clear_skip_c(&mut w);
            if w.head_c != [0; 4] {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases == 6, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong clear never caught ({cases} cases)");
    }

    /// Plants one revocation entry's deciding bytes into an image.
    fn plant_revoc_entry(img: &mut [u8], i: usize, tag: u16, flag: u8, next: u8) {
        let off = i * REVOC_LEN;
        img[off..off + 2].copy_from_slice(&tag.to_le_bytes());
        img[off + 9] = flag;
        img[off + 10] = next;
    }

    #[test]
    fn revoc_revoke_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1E0);
        let mut cases = 0;
        let mut caught = 0;
        for round in 0..6 {
            let len = REVOC_COUNT * REVOC_LEN;
            let mut img = vec![0u8; len];
            rng.bytes(&mut img);
            let key = rng.u32();
            let tag = key as u16;
            // Crafted rows: revoked, next-armed (kept), disarmed (kept),
            // wrong tag (kept). Live count wraps on round 0 (starts 0).
            let live: u32 = if round == 0 { 0 } else { rng.u32() };
            let r0 = (rng.below(REVOC_COUNT as u32 - 8) + 2) as usize;
            plant_revoc_entry(&mut img, r0, tag, REVOC_BIT | 0x03, 0x00);
            plant_revoc_entry(&mut img, r0 + 1, tag, REVOC_BIT | 0x03, REVOC_BIT);
            plant_revoc_entry(&mut img, r0 + 2, tag, 0x03, 0x00);
            plant_revoc_entry(&mut img, r0 + 3, tag ^ 0x0ff0, REVOC_BIT, 0x00);
            plant_revoc_entry(&mut img, r0 + 4, tag, REVOC_BIT, 0x00);
            // Scrub every other row's flag bit so no random row can match.
            for i in 0..REVOC_COUNT {
                if i < r0 || i > r0 + 4 {
                    img[i * REVOC_LEN + 9] &= !REVOC_BIT;
                }
            }
            let pre_b = rng.u32();
            let pre_c = [0u8; 4];
            let pre_d = rng.u32();
            let (base, _, end) = plant_revoc(img, live, pre_b, pre_c, pre_d);
            let before = unsafe { snapshot(base, len) };
            let got = unsafe { fn_00B75E60::rw_00b75e60(key) };
            let after = unsafe { snapshot(base, len) };
            assert_eq!(got, end, "round {round} final cursor");
            let mut lift = RevocTable::from_parts(live, pre_b, pre_c, pre_d, &before);
            let lend = lift.revoke(key);
            assert_eq!(base.wrapping_add(9).wrapping_add(lend as u32), end, "round {round} lift end");
            // Two rows revoked (r0, r0+4): count wraps down by two.
            assert_eq!(lift.head_a, live.wrapping_sub(2), "round {round} live count");
            assert_eq!(unsafe { read_global(R_A_VA) }, live.wrapping_sub(2));
            let mut expect = vec![0u8; len];
            lift.to_bytes(&mut expect);
            assert_eq!(after, expect, "round {round} full image");
            // The revoked rows lost the bit and their tag; the kept rows
            // kept both.
            assert_eq!(after[r0 * REVOC_LEN + 9] & REVOC_BIT, 0);
            assert_eq!(&after[r0 * REVOC_LEN..r0 * REVOC_LEN + 2], &[0, 0]);
            assert_eq!(after[(r0 + 1) * REVOC_LEN + 9] & REVOC_BIT, REVOC_BIT);
            let mut w = RevocTable::from_parts(live, pre_b, pre_c, pre_d, &before);
            wrong::revoke_no_next(&mut w, key);
            if w.head_a != live.wrapping_sub(2) {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases == 6, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong revoke never caught ({cases} cases)");
    }
}
