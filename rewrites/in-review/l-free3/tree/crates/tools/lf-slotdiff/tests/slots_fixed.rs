//! Differential cases, part 2: flag word, fixed tables and adoption.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns (rebuilt addresses
//! where the lift answers indexes) and every effect. Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_slotdiff::rewrites::*;
    use lf_slotdiff::rt;
    use lf_streaming::slots::AdoptOutcome;
    use lf_streaming::slots::AdoptSlot;
    use lf_streaming::slots::AllocEntry;
    use lf_streaming::slots::AllocTable;
    use lf_streaming::slots::GeoSlot;
    use lf_streaming::slots::GeoSlots;
    use lf_streaming::slots::KeyEntries;
    use lf_streaming::slots::KeyEntry;
    use lf_streaming::slots::SlotFlags;
    use lf_streaming::slots::StateSlots;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{addr, get_u32, lock, put_u32, Rng};

    /// Flag word offset inside the slot record.
    const FLAGS_OFF: usize = 0x254;
    /// Keyed entry length and count.
    const KEY_LEN: usize = 28;
    const KEY_COUNT: usize = 256;
    /// Key word offsets inside a keyed entry.
    const KEY_A_OFF: usize = 8;
    const KEY_B_OFF: usize = 12;
    /// Link target state offset past the link.
    const TARGET_OFF: usize = 0x70;
    /// State slot base, stride and count.
    const STATE_BASE: usize = 0x1cb0;
    const STATE_LEN: usize = 0x70;
    const STATE_COUNT: usize = 64;
    /// The state the scan wants.
    const WANTED: u32 = 3;
    /// Alloc counter offset and entry base.
    const ALLOC_COUNT_OFF: usize = 0x38b0;
    const ALLOC_BASE: usize = 0x38b4;
    const ALLOC_LEN: usize = 16;
    const ALLOC_COUNT: usize = 128;
    /// Geometric slot base and stride.
    const GEO_BASE: usize = 0x40d8;
    const GEO_LEN: usize = 0x30;
    const GEO_COUNT: usize = 256;
    /// Owner word offset inside the adopt record.
    const OWNER_OFF: usize = 0x70;

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_streaming::slots::AdoptOutcome;
        use lf_streaming::slots::AdoptSlot;
        use lf_streaming::slots::AllocTable;
        use lf_streaming::slots::GeoSlots;
        use lf_streaming::slots::KeyEntries;
        use lf_streaming::slots::SlotFlags;
        use lf_streaming::slots::StateSlots;

        /// Clears bit 4 with the bit-3 review (the sibling instance).
        pub fn clear34(flags: &mut SlotFlags) -> u32 {
            flags.clear_and_review(4, 3)
        }

        /// Clears bit 3 with the bit-4 review (the sibling instance).
        pub fn clear43(flags: &mut SlotFlags) -> u32 {
            flags.clear_and_review(3, 4)
        }

        /// Sets only the live bit, forgetting bit 3.
        pub fn mark_live_only(flags: &mut SlotFlags) {
            flags.0 |= 1;
        }

        /// Searches the liveness key instead of the key word.
        pub fn find_key_a(table: &KeyEntries, key: u32) -> Option<usize> {
            table.entries.iter().position(|entry| entry.key_a == key)
        }

        /// Counts any non-null link as live, ignoring the target state.
        pub fn find_link_only(table: &KeyEntries, key: u32) -> Option<usize> {
            table
                .entries
                .iter()
                .position(|entry| entry.key_a == key && entry.target.is_some())
        }

        /// Reports the last holding slot instead of the first.
        pub fn find_last(slots: &StateSlots, wanted: u32) -> Option<usize> {
            slots.states.iter().rposition(|state| *state == wanted)
        }

        /// Treats any non-taken flag as free.
        pub fn alloc_not_taken(table: &mut AllocTable, idx: u32) -> Option<usize> {
            if idx <= 0x7f {
                let at = idx as usize;
                if at < table.entries.len() && table.entries[at].flag == 1 {
                    return Some(at);
                }
                return None;
            }
            let at = table.entries.iter().position(|entry| entry.flag != 1)?;
            let id = table.counter.wrapping_add(0x1000_0000);
            table.counter = table.counter.wrapping_add(1);
            let entry = &mut table.entries[at];
            entry.flag = 1;
            entry.id = id;
            entry.zero = 0;
            entry.link = 0xffff_ffff;
            Some(at)
        }

        /// Tests the flag for exactly 1 instead of nonzero.
        pub fn radius_flag1(slots: &GeoSlots, id: u32, point: [f32; 3], radius: f32) -> bool {
            for slot in &slots.slots {
                if slot.flag == 1 && slot.id == id {
                    let dx = point[0] - slot.center[0];
                    let dy = point[1] - slot.center[1];
                    let dz = point[2] - slot.center[2];
                    let d2 = (dy * dy + dx * dx) + dz * dz;
                    if radius * radius >= d2 {
                        return true;
                    }
                }
            }
            false
        }

        /// Treats owner 0 as the orphan value instead of all-ones.
        pub fn adopt_zero_orphan(slot: &mut AdoptSlot, generation: u32) -> AdoptOutcome {
            if slot.owner != 0 {
                return AdoptOutcome::AlreadyAdopted;
            }
            slot.owner = generation;
            AdoptOutcome::Marked(generation)
        }
    }

    /// Reads back planted bytes through a raw pointer, so the compiler
    /// cannot forward the pre-call value.
    fn read_back(base: u32, len: usize) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(base as *const u8, len) }.to_vec()
    }

    /// Flag words sweeping the low six bits plus random upper bits.
    fn flag_shapes(rng: &mut Rng) -> Vec<u32> {
        let mut out = vec![];
        for low in 0..64u32 {
            out.push(low | (rng.u32() & 0xffff_ffc0));
        }
        out.push(0);
        out.push(0xffff_ffff);
        for _ in 0..16 {
            out.push(rng.u32());
        }
        out
    }

    /// Plants a slot record with the flag word at +0x254.
    fn plant_record(rng: &mut Rng, flags: u32) -> (Box<[u8]>, u32) {
        let mut record = vec![0u8; FLAGS_OFF + 4].into_boxed_slice();
        rng.bytes(&mut record[..FLAGS_OFF]);
        put_u32(&mut record, FLAGS_OFF, flags);
        let obj = addr(&record[0]);
        (record, obj)
    }

    #[test]
    fn clear_bit3_matches() {
        let _guard = lock();
        let mut rng = Rng(0xC103);
        let mut cases = 0;
        let mut caught = 0;
        for flags in flag_shapes(&mut rng) {
            let (_record, obj) = plant_record(&mut rng, flags);
            let got = unsafe { fn_00A9F010::rw_00a9f010(obj) };
            let mut lift = SlotFlags::new(flags);
            let want = lift.clear_and_review(3, 4);
            assert_eq!(got, want, "flags={flags:#x}");
            let back = read_back(obj, FLAGS_OFF + 4);
            assert_eq!(get_u32(&back, FLAGS_OFF), lift.word(), "flags={flags:#x}");
            let mut bad = SlotFlags::new(flags);
            if wrong::clear34(&mut bad) != got || bad.word() != lift.word() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 64 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn clear_bit4_matches() {
        let _guard = lock();
        let mut rng = Rng(0xC104);
        let mut cases = 0;
        let mut caught = 0;
        for flags in flag_shapes(&mut rng) {
            let (_record, obj) = plant_record(&mut rng, flags);
            let got = unsafe { fn_00A9F040::rw_00a9f040(obj) };
            let mut lift = SlotFlags::new(flags);
            let want = lift.clear_and_review(4, 3);
            assert_eq!(got, want, "flags={flags:#x}");
            let back = read_back(obj, FLAGS_OFF + 4);
            assert_eq!(get_u32(&back, FLAGS_OFF), lift.word(), "flags={flags:#x}");
            let mut bad = SlotFlags::new(flags);
            if wrong::clear43(&mut bad) != got || bad.word() != lift.word() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 64 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn mark_live_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA911);
        let mut cases = 0;
        let mut caught = 0;
        for flags in flag_shapes(&mut rng) {
            let (_record, obj) = plant_record(&mut rng, flags);
            let got = unsafe { fn_00AA0880::rw_00aa0880(obj) };
            assert_eq!(got, obj, "flags={flags:#x}");
            let mut lift = SlotFlags::new(flags);
            lift.mark_live();
            let back = read_back(obj, FLAGS_OFF + 4);
            assert_eq!(get_u32(&back, FLAGS_OFF), lift.word(), "flags={flags:#x}");
            let mut bad = SlotFlags::new(flags);
            wrong::mark_live_only(&mut bad);
            if bad.word() != lift.word() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 64 && caught >= 1, "cases={cases} caught={caught}");
    }
    /// Plants keyed entries with their link targets, returning the
    /// entry buffer, its base, and the target buffer (kept alive by the
    /// caller). `links[i]` is `None` for a null link.
    fn plant_keyed(
        rng: &mut Rng,
        entries: &[KeyEntry],
        links: &[Option<u32>],
    ) -> (Box<[u8]>, u32, Box<[u8]>) {
        let mut buf = vec![0u8; entries.len() * KEY_LEN].into_boxed_slice();
        rng.bytes(&mut buf);
        let mut targets = vec![0u8; entries.len() * (TARGET_OFF + 4)].into_boxed_slice();
        rng.bytes(&mut targets);
        let base = addr(&buf[0]);
        let tbase = addr(&targets[0]);
        for (i, entry) in entries.iter().enumerate() {
            put_u32(&mut buf, i * KEY_LEN + KEY_A_OFF, entry.key_a);
            match links[i] {
                // A null link plants the key word itself (zero for the
                // live scan's null entries, the key for the key scan).
                None => put_u32(&mut buf, i * KEY_LEN + KEY_B_OFF, entry.key_b),
                Some(state) => {
                    let link = tbase.wrapping_add((i * (TARGET_OFF + 4)) as u32);
                    put_u32(&mut buf, i * KEY_LEN + KEY_B_OFF, link);
                    put_u32(&mut targets, i * (TARGET_OFF + 4) + TARGET_OFF, state);
                }
            }
        }
        (buf, base, targets)
    }

    /// Rebuilds an entry address from a found index, or null for none.
    fn rebuilt(base: u32, found: Option<usize>, stride: u32) -> u32 {
        found.map_or(0, |i| base.wrapping_add((i as u32).wrapping_mul(stride)))
    }

    #[test]
    fn find_by_key_matches() {
        let _guard = lock();
        let mut rng = Rng(0xBE1C);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..6 {
            // Keys from a small set so matches repeat; trial 0 pins
            // small key_b values against high key_a values.
            let mut entries = vec![];
            for i in 0..KEY_COUNT {
                let key_b = if trial == 0 {
                    (i % 9) as u32
                } else {
                    rng.below(12)
                };
                entries.push(KeyEntry {
                    key_a: if trial == 0 {
                        0x1000 + i as u32
                    } else {
                        rng.below(12)
                    },
                    key_b,
                    target: if key_b == 0 { None } else { Some(1) },
                });
            }
            let table = KeyEntries {
                entries: entries.clone(),
            };
            // Null links throughout: the key scan plants and reads the
            // key word, never a link.
            let links: Vec<Option<u32>> = vec![None; entries.len()];
            let (_buf, base, _targets) = plant_keyed(&mut rng, &entries, &links);
            for key in 0..14u32 {
                let got = unsafe { fn_00A9F1E0::rw_00a9f1e0(base, key) };
                assert_eq!(got, rebuilt(base, table.find_by_key(key), KEY_LEN as u32));
                if rebuilt(base, wrong::find_key_a(&table, key), KEY_LEN as u32) != got {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 60 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn find_live_matches() {
        let _guard = lock();
        let mut rng = Rng(0x11E);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..6 {
            let mut entries = vec![];
            let mut links = vec![];
            for i in 0..KEY_COUNT {
                let key_a = rng.below(10);
                // Link shapes: null, dead target, live target.
                let shape = rng.below(3);
                let (key_b, target, link) = match shape {
                    0 => (0, None, None),
                    1 => (0x7000 + i as u32, Some(0), Some(0)),
                    _ => (0x7000 + i as u32, Some(1 + rng.below(99)), None),
                };
                let link = link.or(if shape == 2 { target } else { None });
                entries.push(KeyEntry {
                    key_a,
                    key_b,
                    target,
                });
                links.push(link);
            }
            // Trial 0 pins the trap: the first key-7 entry is dead.
            if trial == 0 {
                entries[0] = KeyEntry {
                    key_a: 7,
                    key_b: 0x7000,
                    target: Some(0),
                };
                links[0] = Some(0);
                entries[1] = KeyEntry {
                    key_a: 7,
                    key_b: 0x7001,
                    target: Some(5),
                };
                links[1] = Some(5);
            }
            let table = KeyEntries {
                entries: entries.clone(),
            };
            let (_buf, base, _targets) = plant_keyed(&mut rng, &entries, &links);
            for key in 0..12u32 {
                let got = unsafe { fn_00A9F190::rw_00a9f190(base, key) };
                assert_eq!(got, rebuilt(base, table.find_live(key), KEY_LEN as u32));
                if rebuilt(base, wrong::find_link_only(&table, key), KEY_LEN as u32) != got {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 60 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn find_state_matches() {
        let _guard = lock();
        let mut rng = Rng(0x57A7);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..8 {
            let mut states = vec![];
            for i in 0..STATE_COUNT {
                states.push(match trial {
                    0 => 0,
                    1 => WANTED,
                    2 => u32::from(i == 0) * WANTED,
                    3 => u32::from(i == STATE_COUNT - 1) * WANTED,
                    _ => [0, 1, 2, WANTED, 4, 0xffff_ffff][rng.below(6) as usize],
                });
            }
            let slots = StateSlots {
                states: states.clone(),
            };
            let mut obj = vec![0u8; STATE_BASE + STATE_COUNT * STATE_LEN].into_boxed_slice();
            rng.bytes(&mut obj);
            for (i, state) in states.iter().enumerate() {
                put_u32(&mut obj, STATE_BASE + i * STATE_LEN, *state);
            }
            let this = addr(&obj[0]);
            let got = unsafe { fn_00A9F460::rw_00a9f460(this) };
            let want = rebuilt(
                this.wrapping_add(STATE_BASE as u32),
                slots.find_first_holding(WANTED),
                STATE_LEN as u32,
            );
            assert_eq!(got, want, "trial {trial}");
            let bad = rebuilt(
                this.wrapping_add(STATE_BASE as u32),
                wrong::find_last(&slots, WANTED),
                STATE_LEN as u32,
            );
            if bad != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 8 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn lookup_or_alloc_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA110);
        let mut cases = 0;
        let mut caught = 0;
        // Flag patterns: all free, all taken, sparse, odd values.
        let patterns: Vec<Vec<u8>> = vec![
            vec![0; ALLOC_COUNT],
            vec![1; ALLOC_COUNT],
            (0..ALLOC_COUNT).map(|i| u8::from(i % 3 == 0)).collect(),
            // Odd values first: the wrong lift frees them, the rewrite skips them.
            (0..ALLOC_COUNT).map(|i| [2, 0xff, 0, 1][i % 4]).collect(),
        ];
        for flags in patterns.iter() {
            for &counter in &[0u32, 1, 0x0fff_ffff, 0xffff_ffff] {
                let mut entries = vec![];
                for &flag in flags {
                    entries.push(AllocEntry {
                        id: rng.u32(),
                        flag,
                        pad: [rng.u32() as u8, rng.u32() as u8, rng.u32() as u8],
                        zero: rng.u32(),
                        link: rng.u32(),
                    });
                }
                // Indexes: direct ends, alloc spellings, edge 0x7f/0x80.
                let idxs = [0u32, 1, 0x7e, 0x7f, 0x80, 0x81, 0x100, 0xffff_ffff];
                for &idx in &idxs {
                    let mut table = AllocTable {
                        counter,
                        entries: entries.clone(),
                    };
                    let mut wrong_table = table.clone();
                    let mut obj =
                        vec![0u8; ALLOC_BASE + ALLOC_COUNT * ALLOC_LEN].into_boxed_slice();
                    rng.bytes(&mut obj[..ALLOC_COUNT_OFF]);
                    put_u32(&mut obj, ALLOC_COUNT_OFF, counter);
                    for (i, entry) in entries.iter().enumerate() {
                        obj[ALLOC_BASE + i * ALLOC_LEN..ALLOC_BASE + (i + 1) * ALLOC_LEN]
                            .copy_from_slice(&entry.to_bytes());
                    }
                    let this = addr(&obj[0]);
                    let entry_base = this.wrapping_add(ALLOC_BASE as u32);
                    let got = unsafe { fn_00A9F4D0::rw_00a9f4d0(this, idx) };
                    let want = table.lookup_or_alloc(idx);
                    assert_eq!(got, rebuilt(entry_base, want, ALLOC_LEN as u32));
                    // Counter and every entry byte match the lifted table.
                    let back = read_back(this, ALLOC_BASE + ALLOC_COUNT * ALLOC_LEN);
                    assert_eq!(get_u32(&back, ALLOC_COUNT_OFF), table.counter);
                    for (i, entry) in table.entries.iter().enumerate() {
                        let at = ALLOC_BASE + i * ALLOC_LEN;
                        assert_eq!(&back[at..at + ALLOC_LEN], entry.to_bytes());
                    }
                    // The wrong lift frees any non-taken flag.
                    let bad = wrong::alloc_not_taken(&mut wrong_table, idx);
                    if bad != want || wrong_table != table {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }
    /// Float bit patterns: zeros, ones, bounds, specials, random.
    fn float_shapes(rng: &mut Rng) -> Vec<u32> {
        let mut out = vec![
            0x0000_0000,
            0x8000_0000,
            0x3f80_0000,
            0xbf80_0000,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0000,
            0xffc0_0000,
            0x7f80_0001,
            0x0000_0001,
            0x8000_0001,
            0x7f7f_ffff,
            0x4b00_0000,
        ];
        for _ in 0..12 {
            out.push(rng.u32());
        }
        out
    }

    #[test]
    fn in_radius_matches() {
        let _guard = lock();
        let mut rng = Rng(0x8AD1);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..10 {
            // Slots with flag/id/centre shapes; the flag byte sits
            // 0x18 before the slot base, the id 0x14 before, the
            // centre at -8/-4/+0.
            let mut slots = vec![];
            for i in 0..GEO_COUNT {
                let flag = [0u8, 1, 2, 0xff][rng.below(4) as usize];
                slots.push(GeoSlot {
                    flag,
                    id: if trial == 0 {
                        (i % 3) as u32
                    } else {
                        rng.below(4)
                    },
                    center: [
                        f32::from_bits(rng.u32()),
                        f32::from_bits(rng.u32()),
                        f32::from_bits(rng.u32()),
                    ],
                });
            }
            // Trial 0 pins one inside candidate with flag 2: the
            // flag-1 wrong lift must miss it.
            if trial == 0 {
                slots[0] = GeoSlot {
                    flag: 2,
                    id: 9,
                    center: [1.0, 0.0, 0.0],
                };
                for slot in slots.iter_mut().skip(1) {
                    slot.id = 8;
                }
            }
            let lift_slots = GeoSlots {
                slots: slots.clone(),
            };
            let mut obj = vec![0u8; 0x18 + GEO_BASE + GEO_COUNT * GEO_LEN + 4].into_boxed_slice();
            rng.bytes(&mut obj);
            let this = addr(&obj[0]).wrapping_add(0x18);
            for (i, slot) in slots.iter().enumerate() {
                let base = 0x18 + GEO_BASE + i * GEO_LEN;
                obj[base - 0x18] = slot.flag;
                put_u32(&mut obj, base - 0x14, slot.id);
                put_u32(&mut obj, base - 8, slot.center[0].to_bits());
                put_u32(&mut obj, base - 4, slot.center[1].to_bits());
                put_u32(&mut obj, base, slot.center[2].to_bits());
            }
            // Queries: matching and missing ids, shaped points/radii.
            let floats = float_shapes(&mut rng);
            let mut queries = vec![];
            for &id in &[0u32, 1, 2, 3, 8, 9] {
                for &px in &floats[..6] {
                    queries.push((id, [px, floats[2], floats[0]], floats[2]));
                }
            }
            queries.push((9, [0x3f80_0000, 0, 0], 0x4000_0000));
            for (id, point_bits, radius_bits) in queries {
                let mut pos = vec![0u8; 12].into_boxed_slice();
                put_u32(&mut pos, 0, point_bits[0]);
                put_u32(&mut pos, 4, point_bits[1]);
                put_u32(&mut pos, 8, point_bits[2]);
                let pos_addr = addr(&pos[0]);
                let got = unsafe { fn_00A9EE60::rw_00a9ee60(this, id, pos_addr, radius_bits) };
                let point = [
                    f32::from_bits(point_bits[0]),
                    f32::from_bits(point_bits[1]),
                    f32::from_bits(point_bits[2]),
                ];
                let radius = f32::from_bits(radius_bits);
                let want = lift_slots.any_in_radius(id, point, radius);
                assert_eq!(got, if want { 1 } else { 0x100 }, "id={id} trial={trial}");
                if wrong::radius_flag1(&lift_slots, id, point, radius) != want {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 200 && caught >= 1, "cases={cases} caught={caught}");
    }

    // Recording stubs for the adoption hook and marker.
    static HOOK_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    extern "thiscall" fn hook_stub(generation: u32, owner_addr: u32) -> u32 {
        HOOK_LOG.lock().unwrap().push((generation, owner_addr));
        0
    }
    static MARK_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static MARK_SCRIPT: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "thiscall" fn mark_stub(this: u32, slot: u32) -> u32 {
        MARK_LOG.lock().unwrap().push((this, slot));
        MARK_SCRIPT.lock().unwrap().pop().unwrap_or(0)
    }

    #[test]
    fn adopt_matches() {
        let _guard = lock();
        rt::set_callee(1, hook_stub as *const () as usize as u32);
        rt::set_callee(2, mark_stub as *const () as usize as u32);
        let mut rng = Rng(0xAD07);
        let mut cases = 0;
        let mut caught = 0;
        // Owner words: orphan, zero, neighbours of orphan, random.
        let mut owners = vec![0xffff_ffffu32, 0, 1, 0xffff_fffe, 0x8000_0000];
        for _ in 0..4 {
            owners.push(rng.u32());
        }
        for &owner in &owners {
            for _trial in 0..4 {
                let generation = rng.u32();
                let mut record = vec![0u8; OWNER_OFF + 4].into_boxed_slice();
                rng.bytes(&mut record[..OWNER_OFF]);
                put_u32(&mut record, OWNER_OFF, owner);
                let slot = addr(&record[0]);
                let manager = vec![0x11u8; 8].into_boxed_slice();
                let this = addr(&manager[0]);
                let scripted = rng.u32();
                MARK_SCRIPT.lock().unwrap().push(scripted);
                HOOK_LOG.lock().unwrap().clear();
                MARK_LOG.lock().unwrap().clear();
                let unused = rng.u32();
                let got = unsafe { fn_00AA0850::rw_00aa0850(this, slot, generation, unused) };
                let back = read_back(slot, OWNER_OFF + 4);
                let hook_log = HOOK_LOG.lock().unwrap().clone();
                let mark_log = MARK_LOG.lock().unwrap().clone();
                let mut lift = AdoptSlot::new(owner);
                let mut lift_hook = vec![];
                let mut lift_mark = vec![];
                let lift_out = lift.adopt(
                    generation,
                    &mut |seen: u32| lift_hook.push(seen),
                    &mut |seen: &AdoptSlot| {
                        lift_mark.push(*seen);
                        scripted
                    },
                );
                match lift_out {
                    AdoptOutcome::AlreadyAdopted => {
                        assert_eq!(got, slot.wrapping_add(OWNER_OFF as u32));
                        assert!(hook_log.is_empty() && mark_log.is_empty());
                        assert!(lift_hook.is_empty() && lift_mark.is_empty());
                        assert_eq!(get_u32(&back, OWNER_OFF), owner);
                        assert_eq!(lift.owner, owner);
                    }
                    AdoptOutcome::Marked(answer) => {
                        assert_eq!(got, scripted);
                        assert_eq!(answer, scripted);
                        assert_eq!(
                            hook_log,
                            vec![(generation, slot.wrapping_add(OWNER_OFF as u32))]
                        );
                        assert_eq!(mark_log, vec![(this, slot)]);
                        assert_eq!(lift_hook, vec![generation]);
                        assert_eq!(lift_mark, vec![AdoptSlot::new(generation)]);
                        assert_eq!(get_u32(&back, OWNER_OFF), generation);
                        assert_eq!(lift.owner, generation);
                    }
                }
                // The wrong lift orphans on zero instead of all-ones.
                let mut bad = AdoptSlot::new(owner);
                let bad_out = wrong::adopt_zero_orphan(&mut bad, generation);
                let rewrite_outcome = if hook_log.is_empty() {
                    AdoptOutcome::AlreadyAdopted
                } else {
                    AdoptOutcome::Marked(got)
                };
                if bad_out != rewrite_outcome {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 20 && caught >= 1, "cases={cases} caught={caught}");
    }
}
