//! Differential cases: each lifted slot against every verified rewrite.
//!
//! 32-bit target only (the fetch stubs fill the rewrites' out-blocks
//! through real addresses). On other hosts this file builds one empty test.

#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_lb_diff::rt;
    use lf_lb_diff::rt::Call;
    use lf_lb_diff::rt::FetchLayout;
    use lf_lb_diff::rt::FetchScript;
    use lf_leaderboard::desc::LeaderboardDesc;
    use lf_leaderboard::tables;

    /// One verified rewrite under test (generated fields; see `diff_gen.rs`).
    pub struct Case {
        /// Rewrite file stem, for failure messages.
        pub file: &'static str,
        /// Board id the rewrite fetches by.
        pub board: u32,
        /// Fetch callee slot used by this rewrite.
        pub fetch_slot: u32,
        /// Classify callee slot (`u32::MAX` when the slot has none).
        pub class_slot: u32,
        /// 1 when the rewrite declares the classifier fastcall, else 0.
        pub class_fastcall: u32,
        /// The rewrite as a plain function of (`this`, argument).
        pub call: fn(u32, u32) -> u32,
    }

    /// One verified probe rewrite under test.
    pub struct Case2 {
        /// Rewrite file stem, for failure messages.
        pub file: &'static str,
        /// Tag file VA (relocated by most rewrites, hardcoded by six).
        pub tag: u32,
        /// 1 when the rewrite stores the tag literally.
        pub hardcoded: u32,
        /// The rewrite as a plain function.
        pub call: fn(u32, u32, u32) -> u32,
    }

    include!("support/diff_gen.rs");

    /// Small deterministic generator.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn u32(&mut self) -> u32 {
            (self.next() >> 32) as u32
        }
    }

    /// Classifier answers the script cycles through.
    const ANSWERS: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 0xFFFF_FFFF];

    /// The scripted classifier: a pure function of the entry value.
    fn script(cell: u32) -> u32 {
        let h = cell ^ cell.rotate_right(16) ^ 0x5BD1_E995;
        ANSWERS[(h % 8) as usize]
    }

    /// One cell value per scripted answer (brute-forced once per slot).
    fn forced_cells() -> [u32; 8] {
        let mut out = [0u32; 8];
        let mut found = [false; 8];
        let mut cell = 0u32;
        while found.iter().any(|f| !f) {
            let a = script(cell);
            if let Some(i) = ANSWERS.iter().position(|x| *x == a) {
                if !found[i] {
                    found[i] = true;
                    out[i] = cell;
                }
            }
            cell = cell.wrapping_add(1);
        }
        out
    }

    /// The test image: table A in words `0..TABLE_WORDS`, table B in
    /// `512..512 + TABLE_WORDS`. Both sides read the same backing words.
    const TABLE_WORDS: usize = 320;

    struct Image {
        words: Box<[u32; 1024]>,
    }

    impl Image {
        fn new() -> Self {
            Self {
                words: Box::new([0u32; 1024]),
            }
        }

        /// Address of word `w` (32-bit target: a real address).
        fn addr(&self, w: usize) -> u32 {
            (&self.words[w] as *const u32).addr() as u32
        }

        fn fill(&mut self, seed: u64) {
            let mut rng = Rng(seed);
            for w in self.words.iter_mut() {
                *w = rng.u32();
            }
        }
    }

    /// The lift side's fake collaborator: scripted data, translated calls.
    struct Fake<'a> {
        fetch_slot: u32,
        class_slot: u32,
        class_fastcall: bool,
        view: Option<tables::BoardView<'a>>,
    }

    impl tables::BoardTables for Fake<'_> {
        fn fetch(&mut self, board: u32) -> Option<tables::BoardView<'_>> {
            rt::record(self.fetch_slot, &[board]);
            self.view
        }

        fn classify(&mut self, value: u32) -> u32 {
            // The translation drops the dummy second word of
            // fastcall-declared classify calls (unobserved by the callee).
            rt::record(self.class_slot, &[value]);
            let _ = self.class_fastcall;
            script(value)
        }
    }

    /// One generated input.
    struct Input {
        ok: bool,
        count: u32,
        arg: u32,
        seed: u64,
        plant: u8,
    }

    const EDGE_U32: [u32; 10] = [
        0,
        1,
        2,
        3,
        0x7FFF_FFFF,
        0x8000_0000,
        0xFFFF_FFFE,
        0xFFFF_FFFF,
        0x5E,
        0xDEAD_BEEF,
    ];

    /// The six slot plans: layout, counts, and how the argument reads.
    #[derive(Clone, Copy)]
    enum Slot {
        Vf6,
        Vf7,
        Vf8,
        Vf9,
        Vf12,
        Vf13,
    }

    impl Slot {
        fn layout(self) -> FetchLayout {
            match self {
                // count@3, table@4
                Self::Vf6 => FetchLayout {
                    count_idx: Some(3),
                    table_a_idx: 4,
                    table_b_idx: None,
                },
                // table@4 only (no count read)
                Self::Vf7 => FetchLayout {
                    count_idx: None,
                    table_a_idx: 4,
                    table_b_idx: None,
                },
                // table@5 only (no count read)
                Self::Vf8 | Self::Vf9 => FetchLayout {
                    count_idx: None,
                    table_a_idx: 5,
                    table_b_idx: None,
                },
                // count@1, tableB@2, tableA@5
                Self::Vf12 => FetchLayout {
                    count_idx: Some(1),
                    table_a_idx: 5,
                    table_b_idx: Some(2),
                },
                // count@3, tableA@4, tableB@5
                Self::Vf13 => FetchLayout {
                    count_idx: Some(3),
                    table_a_idx: 4,
                    table_b_idx: Some(5),
                },
            }
        }

        fn counts(self) -> &'static [u32] {
            match self {
                Self::Vf6 | Self::Vf13 => &[
                    0,
                    1,
                    2,
                    3,
                    7,
                    64,
                    300,
                    0xFFFF_FFFF,
                    0x8000_0000,
                    0x8000_0001,
                    5,
                ],
                Self::Vf12 => &[0, 1, 2, 3, 7, 64, 300, 5],
                // Unread by these slots; varied to prove it.
                Self::Vf7 | Self::Vf8 | Self::Vf9 => &[0, 1, 0xFFFF_FFFF],
            }
        }

        /// True for the key-taking slots (vf6, vf13).
        fn takes_key(self) -> bool {
            matches!(self, Self::Vf6 | Self::Vf13)
        }
    }

    /// Builds the input list for one case.
    fn inputs(slot: Slot, case_idx: usize) -> Vec<Input> {
        let mut out = Vec::new();
        let mut push = |ok: bool, count: u32, arg: u32, plant: u8| {
            let seed = ((case_idx as u64) << 32) | (out.len() as u64);
            out.push(Input {
                ok,
                count,
                arg,
                seed,
                plant,
            });
        };
        if slot.takes_key() {
            for (ci, &count) in slot.counts().iter().enumerate() {
                for (ai, &edge) in EDGE_U32.iter().enumerate().take(6) {
                    push(true, count, edge, ((ci + ai) % 4) as u8);
                }
                let mut rng = Rng((case_idx as u64) ^ (count as u64).wrapping_mul(0x9E37_79B9));
                push(true, count, rng.u32(), (ci % 4) as u8);
            }
            push(false, 7, 0x1234_5678, 0);
            push(false, 0, 0xFFFF_FFFF, 0);
        } else {
            let forced = forced_cells();
            for (ci, &count) in slot.counts().iter().enumerate() {
                for i in 0..10u32 {
                    let arg = match i {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 7,
                        4 => 63,
                        5 => 319,
                        6 => 11,
                        7 => 255,
                        _ => (Rng(ci as u64 ^ (i as u64) << 32).u32() % TABLE_WORDS as u32),
                    };
                    // The classify slots cycle forced answers over the
                    // index; vf12 cycles key/plant shapes; vf7 needs none.
                    let plant = if matches!(slot, Slot::Vf8 | Slot::Vf9) {
                        (i % 9) as u8
                    } else if matches!(slot, Slot::Vf12) {
                        (i % 4) as u8
                    } else {
                        0
                    };
                    push(true, count, arg.min(TABLE_WORDS as u32 - 1), plant);
                }
            }
            push(false, 7, 3, 0);
            push(false, 0, 0, 0);
        }
        out
    }

    /// Plants scan hits/misses for the key slots over `table[0..bound]`.
    fn plant_scan(table: &mut [u32], key: u32, bound: usize, plant: u8) {
        let bound = bound.min(table.len());
        match plant {
            // Absent.
            0 => {
                for w in table.iter_mut().take(bound) {
                    if *w == key {
                        *w = key ^ 0x9E37_79B9;
                    }
                }
            }
            // At 0.
            1 => {
                if bound > 0 {
                    table[0] = key;
                }
            }
            // At the last in-range index.
            2 => {
                if bound > 0 {
                    table[bound - 1] = key;
                }
            }
            // Doubled in the middle.
            _ => {
                if bound > 3 {
                    table[1] = key;
                    table[2] = key;
                } else if bound > 0 {
                    table[0] = key;
                }
            }
        }
    }

    /// Deliberately wrong lifts, one per slot (each must be caught).
    mod wrong {
        use lf_leaderboard::desc::LeaderboardDesc;
        use lf_leaderboard::tables::BoardTables;
        use lf_leaderboard::tables::NOT_FOUND;

        /// vf6: reports the hit one slot too late.
        pub fn find_index(
            store: &mut impl BoardTables,
            desc: &LeaderboardDesc,
            key: u32,
        ) -> u32 {
            let Some(view) = store.fetch(desc.board_id) else {
                return NOT_FOUND;
            };
            let count = view.count as i32;
            if count <= 0 {
                return NOT_FOUND;
            }
            let mut i = 0u32;
            loop {
                if view.primary[i as usize] == key {
                    return i.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                if (i as i32) >= count {
                    return NOT_FOUND;
                }
            }
        }

        /// vf7: reads the next slot.
        pub fn fetch_id(
            store: &mut impl BoardTables,
            desc: &LeaderboardDesc,
            index: u32,
        ) -> u32 {
            let Some(view) = store.fetch(desc.board_id) else {
                return NOT_FOUND;
            };
            view.primary[(index as usize + 1) % view.primary.len()]
        }

        /// vf8: maps class 1 to the wrong tag.
        pub fn class_tag(
            store: &mut impl BoardTables,
            desc: &LeaderboardDesc,
            index: u32,
        ) -> u32 {
            let Some(view) = store.fetch(desc.board_id) else {
                return 0;
            };
            let entry = view.primary[index as usize];
            match store.classify(entry) {
                1 => 8,
                2 | 3 => 8,
                5 => 4,
                _ => 0,
            }
        }

        /// vf9: maps class 2 to the wrong rank.
        pub fn class_rank(
            store: &mut impl BoardTables,
            desc: &LeaderboardDesc,
            index: u32,
        ) -> u32 {
            let Some(view) = store.fetch(desc.board_id) else {
                return NOT_FOUND;
            };
            let entry = view.primary[index as usize];
            match store.classify(entry) {
                1 => 0,
                2 => 0,
                3 => 3,
                4 => NOT_FOUND,
                5 => 2,
                _ => NOT_FOUND,
            }
        }

        /// vf12: starts the second-table scan at 1.
        pub fn reverse_lookup(
            store: &mut impl BoardTables,
            desc: &LeaderboardDesc,
            index: u32,
        ) -> u32 {
            let Some(view) = store.fetch(desc.board_id) else {
                return NOT_FOUND;
            };
            let key = view.primary[index as usize];
            if key == NOT_FOUND {
                return NOT_FOUND;
            }
            let count = view.count;
            if count == 0 {
                return NOT_FOUND;
            }
            let table_b = view.secondary.expect("second table");
            let mut i = 1u32;
            loop {
                if i >= count {
                    return NOT_FOUND;
                }
                if table_b[i as usize] == key {
                    return i;
                }
                i = i.wrapping_add(1);
            }
        }

        /// vf13: returns the second table one slot too late.
        pub fn joined_fetch(
            store: &mut impl BoardTables,
            desc: &LeaderboardDesc,
            key: u32,
        ) -> u32 {
            let Some(view) = store.fetch(desc.board_id) else {
                return NOT_FOUND;
            };
            let count = view.count as i32;
            if count <= 0 {
                return NOT_FOUND;
            }
            let table_b = view.secondary.expect("second table");
            let mut i = 0u32;
            let idx = loop {
                if view.primary[i as usize] == key {
                    break i;
                }
                i = i.wrapping_add(1);
                if (i as i32) >= count {
                    return NOT_FOUND;
                }
            };
            table_b[(idx as usize + 1) % table_b.len()]
        }
    }

    /// The reference call log with unobserved words dropped: the fetch
    /// out-pointer (the rewrite's own stack address) and the dummy second
    /// word of fastcall-declared classify calls.
    fn canonical(calls: &[Call], case: &Case) -> Vec<Call> {
        calls
            .iter()
            .map(|c| {
                if c.slot == case.fetch_slot {
                    assert_eq!(c.args.len(), 2, "fetch takes (board, out-block)");
                    Call {
                        slot: c.slot,
                        args: vec![c.args[0]],
                    }
                } else {
                    assert_eq!(c.slot, case.class_slot, "unexpected slot {}", c.slot);
                    Call {
                        slot: c.slot,
                        args: vec![c.args[0]],
                    }
                }
            })
            .collect()
    }

    /// Runs one slot over all its cases.
    fn run(slot: Slot, cases: &[Case]) {
        let _session = rt::session();
        rt::set_layout(slot.layout());
        rt::set_classify(script);
        let forced = forced_cells();
        let mut image = Image::new();
        for (case_idx, case) in cases.iter().enumerate() {
            rt::install(
                case.fetch_slot,
                if case.class_slot == u32::MAX {
                    u32::MAX
                } else {
                    case.class_slot
                },
                case.class_fastcall == 1,
            );
            let desc = LeaderboardDesc {
                board: case.file,
                board_id: case.board,
                rows: 0,
            };
            let mut distinguished = false;
            for input in inputs(slot, case_idx) {
                image.fill(input.seed);
                let (front, back) = image.words.split_at_mut(512);
                let a = &mut front[0..TABLE_WORDS];
                let b = &mut back[0..TABLE_WORDS];
                match slot {
                    Slot::Vf6 | Slot::Vf13 => {
                        let bound = if (input.count as i32) > 0 {
                            input.count as usize
                        } else {
                            0
                        };
                        plant_scan(a, input.arg, bound, input.plant);
                    }
                    Slot::Vf12 => {
                        // The key sometimes misses on purpose.
                        if input.plant % 4 == 0 {
                            a[input.arg as usize] = 0xFFFF_FFFF;
                        }
                        let key = a[input.arg as usize];
                        plant_scan(b, key, input.count as usize, input.plant);
                    }
                    Slot::Vf8 | Slot::Vf9 => {
                        // Cycle the forced classify answers over the index.
                        if input.plant < 8 {
                            a[input.arg as usize] = forced[input.plant as usize];
                        }
                    }
                    Slot::Vf7 => {}
                }
                rt::set_fetch(FetchScript {
                    ok: input.ok,
                    count: input.count,
                    table_a: image.addr(0),
                    table_b: image.addr(512),
                });
                // `this` varies per input (the slots ignore it; the lift has
                // no such parameter, so any dependence would fail here).
                let this = (input.seed as u32)
                    .wrapping_mul(0x9E37_79B1)
                    .wrapping_add(case.board);
                let (r0, c0) = rt::capture(|| (case.call)(this, input.arg));
                let view = |ok: bool| {
                    ok.then(|| tables::BoardView {
                        count: input.count,
                        primary: &image.words[0..TABLE_WORDS],
                        secondary: Some(&image.words[512..512 + TABLE_WORDS]),
                    })
                };
                let lift = |fake: &mut Fake<'_>| match slot {
                    Slot::Vf6 => tables::find_index(fake, &desc, input.arg),
                    Slot::Vf7 => tables::fetch_id(fake, &desc, input.arg),
                    Slot::Vf8 => tables::class_tag(fake, &desc, input.arg),
                    Slot::Vf9 => tables::class_rank(fake, &desc, input.arg),
                    Slot::Vf12 => tables::reverse_lookup(fake, &desc, input.arg),
                    Slot::Vf13 => tables::joined_fetch(fake, &desc, input.arg),
                };
                let mut fake = Fake {
                    fetch_slot: case.fetch_slot,
                    class_slot: case.class_slot,
                    class_fastcall: case.class_fastcall == 1,
                    view: view(input.ok),
                };
                let (r1, c1) = rt::capture(|| lift(&mut fake));
                assert_eq!(
                    r0, r1,
                    "{} arg={:#x} ok={} count={:#x} plant={}",
                    case.file, input.arg, input.ok, input.count, input.plant
                );
                assert_eq!(
                    canonical(&c0, case),
                    c1,
                    "{} calls differ arg={:#x} ok={} count={:#x}",
                    case.file,
                    input.arg,
                    input.ok,
                    input.count
                );
                // The deliberately wrong lift must be caught on this case.
                let mut wfake = Fake {
                    fetch_slot: case.fetch_slot,
                    class_slot: case.class_slot,
                    class_fastcall: case.class_fastcall == 1,
                    view: view(input.ok),
                };
                let rw = match slot {
                    Slot::Vf6 => wrong::find_index(&mut wfake, &desc, input.arg),
                    Slot::Vf7 => wrong::fetch_id(&mut wfake, &desc, input.arg),
                    Slot::Vf8 => wrong::class_tag(&mut wfake, &desc, input.arg),
                    Slot::Vf9 => wrong::class_rank(&mut wfake, &desc, input.arg),
                    Slot::Vf12 => wrong::reverse_lookup(&mut wfake, &desc, input.arg),
                    Slot::Vf13 => wrong::joined_fetch(&mut wfake, &desc, input.arg),
                };
                distinguished |= rw != r0;
            }
            assert!(
                distinguished,
                "wrong lift never caught for {} (contract blind?)",
                case.file
            );
        }
    }

    /// Runs the probe slot over all its cases.
    ///
    /// The fabricated object lives in the image: `obj[0]` holds the vtable
    /// address, `vt[1]` the key stub. The out word is preset to a sentinel
    /// to prove the no-store paths store nothing.
    fn run_vf2(cases: &[Case2]) {
        use lf_leaderboard::probe::BoardObject;
        use lf_leaderboard::probe::query_tag;
        use lf_leaderboard::Tag;

        const SENTINEL: u32 = 0xA5A5_A5A5;
        const EXPECTEDS: [u32; 4] = [0, 1, 0x1234_5678, 0xFFFF_FFFF];

        let _session = rt::session();
        let mut image = Image::new();
        let obj = image.addr(0);
        let vt = image.addr(8);
        let out_addr = image.addr(16);
        image.words[0] = vt;
        image.words[8] = 0xEEEE_EEEE;
        image.words[9] = rt::key_stub_addr();
        for case in cases {
            // Relocated-tag rewrites run under two mappings; hardcoded
            // ones only under identity (the mapping they were verified
            // under: their store is mapping-fragile by construction).
            let xbases: &[u32] = if case.hardcoded == 1 {
                &[0x0040_0000]
            } else {
                &[0, 0x0040_0000]
            };
            let mut distinguished = false;
            for &xbase in xbases {
                rt::set_xbase(xbase);
                for &expected in &EXPECTEDS {
                    for key in [
                        expected,
                        expected.wrapping_add(1),
                        expected.wrapping_sub(1),
                        0,
                        0xFFFF_FFFF,
                        0xDEAD_BEEF,
                    ] {
                        for &null_out in &[false, true] {
                            image.words[16] = SENTINEL;
                            rt::set_key(key);
                            let out = if null_out { 0 } else { out_addr };
                            let (r0, c0) =
                                rt::capture(|| (case.call)(obj, out, expected));
                            // The key slot runs exactly once, on the object,
                            // on every path (the rewrite calls before it
                            // tests anything).
                            assert_eq!(
                                c0,
                                vec![Call {
                                    slot: rt::VTABLE_SLOT,
                                    args: vec![obj],
                                }],
                                "{} key dispatch",
                                case.file
                            );
                            let board_obj = BoardObject {
                                key,
                                tag: Tag::new(case.tag),
                            };
                            let lift = query_tag(&board_obj, expected);
                            let stored = lift.is_some() && out != 0;
                            assert_eq!(
                                r0,
                                if stored { out } else { 0 },
                                "{} return xbase={xbase:#x} key={key:#x} expected={expected:#x} null_out={null_out}",
                                case.file
                            );
                            assert_eq!(
                                image.words[16],
                                if stored {
                                    rt::relocated(case.tag)
                                } else {
                                    SENTINEL
                                },
                                "{} stored word xbase={xbase:#x} key={key:#x} expected={expected:#x} null_out={null_out}",
                                case.file
                            );
                            // Deliberately wrong lift: inverted match.
                            let wrong = if board_obj.key == expected {
                                None
                            } else {
                                Some(board_obj.tag)
                            };
                            distinguished |= wrong.is_some() != lift.is_some();
                        }
                    }
                }
            }
            assert!(
                distinguished,
                "wrong lift never caught for {} (contract blind?)",
                case.file
            );
        }
    }

    #[test]
    fn slot_vf2() {
        run_vf2(CASES_VF2);
    }

    #[test]
    fn slot_vf6() {
        run(Slot::Vf6, CASES_VF6);
    }

    #[test]
    fn slot_vf7() {
        run(Slot::Vf7, CASES_VF7);
    }

    #[test]
    fn slot_vf8() {
        run(Slot::Vf8, CASES_VF8);
    }

    #[test]
    fn slot_vf9() {
        run(Slot::Vf9, CASES_VF9);
    }

    #[test]
    fn slot_vf12() {
        run(Slot::Vf12, CASES_VF12);
    }

    #[test]
    fn slot_vf13() {
        run(Slot::Vf13, CASES_VF13);
    }
}

#[cfg(not(target_arch = "x86"))]
#[test]
fn host_build_only() {
    // The differential cases need the 32-bit target; the lifted crate's own
    // tests (in lf-leaderboard) run on every host.
}
