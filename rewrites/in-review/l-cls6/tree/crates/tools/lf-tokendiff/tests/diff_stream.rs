//! Differential cases: the streaming device.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_files_memory::tokenizer::{
        StreamChannel, StreamDevice, StreamEntry, StreamTables,
    };
    use lf_tokendiff::rewrites::*;
    use lf_tokendiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{StreamCall, StreamFake, Image, Rng, U32_EDGE, VTable, cookie, raw};

    // Streaming global VAs, as the verified rewrites name them.
    const SPAN_LO: u32 = 0x012F_B3B8;
    const SPAN_HI: u32 = 0x012F_B3BC;
    const SLOT2: u32 = 0x012F_B44C;
    const SLOT1: u32 = 0x012F_B450;
    const TBASE: u32 = 0x012F_B3A8;
    // Per-kind row stride in the global tables.
    const ROW: usize = 160;
    // Entry stride in the entry table.
    const ENTRY: usize = 24;

    // Object field offsets, as the verified rewrites use them.
    const OBJ_ELEMS: usize = 0x04;
    const OBJ_BASE: usize = 0x08;
    const OBJ_FLAG: usize = 0x0C;

    // Entry field offsets.
    const ENT_FIRST: usize = 0x00;
    const ENT_KIND: usize = 0x04;
    const ENT_SIZE: usize = 0x08;
    const ENT_FLAGS: usize = 0x0E;

    // Channel row field offsets.
    const CH_LO: usize = 0x08;
    const CH_HI: usize = 0x0C;
    const CH_OBJ: usize = 0x90;
    const CH_SINK: usize = 0x98;

    /// The streaming fixture: device object, element tables, entries,
    /// global tables, and channel objects, kept alive together.
    struct Fixture {
        obj: Image,
        mid: Image,
        elem_base: Image,
        entries: Image,
        tbase: Image,
        span_lo: Image,
        span_hi: Image,
        slot1: Image,
        slot2: Image,
        chan_vtable: VTable,
        chan_objs: Vec<Image>,
        kinds: Vec<u8>,
    }

    impl Fixture {
        fn build(rng: &mut Rng, n_entries: usize, n_elems: usize) -> Self {
            let mut obj = Image::zeroed(0x10);
            let mut elem_base = Image::zeroed(n_elems * ENTRY);
            let kinds: Vec<u8> = (0..n_elems).map(|_| rng.u32() as u8).collect();
            for (i, k) in kinds.iter().enumerate() {
                elem_base.w8(i * ENTRY + ENT_KIND, *k);
            }
            let mut mid = Image::zeroed(4);
            mid.w32(0, elem_base.addr());
            obj.w32(OBJ_ELEMS, mid.addr());
            let entries = Image::zeroed(n_entries * ENTRY);
            let mut tbase = Image::zeroed(4);
            tbase.w32(0, entries.addr());
            let span_lo = Image::zeroed(256 * ROW);
            let span_hi = Image::zeroed(256 * ROW);
            let slot1 = Image::zeroed(256 * ROW);
            let slot2 = Image::zeroed(256 * ROW);
            let mut chan_vtable = VTable::zeroed(0x40);
            chan_vtable.set(0x80, support::stub_close as *const () as usize as u32);
            chan_vtable.set(0x18, support::stub_post as *const () as usize as u32);
            Self {
                obj,
                mid,
                elem_base,
                entries,
                tbase,
                span_lo,
                span_hi,
                slot1,
                slot2,
                chan_vtable,
                chan_objs: Vec::new(),
                kinds,
            }
        }

        fn plant_entry(&mut self, idx: usize, first: u32, kind: u8, size: u32, flags: u16) {
            let base = idx * ENTRY;
            self.entries.w32(base + ENT_FIRST, first);
            self.entries.w8(base + ENT_KIND, kind);
            self.entries.w32(base + ENT_SIZE, size);
            self.entries.w16(base + ENT_FLAGS, flags);
        }

        fn entry_addr(&self, idx: usize) -> u32 {
            self.entries.addr().wrapping_add((idx * ENTRY) as u32)
        }

        fn entry(&self, idx: usize) -> StreamEntry {
            let base = idx * ENTRY;
            StreamEntry {
                offset: (idx * ENTRY) as i32,
                first: self.entries.r32(base + ENT_FIRST),
                kind: self.entries.r8(base + ENT_KIND),
                size: self.entries.r32(base + ENT_SIZE),
                flags: self.entries.r16(base + ENT_FLAGS),
            }
        }

        fn fill_tables(&mut self, tables: &StreamTables) {
            for (kind, row) in tables.spans.iter().enumerate() {
                self.span_lo.w32(kind * ROW, row.0);
                self.span_hi.w32(kind * ROW, row.1);
            }
            for (kind, word) in tables.slots_v1.iter().enumerate() {
                self.slot1.w32(kind * ROW, *word);
            }
            for (kind, word) in tables.slots_v2.iter().enumerate() {
                self.slot2.w32(kind * ROW, *word);
            }
            for (kind, ch) in tables.channels.iter().enumerate() {
                self.span_lo.w32(kind * ROW + CH_LO, ch.cursor_lo);
                self.span_lo.w32(kind * ROW + CH_HI, ch.cursor_hi);
                self.span_lo.w32(kind * ROW + CH_OBJ, raw(ch.object));
                self.span_lo.w32(kind * ROW + CH_SINK, ch.sink_arg);
            }
        }

        fn plant_channel_obj(&mut self) -> u32 {
            let mut img = Image::zeroed(4);
            img.w32(0, self.chan_vtable.addr());
            let addr = img.addr();
            self.chan_objs.push(img);
            addr
        }

        fn install_globals(&self) {
            rt::set_globals(&[
                (SPAN_LO, self.span_lo.addr()),
                (SPAN_HI, self.span_hi.addr()),
                (SLOT1, self.slot1.addr()),
                (SLOT2, self.slot2.addr()),
                (TBASE, self.tbase.addr()),
            ]);
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        fn lift(&self, base_key: u32, close_first: bool) -> StreamDevice {
            StreamDevice {
                base_key,
                elem_kinds: self.kinds.clone(),
                close_first,
            }
        }
    }

    fn tables_random(rng: &mut Rng, objects: &[u32]) -> StreamTables {
        let mut channels = Vec::with_capacity(256);
        for i in 0..256 {
            let obj = if i < objects.len() {
                cookie(objects[i])
            } else {
                None
            };
            channels.push(StreamChannel {
                cursor_lo: rng.u32(),
                cursor_hi: rng.u32(),
                object: obj,
                sink_arg: rng.u32(),
            });
        }
        StreamTables {
            spans: (0..256).map(|_| (rng.u32(), rng.u32())).collect(),
            slots_v1: (0..256).map(|_| rng.u32()).collect(),
            slots_v2: (0..256).map(|_| rng.u32()).collect(),
            channels,
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_files_memory::tokenizer::{StreamDevice, StreamTables, StreamWorld};

        /// Requires both the size and the present bit (AND instead of OR).
        pub fn active_and<W: StreamWorld>(d: &StreamDevice, w: &mut W, delta: u32) -> bool {
            let Some(e) = w.resolve(d.base_key.wrapping_add(delta)) else {
                return false;
            };
            e.size & 0xFFFF_FFFC != 0 && (e.flags as u32 >> 11) & 1 != 0
        }

        /// Flips the low bit of the shifted answer.
        pub fn quarter_flip<W: StreamWorld>(d: &StreamDevice, w: &mut W, delta: u32) -> u32 {
            let Some(e) = w.resolve(d.base_key.wrapping_add(delta)) else {
                return 0;
            };
            w.shifted(&e) ^ 1
        }

        /// Swaps the pair.
        pub fn span_swapped<W: StreamWorld>(
            d: &StreamDevice,
            w: &mut W,
            t: &StreamTables,
            delta: u32,
        ) -> (u32, u32) {
            let e = w.resolve(d.base_key.wrapping_add(delta)).unwrap();
            let (lo, hi) = t.spans[e.kind as usize];
            (hi, lo)
        }

        /// Tests gate bit 12 instead of 13.
        pub fn gated_bit12<W: StreamWorld>(d: &StreamDevice, w: &mut W, delta: u32) -> Option<u32> {
            let e = w.resolve(d.base_key.wrapping_add(delta))?;
            if (e.flags as u32 >> 12) & 1 == 0 {
                return None;
            }
            Some(e.first)
        }

        /// Selects the next kind's slot.
        pub fn slot_next<W: StreamWorld>(
            d: &StreamDevice,
            w: &mut W,
            t: &StreamTables,
            delta: u32,
        ) -> u32 {
            let e = w.resolve(d.base_key.wrapping_add(delta)).unwrap();
            t.slots_v1[e.kind.wrapping_add(1) as usize]
        }

        /// Scales by 10 bits instead of 11.
        pub fn span_shift10<W: StreamWorld>(
            d: &StreamDevice,
            w: &mut W,
            t: &StreamTables,
            delta: u32,
        ) -> Option<(u32, u32)> {
            let e = w.resolve(d.base_key.wrapping_add(delta))?;
            let data = w.data_word(&e);
            let slot = t.slots_v2[e.kind as usize];
            let off = data.wrapping_sub(slot).wrapping_shl(10);
            let hi = ((i64::from(e.offset).wrapping_mul(0x2AAAAAAB)) >> 32) as i32;
            let s = hi >> 2;
            let slot_no = s.wrapping_add(((s as u32) >> 31) as i32) as u16 as u32;
            Some((off, slot_no))
        }

        /// Forgets the carry into the high word.
        pub fn post_no_carry<W: StreamWorld>(
            d: &StreamDevice,
            w: &mut W,
            t: &StreamTables,
            a0: u32,
            a1: u32,
            a2: u32,
            a3: u32,
            a4: u32,
        ) -> (u32, u32) {
            let kind = d.elem_kinds[a0 as usize];
            let ch = &t.channels[kind as usize];
            let lo = ch.cursor_lo.wrapping_add(a1);
            let hi = ch.cursor_hi.wrapping_add(a2);
            let _ = (a3, a4);
            let _ = w;
            (lo, hi)
        }
    }

    /// Flag words covering the present and gate bits both ways.
    fn flag_cases() -> Vec<u16> {
        let mut v = vec![0x0000, 0xFFFF];
        for bit in [10, 11, 12, 13, 14] {
            v.push(1 << bit);
            v.push(!(1 << bit) & 0xFFFF);
        }
        v.push(0x0800 | 0x2000);
        v.push(0x0800);
        v.push(0x2000);
        v
    }

    #[test]
    fn device_active_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xA27);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 4, 2);
        fx.plant_entry(0, 0x1111_1111, 7, 0, 0);
        fx.plant_entry(1, 0x2222_2222, 9, 4, 0);
        fx.plant_entry(2, 0x3333_3333, 3, 0, 1 << 11);
        fx.plant_entry(3, 0x4444_4444, 5, 3, 0);
        let base_key = 0x1000;
        fx.obj.w32(OBJ_BASE, base_key);
        let mut run = |fx: &mut Fixture, entry: Option<usize>, flags: u16, catch: &mut u32| {
            if let Some(idx) = entry {
                let base = idx * ENTRY;
                fx.entries.w16(base + ENT_FLAGS, flags);
            }
            let addr = entry.map(|i| fx.entry_addr(i)).unwrap_or(0);
            rt::set_script(&[(0, StubKind::Thiscall2, vec![addr])]);
            fx.install_globals();
            let delta = 0x40;
            let got = unsafe { fn_00A942B0::rw_00a942b0(fx.this(), delta) };
            let dev = fx.lift(base_key, false);
            let mut fake = StreamFake::new();
            fake.resolve.push_back(entry.map(|i| fx.entry(i)));
            let lift = dev.is_active(&mut fake, delta);
            assert_eq!(got, u32::from(lift), "entry {entry:?} flags {flags:#x}");
            assert!(
                got == 1 || got == 0xFFFF_FFFF,
                "unexpected word {got:#x}"
            );
            assert_eq!(
                rt::take_numbered().iter().map(|c| (c.id, c.args.clone())).collect::<Vec<_>>(),
                vec![(0, vec![fx.this(), base_key.wrapping_add(delta)])],
                "rewrite calls"
            );
            assert_eq!(
                fake.log,
                vec![StreamCall::Resolve(base_key.wrapping_add(delta))],
                "lift calls"
            );
            let mut fake2 = StreamFake::new();
            fake2.resolve.push_back(entry.map(|i| fx.entry(i)));
            if wrong::active_and(&dev, &mut fake2, delta) != lift {
                *catch += 1;
            }
            cases += 1;
        };
        run(&mut fx, None, 0, &mut caught);
        for idx in 0..4 {
            for &flags in &flag_cases() {
                run(&mut fx, Some(idx), flags, &mut caught);
            }
        }
        // Size words around the mask, with the present bit clear.
        for &size in &[0u32, 1, 2, 3, 4, 5, 0x8000_0000, 0xFFFF_FFFC, 0xFFFF_FFFF] {
            fx.entries.w32(ENT_SIZE, size);
            fx.entries.w16(ENT_FLAGS, 0);
            run(&mut fx, Some(0), 0, &mut caught);
        }
        assert!(caught > 0, "wrong active never caught ({cases} cases)");
    }

    #[test]
    fn device_quarter_size_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x9439);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 2, 1);
        fx.plant_entry(0, 1, 2, 3, 4);
        fx.plant_entry(1, 5, 6, 7, 8);
        let base_key = rng.u32();
        fx.obj.w32(OBJ_BASE, base_key);
        let mut run = |fx: &mut Fixture, rng: &mut Rng, entry: Option<usize>, shifted: u32, catch: &mut u32| {
            let addr = entry.map(|i| fx.entry_addr(i)).unwrap_or(0);
            rt::set_script(&[
                (0, StubKind::Thiscall2, vec![addr]),
                (1, StubKind::Thiscall1, vec![shifted]),
            ]);
            fx.install_globals();
            let delta = rng.u32();
            let got = unsafe { fn_00A94390::rw_00a94390(fx.this(), delta) };
            let dev = fx.lift(base_key, false);
            let mut fake = StreamFake::new();
            fake.resolve.push_back(entry.map(|i| fx.entry(i)));
            fake.shifted.push_back(shifted);
            let lift = dev.quarter_size(&mut fake, delta);
            assert_eq!(got, u64::from(lift), "entry {entry:?}");
            assert_eq!(got >> 32, 0, "high half not cleared");
            let numbered = rt::take_numbered();
            assert_eq!(numbered[0].id, 0);
            assert_eq!(numbered[0].args, vec![fx.this(), base_key.wrapping_add(delta)]);
            assert_eq!(
                fake.log[0],
                StreamCall::Resolve(base_key.wrapping_add(delta))
            );
            if entry.is_some() {
                assert_eq!(numbered.len(), 2);
                assert_eq!(numbered[1], rt::NumberedCall { id: 1, args: vec![addr], snaps: vec![] });
                assert_eq!(fake.log.len(), 2);
            } else {
                assert_eq!(numbered.len(), 1);
                assert_eq!(fake.log.len(), 1);
            }
            let mut fake2 = StreamFake::new();
            fake2.resolve.push_back(entry.map(|i| fx.entry(i)));
            fake2.shifted.push_back(shifted);
            if wrong::quarter_flip(&dev, &mut fake2, delta) != lift {
                *catch += 1;
            }
            cases += 1;
        };
        run(&mut fx, &mut rng, None, 0, &mut caught);
        for &shifted in U32_EDGE {
            run(&mut fx, &mut rng, Some(0), shifted, &mut caught);
            run(&mut fx, &mut rng, Some(1), shifted, &mut caught);
        }
        for _ in 0..32 {
            run(&mut fx, &mut rng, Some(rng.below(2) as usize), rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong quarter size never caught ({cases} cases)");
    }

    #[test]
    fn device_span_pair_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x943C);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 8, 1);
        let tables = tables_random(&mut rng, &[]);
        fx.fill_tables(&tables);
        let base_key = rng.u32();
        fx.obj.w32(OBJ_BASE, base_key);
        let mut kinds: Vec<u8> = vec![0, 1, 2, 127, 128, 254, 255];
        for _ in 0..8 {
            kinds.push(rng.u32() as u8);
        }
        for (i, &kind) in kinds.iter().enumerate() {
            let idx = i % 8;
            fx.plant_entry(idx, rng.u32(), kind, rng.u32(), rng.u32() as u16);
            let addr = fx.entry_addr(idx);
            rt::set_script(&[(0, StubKind::Thiscall2, vec![addr])]);
            fx.install_globals();
            let delta = rng.u32();
            let got = unsafe { fn_00A943C0::rw_00a943c0(fx.this(), delta) };
            let dev = fx.lift(base_key, false);
            let mut fake = StreamFake::new();
            fake.resolve.push_back(Some(fx.entry(idx)));
            let lift = dev.span_pair(&mut fake, &tables, delta);
            assert_eq!(got, (u64::from(lift.1) << 32) | u64::from(lift.0), "kind {kind}");
            assert_eq!(
                rt::take_numbered().len(),
                1,
                "one resolve call"
            );
            assert_eq!(fake.log.len(), 1);
            let mut fake2 = StreamFake::new();
            fake2.resolve.push_back(Some(fx.entry(idx)));
            if wrong::span_swapped(&dev, &mut fake2, &tables, delta) != lift {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong span pair never caught ({cases} cases)");
    }

    #[test]
    fn device_gated_word_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x9479);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 2, 1);
        fx.plant_entry(0, 0xDEAD_BEEF, 4, 0, 0);
        fx.plant_entry(1, 0x1234_5678, 6, 0, 0);
        let base_key = rng.u32();
        fx.obj.w32(OBJ_BASE, base_key);
        let mut run = |fx: &mut Fixture, entry: Option<usize>, flags: u16, catch: &mut u32| {
            if let Some(idx) = entry {
                fx.entries.w16(idx * ENTRY + ENT_FLAGS, flags);
            }
            let addr = entry.map(|i| fx.entry_addr(i)).unwrap_or(0);
            rt::set_script(&[(0, StubKind::Thiscall2, vec![addr])]);
            fx.install_globals();
            let delta = 0x20;
            let mut out = Image::zeroed(4);
            let before = out.buf.to_vec();
            let got = unsafe { fn_00A94790::rw_00a94790(fx.this(), delta, out.addr()) };
            let dev = fx.lift(base_key, false);
            let mut fake = StreamFake::new();
            fake.resolve.push_back(entry.map(|i| fx.entry(i)));
            let lift = dev.gated_first_word(&mut fake, delta);
            match lift {
                Some(word) => {
                    assert_eq!(got, 0xFFFF_FFFF, "flags {flags:#x}");
                    assert_eq!(out.r32(0), word, "written word");
                }
                None => {
                    assert_eq!(got, 0, "flags {flags:#x}");
                    assert_eq!(out.buf.to_vec(), before, "nothing written");
                }
            }
            assert_eq!(rt::take_numbered().len(), 1);
            assert_eq!(fake.log.len(), 1);
            let mut fake2 = StreamFake::new();
            fake2.resolve.push_back(entry.map(|i| fx.entry(i)));
            if wrong::gated_bit12(&dev, &mut fake2, delta) != lift {
                *catch += 1;
            }
            cases += 1;
        };
        run(&mut fx, None, 0, &mut caught);
        for idx in [0, 1] {
            for &flags in &flag_cases() {
                run(&mut fx, Some(idx), flags, &mut caught);
            }
        }
        assert!(caught > 0, "wrong gated word never caught ({cases} cases)");
    }

    #[test]
    fn device_slot_word_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x94D0);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 4, 1);
        let tables = tables_random(&mut rng, &[]);
        fx.fill_tables(&tables);
        let base_key = rng.u32();
        fx.obj.w32(OBJ_BASE, base_key);
        for i in 0..64 {
            let idx = i % 4;
            let kind = if i < 8 {
                [0u8, 1, 127, 128, 200, 254, 255, 100][i]
            } else {
                rng.u32() as u8
            };
            fx.plant_entry(idx, rng.u32(), kind, rng.u32(), rng.u32() as u16);
            let addr = fx.entry_addr(idx);
            rt::set_script(&[(0, StubKind::Thiscall2, vec![addr])]);
            fx.install_globals();
            let delta = rng.u32();
            let unused = rng.u32();
            let got = unsafe { fn_00A94D00::rw_00a94d00(fx.this(), delta, unused) };
            let dev = fx.lift(base_key, false);
            let mut fake = StreamFake::new();
            fake.resolve.push_back(Some(fx.entry(idx)));
            let lift = dev.slot_word(&mut fake, &tables, delta);
            assert_eq!(got, lift, "kind {kind}");
            assert_eq!(rt::take_numbered().len(), 1);
            assert_eq!(fake.log.len(), 1);
            let mut fake2 = StreamFake::new();
            fake2.resolve.push_back(Some(fx.entry(idx)));
            if wrong::slot_next(&dev, &mut fake2, &tables, delta) != lift {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong slot word never caught ({cases} cases)");
    }

    #[test]
    fn device_data_span_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x94D2);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 6, 1);
        let tables = tables_random(&mut rng, &[]);
        fx.fill_tables(&tables);
        let base_key = rng.u32();
        fx.obj.w32(OBJ_BASE, base_key);
        let mut run = |fx: &mut Fixture, rng: &mut Rng, entry: Option<usize>, data: u32, catch: &mut u32| {
            let addr = entry.map(|i| fx.entry_addr(i)).unwrap_or(0);
            rt::set_script(&[
                (0, StubKind::Thiscall2, vec![addr]),
                (1, StubKind::Thiscall1, vec![data]),
            ]);
            fx.install_globals();
            let delta = rng.u32();
            let mut out = Image::zeroed(8);
            let got = unsafe { fn_00A94D20::rw_00a94d20(fx.this(), delta, out.addr()) };
            let dev = fx.lift(base_key, false);
            let mut fake = StreamFake::new();
            fake.resolve.push_back(entry.map(|i| fx.entry(i)));
            fake.data.push_back(data);
            let lift = dev.data_span(&mut fake, &tables, delta);
            match lift {
                Some(span) => {
                    assert_eq!(out.r32(0), span.offset, "offset");
                    assert_eq!(out.r32(4), 0, "high word pinned zero");
                    assert_eq!(got, span.slot, "slot");
                }
                None => {
                    assert_eq!(got, 0xFFFF_FFFF, "unresolved");
                }
            }
            let numbered = rt::take_numbered();
            assert_eq!(numbered[0].id, 0);
            if entry.is_some() {
                assert_eq!(numbered.len(), 2);
                assert_eq!(numbered[1].args, vec![addr]);
                assert_eq!(fake.log.len(), 2);
            } else {
                assert_eq!(numbered.len(), 1);
                assert_eq!(fake.log.len(), 1);
            }
            let mut fake2 = StreamFake::new();
            fake2.resolve.push_back(entry.map(|i| fx.entry(i)));
            fake2.data.push_back(data);
            let wrong = wrong::span_shift10(&dev, &mut fake2, &tables, delta);
            let right = lift.map(|s| (s.offset, s.slot));
            if wrong != right {
                *catch += 1;
            }
            cases += 1;
        };
        run(&mut fx, &mut rng, None, 0, &mut caught);
        for i in 0..6 {
            fx.plant_entry(i, rng.u32(), (i * 40) as u8, rng.u32(), rng.u32() as u16);
        }
        for &data in U32_EDGE {
            run(&mut fx, &mut rng, Some(0), data, &mut caught);
            run(&mut fx, &mut rng, Some(5), data, &mut caught);
        }
        for _ in 0..32 {
            run(&mut fx, &mut rng, Some(rng.below(6) as usize), rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong data span never caught ({cases} cases)");
    }

    #[test]
    fn device_post_span_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x94F3);
        let mut caught = 0;
        let mut cases = 0;
        let mut fx = Fixture::build(&mut rng, 1, 4);
        let obj0 = fx.plant_channel_obj();
        let obj1 = fx.plant_channel_obj();
        // Kinds of elements 0..4 select planted channels; element 3 has
        // no object (null row).
        let elem_kinds = [11u8, 200, 0, 77];
        for (i, k) in elem_kinds.iter().enumerate() {
            fx.elem_base.w8(i * ENTRY + ENT_KIND, *k);
        }
        fx.kinds = elem_kinds.to_vec();
        let tables = tables_random(&mut rng, &[]);
        let mut tables = tables;
        tables.channels[11] = StreamChannel {
            cursor_lo: 0xFFFF_FFFF,
            cursor_hi: 0x0000_0001,
            object: cookie(obj0),
            sink_arg: 0xAAAA_AAAA,
        };
        tables.channels[200] = StreamChannel {
            cursor_lo: 0x1234_5678,
            cursor_hi: 0x9ABC_DEF0,
            object: cookie(obj1),
            sink_arg: 0x5555_5555,
        };
        tables.channels[77] = StreamChannel {
            cursor_lo: 0,
            cursor_hi: 0,
            object: None,
            sink_arg: 0xDEAD_BEEF,
        };
        fx.fill_tables(&tables);
        let mut run = |fx: &mut Fixture, rng: &mut Rng, a0: u32, close_first: bool, a1: u32, a2: u32, catch: &mut u32| {
            fx.obj.w8(OBJ_FLAG, u8::from(close_first));
            rt::set_script(&[]);
            fx.install_globals();
            let sink_answer = rng.u32();
            rt::set_virtual(&[("close", vec![0]), ("post", vec![sink_answer])]);
            let (a3, a4) = (rng.u32(), rng.u32());
            let got = unsafe { fn_00A94F30::rw_00a94f30(fx.this(), a0, a1, a2, a3, a4) };
            let dev = fx.lift(0, close_first);
            let mut fake = StreamFake::new();
            fake.post.push_back(sink_answer);
            let lift = dev.post_span_for(&mut fake, &tables, a0, a1, a2, a3, a4);
            assert_eq!(got, lift, "a0 {a0}");
            // The expected calls.
            let kind = elem_kinds[a0 as usize];
            let ch = &tables.channels[kind as usize];
            let lo = ch.cursor_lo.wrapping_add(a1);
            let hi = ch
                .cursor_hi
                .wrapping_add(a2)
                .wrapping_add(u32::from(lo < a1));
            let virtual_calls = rt::take_virtual();
            let mut expect_virtual = Vec::new();
            let mut expect_fake = Vec::new();
            if close_first {
                expect_virtual.push(("close".to_string(), vec![raw(ch.object)], vec![]));
                expect_fake.push(StreamCall::Close(raw(ch.object)));
            }
            expect_virtual.push((
                "post".to_string(),
                vec![raw(ch.object), ch.sink_arg, lo, hi, a3, a4],
                vec![],
            ));
            expect_fake.push(StreamCall::Post(raw(ch.object), ch.sink_arg, lo, hi, a3, a4));
            assert_eq!(virtual_calls, expect_virtual, "rewrite calls");
            assert_eq!(fake.log, expect_fake, "lift calls");
            assert!(rt::take_numbered().is_empty(), "no numbered calls");
            let (wlo, whi) = wrong::post_no_carry(&dev, &mut fake, &tables, a0, a1, a2, a3, a4);
            if (wlo, whi) != (lo, hi) {
                *catch += 1;
            }
            cases += 1;
        };
        for &a0 in &[0u32, 1, 2, 3] {
            for &close in &[false, true] {
                // Carry and no-carry cursor pairs.
                run(&mut fx, &mut rng, a0, close, 0, 0, &mut caught);
                run(&mut fx, &mut rng, a0, close, 1, 0, &mut caught);
                run(&mut fx, &mut rng, a0, close, 0xFFFF_FFFF, 0xFFFF_FFFF, &mut caught);
                run(&mut fx, &mut rng, a0, close, rng.u32(), rng.u32(), &mut caught);
            }
        }
        for _ in 0..16 {
            run(&mut fx, &mut rng, rng.below(4), rng.u32() & 1 != 0, rng.u32(), rng.u32(), &mut caught);
        }
        assert!(caught > 0, "wrong post span never caught ({cases} cases)");
    }
}
