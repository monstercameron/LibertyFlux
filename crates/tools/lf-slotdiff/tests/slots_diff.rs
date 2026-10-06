//! Differential cases: the streaming entry table against its rewrites.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (written bytes, out words, callee call logs). Each method has a
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
    use lf_streaming::slots::DataSlots;
    use lf_streaming::slots::KindBytes;
    use lf_streaming::slots::KindSlots;
    use lf_streaming::slots::SlotTable;
    use lf_streaming::slots::StreamEntry;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        addr, get_u32, lock, put_u32, Rng, DATA_VA, KIND_FLAG_VA, KIND_TABLE_VA, TABLE_BASE_VA,
    };

    /// Byte length of one 32-bit entry.
    const ENTRY_LEN: usize = 24;
    /// Data-slot stride of the 32-bit slot region.
    const SLOT_STRIDE: usize = 160;
    /// Kind-flag base offset inside the shared slot region.
    const KIND_FLAG_OFF: usize = 11;
    /// Kind-table stride of the 32-bit slot-index table.
    const KIND_SLOT_STRIDE: usize = 100;

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_streaming::slots::DataSlots;
        use lf_streaming::slots::KindBytes;
        use lf_streaming::slots::SlotTable;
        use lf_streaming::slots::StreamEntry;

        /// Empty except all-ones links: zeroes the link words too.
        pub fn init_zero_links() -> StreamEntry {
            StreamEntry::from_words([0, 0, 0, 0, 0, 0])
        }

        /// Tests flag bit 10 instead of bit 11.
        pub fn active_bit10(entry: StreamEntry) -> bool {
            entry.words()[2] & 0xffff_fffc != 0 || (entry.flags() as u32 >> 10) & 1 != 0
        }

        /// Forgets the round-up: floor instead of ceiling.
        pub fn blocks_floor(entry: StreamEntry) -> u32 {
            (entry.words()[2] >> 2) >> 11
        }

        /// Data address with the bit-10 active test.
        pub fn data_bit10(entry: StreamEntry, slots: &DataSlots) -> u32 {
            if !active_bit10(entry) {
                return 0;
            }
            let value = entry.value();
            (value >> 8).wrapping_add(slots.slots[(value & 0xff) as usize])
        }

        /// Location with the bit-10 active test.
        pub fn locate_bit10(entry: &StreamEntry, data: u32) -> Option<(u32, u32)> {
            if !active_bit10(*entry) {
                return None;
            }
            Some((data, entry.block_count()))
        }

        /// Slot index through the value kind instead of the kind byte.
        pub fn slot_value_kind(table: &SlotTable, idx: u32, kinds: &[u32; 256]) -> u32 {
            let entries = table.entries();
            let at = idx as usize;
            let offset = idx.wrapping_mul(24);
            let high = ((offset as i32 as i64 * 0x2aaa_aaabi64) >> 32) as i32;
            let shifted = high >> 2;
            let index = shifted.wrapping_add(((shifted as u32) >> 31) as i32) as u16 as u32;
            let kind = entries[at].value_kind();
            index.wrapping_sub(kinds[usize::from(kind)])
        }

        /// Range check without the sentinel exclusion.
        pub fn usable_no_sentinel(table: &SlotTable, idx: u32) -> bool {
            if table.cap() < 0 || table.entries().is_empty() {
                return false;
            }
            let signed = idx as i32;
            if signed < 0 {
                return false;
            }
            signed < table.cap().wrapping_add(6)
        }

        /// Mask test without the always-tested bits.
        pub fn mask_plain(table: &SlotTable, idx: u32, mask: u32) -> bool {
            u32::from(table.entries()[idx as usize].flags()) & mask == 0
        }

        /// Active test that answers true for an empty table.
        pub fn entry_active_no_null(table: &SlotTable, idx: u32) -> bool {
            if table.entries().is_empty() {
                return true;
            }
            table.entries()[idx as usize].is_active()
        }

        /// Kind flag scaled by 96 instead of 160.
        pub fn kind_flag_96(table: &SlotTable, idx: u32, kinds: &KindBytes) -> u32 {
            let scaled = u32::from(table.entries()[idx as usize].value_kind()).wrapping_mul(96);
            (scaled & 0xffff_ff00) | u32::from(kinds.bytes[scaled as usize])
        }

        /// Sets bit 14 instead of bit 15.
        pub fn set_bit14(entry: &mut StreamEntry) {
            let mut words = entry.words();
            words[3] |= 0x4000_0000;
            *entry = StreamEntry::from_words(words);
        }
    }

    /// Plants entries as 32-bit bytes, returning the buffer and its address.
    fn plant_entries(entries: &[StreamEntry]) -> (Box<[u8]>, u32) {
        let mut buf = vec![0u8; entries.len() * ENTRY_LEN].into_boxed_slice();
        for (i, entry) in entries.iter().enumerate() {
            buf[i * ENTRY_LEN..(i + 1) * ENTRY_LEN].copy_from_slice(&entry.to_bytes());
        }
        let base = addr(&buf[0]);
        (buf, base)
    }

    /// Reads back planted bytes through a raw pointer, so the compiler
    /// cannot forward the pre-call value.
    fn read_back(base: u32, len: usize) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(base as *const u8, len) }.to_vec()
    }

    /// Plants a table object (base at +0, capacity at +4).
    fn plant_table(base: u32, cap: i32) -> (Box<[u8]>, u32) {
        let mut obj = vec![0u8; 8].into_boxed_slice();
        put_u32(&mut obj, 0, base);
        put_u32(&mut obj, 4, cap as u32);
        let this = addr(&obj[0]);
        (obj, this)
    }

    /// Random entry plus targeted size/flag shapes.
    fn entry_shapes(rng: &mut Rng) -> Vec<StreamEntry> {
        let mut out = vec![
            StreamEntry::empty(),
            StreamEntry::from_words([0, 0, 3, 0, 0, 0]),
            StreamEntry::from_words([0, 0, 4, 0, 0, 0]),
            StreamEntry::from_words([0, 0, 0, 0x0400_0000, 0, 0]),
            StreamEntry::from_words([0, 0, 0, 0x0800_0000, 0, 0]),
            StreamEntry::from_words([0, 0, 0, 0x0c00_0000, 0, 0]),
            StreamEntry::from_words([0, 0, 0xffff_ffff, 0xffff_0000, 0, 0]),
            StreamEntry::from_words([0, 0xff, 0, 0x0800_0000, 0, 0xff00_0000]),
        ];
        for _ in 0..24 {
            out.push(StreamEntry::from_words([
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
            ]));
        }
        out
    }

    #[test]
    fn init_matches() {
        let _guard = lock();
        let mut rng = Rng(0x1117);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..40 {
            // Scribbled entry bytes, never already empty past trial 0.
            let mut raw = [0u8; ENTRY_LEN];
            rng.bytes(&mut raw);
            if trial == 0 {
                raw = [0u8; ENTRY_LEN];
            }
            let mut full = vec![0u8; ENTRY_LEN].into_boxed_slice();
            full.copy_from_slice(&raw);
            let this = addr(&full[0]);
            let got = unsafe { fn_00A947D0::rw_00a947d0(this) };
            assert_eq!(got, 0, "trial {trial}");
            let back = read_back(this, ENTRY_LEN);
            assert_eq!(back, StreamEntry::empty().to_bytes(), "trial {trial}");
            // The wrong lift zeroes the link words: any case catches it.
            if StreamEntry::from_bytes(back.try_into().unwrap()) != wrong::init_zero_links() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(
            cases >= 40 && caught == cases,
            "cases={cases} caught={caught}"
        );
    }

    #[test]
    fn is_active_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAC71);
        let mut cases = 0;
        let mut caught = 0;
        for entry in entry_shapes(&mut rng) {
            let (_buf, this) = plant_entries(&[entry]);
            let got = unsafe { fn_00A94A20::rw_00a94a20(this) };
            assert_eq!(got, u8::from(entry.is_active()), "entry={entry:?}");
            if u8::from(wrong::active_bit10(entry)) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 8 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn block_count_matches() {
        let _guard = lock();
        let mut rng = Rng(0xB10C);
        let mut cases = 0;
        let mut caught = 0;
        let mut shapes = entry_shapes(&mut rng);
        // Targeted sizes around the 2048-unit boundary.
        for size in [0x2000u32, 0x2004, 0x3ffc, 0x4000, 0xffff_fffc, 0xffff_ffff] {
            shapes.push(StreamEntry::from_words([0, 0, size, 0, 0, 0]));
        }
        for entry in shapes {
            let (_buf, this) = plant_entries(&[entry]);
            let got = unsafe { fn_00A94380::rw_00a94380(this) };
            assert_eq!(got, entry.block_count(), "entry={entry:?}");
            if wrong::blocks_floor(entry) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 8 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn data_address_matches() {
        let _guard = lock();
        let mut rng = Rng(0xDA7A);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..30 {
            // Slot values: trial 0 pins small distinct values, else random.
            let mut slots = [0u32; 256];
            for (i, slot) in slots.iter_mut().enumerate() {
                *slot = if trial == 0 {
                    0x1000 + i as u32
                } else {
                    rng.u32()
                };
            }
            let lift_slots = DataSlots::from_slots(slots);
            // The 32-bit region: dwords at stride 160.
            let mut region = vec![0u8; 255 * SLOT_STRIDE + 4].into_boxed_slice();
            for (i, slot) in slots.iter().enumerate() {
                put_u32(&mut region, i * SLOT_STRIDE, *slot);
            }
            rt::set_relocated(DATA_VA, addr(&region[0]));
            let shapes = entry_shapes(&mut rng);
            for entry in &shapes {
                let (_buf, this) = plant_entries(&[*entry]);
                let got = unsafe { fn_00A94300::rw_00a94300(this) };
                assert_eq!(got, entry.data_address(&lift_slots), "entry={entry:?}");
                if wrong::data_bit10(*entry, &lift_slots) != got {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 200 && caught >= 1, "cases={cases} caught={caught}");
    }
    // Recording stub for the location routine's address callee.
    static LOCATE_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static LOCATE_SCRIPT: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "thiscall" fn locate_stub(ent: u32) -> u32 {
        LOCATE_LOG.lock().unwrap().push(ent);
        LOCATE_SCRIPT.lock().unwrap().pop().unwrap_or(0)
    }

    #[test]
    fn locate_matches() {
        let _guard = lock();
        rt::set_callee(0, locate_stub as *const () as usize as u32);
        let mut rng = Rng(0x10CA);
        let mut cases = 0;
        let mut caught = 0;
        for entry in entry_shapes(&mut rng) {
            let (_buf, this) = plant_entries(&[entry]);
            // Out words start scribbled; the empty path must leave them.
            let scribble_data = rng.u32();
            let scribble_blocks = rng.u32();
            let out_data = Box::new(scribble_data);
            let out_blocks = Box::new(scribble_blocks);
            let data_addr = addr(&*out_data);
            let blocks_addr = addr(&*out_blocks);
            let scripted = rng.u32();
            LOCATE_SCRIPT.lock().unwrap().push(scripted);
            LOCATE_LOG.lock().unwrap().clear();
            let got = unsafe { fn_00A94330::rw_00a94330(this, data_addr, blocks_addr) };
            // Read back through raw pointers (see `read_back`).
            let got_data = unsafe { (data_addr as *const u32).read_unaligned() };
            let got_blocks = unsafe { (blocks_addr as *const u32).read_unaligned() };
            let log = LOCATE_LOG.lock().unwrap().clone();
            let mut lift_calls = 0;
            let lift = entry.locate(&mut |seen: &StreamEntry| {
                assert_eq!(*seen, entry);
                lift_calls += 1;
                scripted
            });
            match lift {
                None => {
                    assert_eq!(got, 0, "entry={entry:?}");
                    assert!(log.is_empty(), "callee called for {entry:?}");
                    assert_eq!(lift_calls, 0);
                    // Untouched out words: the scribble survives.
                    assert_eq!(got_data, scribble_data, "entry={entry:?}");
                    assert_eq!(got_blocks, scribble_blocks, "entry={entry:?}");
                }
                Some((data, blocks)) => {
                    assert_eq!(got, 1, "entry={entry:?}");
                    assert_eq!(log, vec![this], "entry={entry:?}");
                    assert_eq!(lift_calls, 1);
                    assert_eq!((got_data, got_blocks), (data, blocks), "entry={entry:?}");
                }
            }
            let wrong_pair = wrong::locate_bit10(&entry, scripted);
            let rewrite_pair = if got == 0 {
                None
            } else {
                Some((got_data, got_blocks))
            };
            if wrong_pair != rewrite_pair {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 8 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn relative_slot_matches() {
        let _guard = lock();
        let mut rng = Rng(0x8E17);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..12 {
            // Distinct kind slots so a wrong kind byte always shows.
            let mut slots = [0u32; 256];
            for (i, slot) in slots.iter_mut().enumerate() {
                *slot = 0x5000 + (i as u32) * 4 + (rng.u32() & 0xffff_0000);
            }
            if trial == 0 {
                for (i, slot) in slots.iter_mut().enumerate() {
                    *slot = 0x5000 + i as u32;
                }
            }
            let kinds = KindSlots::from_slots(slots);
            // The 32-bit kind table: dwords at stride 100.
            let mut region = vec![0u8; 255 * KIND_SLOT_STRIDE + 4].into_boxed_slice();
            for (i, slot) in slots.iter().enumerate() {
                put_u32(&mut region, i * KIND_SLOT_STRIDE, *slot);
            }
            rt::set_relocated(KIND_TABLE_VA, addr(&region[0]));
            // Entries with kind bytes that differ from the value kind.
            let n = 1 + (trial % 8) as usize;
            let mut entries = vec![];
            for i in 0..n {
                let kind = rng.u32() as u8;
                let value_kind = (u32::from(kind).wrapping_add(1 + i as u32)) as u8;
                entries.push(StreamEntry::from_words([
                    rng.u32(),
                    u32::from(value_kind) | (rng.u32() & 0xffff_ff00),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    u32::from(kind) << 24 | (rng.u32() & 0x00ff_ffff),
                ]));
            }
            let table = SlotTable::from_parts(entries.clone(), n as i32);
            let (_buf, base) = plant_entries(&entries);
            let cell = Box::new(base);
            rt::set_relocated(TABLE_BASE_VA, addr(&*cell));
            for idx in 0..n as u32 {
                let this = base.wrapping_add(idx.wrapping_mul(ENTRY_LEN as u32));
                let got = unsafe { fn_00A94740::rw_00a94740(this) };
                assert_eq!(got, table.relative_slot(idx, &kinds), "idx={idx}");
                if wrong::slot_value_kind(&table, idx, &slots) != got {
                    caught += 1;
                }
                cases += 1;
            }
            // Sanity on the planted image while it is alive.
            assert_eq!(get_u32(&region, 0), slots[0]);
        }
        assert!(cases >= 12 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn is_slot_usable_matches() {
        let _guard = lock();
        let mut rng = Rng(0x05AB);
        let mut cases = 0;
        let mut caught = 0;
        let caps = [
            -2i32,
            -1,
            0,
            1,
            5,
            6,
            100,
            0x1_0000,
            i32::MAX - 6,
            i32::MAX - 5,
            i32::MAX,
        ];
        let idxs = [
            0u32,
            1,
            5,
            6,
            7,
            100,
            0xfffe,
            0xffff,
            0x1_0000,
            0x7fff_ffff,
            0x8000_0000,
            0xffff_ffff,
        ];
        for &cap in &caps {
            for &base_null in &[false, true] {
                // A live entry array behind non-null bases.
                let live = vec![StreamEntry::empty(), StreamEntry::empty()];
                let (_ebuf, base) = plant_entries(&live);
                let (obj, this) = plant_table(if base_null { 0 } else { base }, cap);
                let _ = obj;
                let table =
                    SlotTable::from_parts(if base_null { vec![] } else { live.clone() }, cap);
                for &idx in &idxs {
                    let got = unsafe { fn_00A94A40::rw_00a94a40(this, idx) };
                    assert_eq!(
                        got,
                        u8::from(table.is_slot_usable(idx)),
                        "cap={cap} null={base_null} idx={idx:#x}"
                    );
                    if u8::from(wrong::usable_no_sentinel(&table, idx)) != got {
                        caught += 1;
                    }
                    cases += 1;
                }
                // Random indexes around the slack edge.
                for _ in 0..4 {
                    let idx = (cap as u32).wrapping_add(rng.below(13)).wrapping_sub(6);
                    let got = unsafe { fn_00A94A40::rw_00a94a40(this, idx) };
                    assert_eq!(
                        got,
                        u8::from(table.is_slot_usable(idx)),
                        "cap={cap} null={base_null} idx={idx:#x}"
                    );
                    if u8::from(wrong::usable_no_sentinel(&table, idx)) != got {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 200 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn test_mask_matches() {
        let _guard = lock();
        let mut rng = Rng(0x7E57);
        let mut cases = 0;
        let mut caught = 0;
        // Flag words pinning the always-tested bits and their neighbours.
        let flag_words = [
            0x0000_0000u32,
            0x0001_0000,
            0x0002_0000,
            0x0004_0000,
            0x0040_0000,
            0x0080_0000,
            0x00c0_0000,
            0x00c6_0000,
            0x00ff_0000,
            0xffff_0000,
        ];
        let masks = [0u32, 1, 0x40, 0x80, 0xc6, 0x100, 0xffff, 0xffff_ffff];
        for trial in 0..8 {
            let mut entries = vec![];
            for &flags in &flag_words {
                entries.push(StreamEntry::from_words([
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    flags | (rng.u32() & 0x0000_ffff),
                    rng.u32(),
                    rng.u32(),
                ]));
            }
            if trial == 0 {
                entries.push(StreamEntry::empty());
            }
            let table = SlotTable::from_parts(entries.clone(), entries.len() as i32);
            let (_ebuf, base) = plant_entries(&entries);
            let (_obj, this) = plant_table(base, entries.len() as i32);
            for idx in 0..entries.len() as u32 {
                for &mask in &masks {
                    let got = unsafe { fn_00A94A70::rw_00a94a70(this, idx, mask) };
                    assert_eq!(
                        got,
                        u32::from(table.test_mask(idx, mask)),
                        "idx={idx} mask={mask:#x}"
                    );
                    if u32::from(wrong::mask_plain(&table, idx, mask)) != got {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 200 && caught >= 1, "cases={cases} caught={caught}");
    }
    #[test]
    fn table_entry_is_active_matches() {
        let _guard = lock();
        let mut rng = Rng(0x7AB1);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..10 {
            let shapes = entry_shapes(&mut rng);
            // The null-base path runs once per trial with an empty lift table.
            let (nobj, null_this) = plant_table(0, shapes.len() as i32);
            let _ = nobj;
            let empty = SlotTable::from_parts(vec![], shapes.len() as i32);
            let got = unsafe { fn_00A94AA0::rw_00a94aa0(null_this, 0) };
            assert_eq!(got, 0, "trial {trial}");
            assert!(!empty.entry_is_active(0));
            if wrong::entry_active_no_null(&empty, 0) {
                caught += 1;
            }
            cases += 1;
            // Live entries behind a real base.
            let table = SlotTable::from_parts(shapes.clone(), shapes.len() as i32);
            let (_ebuf, base) = plant_entries(&shapes);
            let (_obj, this) = plant_table(base, shapes.len() as i32);
            for idx in 0..shapes.len() as u32 {
                let got = unsafe { fn_00A94AA0::rw_00a94aa0(this, idx) };
                assert_eq!(got, u8::from(table.entry_is_active(idx)), "idx={idx}");
                cases += 1;
            }
        }
        assert!(cases >= 20 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn kind_flag_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1A6);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..10 {
            // Kind bytes are their offset's low byte: the stride-96 wrong
            // lift then differs from the rewrite on every nonzero kind,
            // in the byte, the residue, or both.
            let mut bytes = [0u8; KindBytes::LEN];
            for (i, byte) in bytes.iter_mut().enumerate() {
                *byte = i as u8;
            }
            if trial > 0 {
                rng.bytes(&mut bytes);
                // Keep one pinned byte so trial 1+ still pins the residue.
                bytes[160] = 0x5A;
            }
            let kinds = KindBytes::from_bytes(bytes);
            // The shared 32-bit region: kind bytes at offset 11.
            let mut region = vec![0u8; KIND_FLAG_OFF + KindBytes::LEN].into_boxed_slice();
            rng.bytes(&mut region[..KIND_FLAG_OFF]);
            region[KIND_FLAG_OFF..].copy_from_slice(&bytes);
            let region_base = addr(&region[0]);
            rt::set_relocated(DATA_VA, region_base);
            rt::set_relocated(KIND_FLAG_VA, region_base.wrapping_add(KIND_FLAG_OFF as u32));
            // Value kinds covering the ends and the middle.
            let mut value_kinds = vec![0u8, 1, 2, 3, 4, 127, 128, 254, 255];
            for _ in 0..8 {
                value_kinds.push(rng.u32() as u8);
            }
            let mut entries = vec![];
            for &value_kind in &value_kinds {
                entries.push(StreamEntry::from_words([
                    rng.u32(),
                    u32::from(value_kind) | (rng.u32() & 0xffff_ff00),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                ]));
            }
            let table = SlotTable::from_parts(entries.clone(), entries.len() as i32);
            let (_ebuf, base) = plant_entries(&entries);
            let (_obj, this) = plant_table(base, entries.len() as i32);
            for idx in 0..entries.len() as u32 {
                let got = unsafe { fn_00A94B30::rw_00a94b30(this, idx) };
                assert_eq!(got, table.kind_flag(idx, &kinds), "idx={idx}");
                if wrong::kind_flag_96(&table, idx, &kinds) != got {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn set_flag_bit15_matches() {
        let _guard = lock();
        let mut rng = Rng(0x5E7);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..12 {
            let mut shapes = entry_shapes(&mut rng);
            if trial == 0 {
                // Flags without bits 14/15: the wrong bit always shows.
                shapes = vec![
                    StreamEntry::from_words([1, 2, 3, 0x0000_0004, 5, 6]),
                    StreamEntry::from_words([1, 2, 3, 0x0800_0000, 5, 6]),
                ];
            }
            let mut table = SlotTable::from_parts(shapes.clone(), shapes.len() as i32);
            let (_ebuf, base) = plant_entries(&shapes);
            let (_obj, this) = plant_table(base, shapes.len() as i32);
            for idx in 0..shapes.len() as u32 {
                table.set_flag_bit15(idx);
                let got = unsafe { fn_00A94CE0::rw_00a94ce0(this, idx) };
                // The rewrite answers the planted table base.
                assert_eq!(got, base, "idx={idx}");
                // Every written byte matches the lifted table.
                let back = read_back(base, shapes.len() * ENTRY_LEN);
                let mut expect = vec![0u8; shapes.len() * ENTRY_LEN];
                for (i, entry) in table.entries().iter().enumerate() {
                    expect[i * ENTRY_LEN..(i + 1) * ENTRY_LEN].copy_from_slice(&entry.to_bytes());
                }
                assert_eq!(back, expect, "idx={idx}");
                // The wrong lift sets bit 14: compare its flag word.
                let mut wrong_entry = shapes[idx as usize];
                wrong::set_bit14(&mut wrong_entry);
                let at = idx as usize * ENTRY_LEN;
                let wrong_bytes = wrong_entry.to_bytes();
                if back[at..at + ENTRY_LEN] != wrong_bytes {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 20 && caught >= 1, "cases={cases} caught={caught}");
    }
}
