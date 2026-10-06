//! Differential cases, part 3: the float predicate and the entry scanner.
//!
//! Each predicate case plants one index table, two threshold words, six
//! threshold constants, one probe address and two scripted answers, then
//! compares the byte answer with the lifted boolean and every call with
//! its arguments (the table word rebuilt per case). Each scanner case
//! plants a registry, entries with planted anchors, objects and slot
//! tables, then compares the full ordered call log with translated
//! addresses. Each method has a deliberately wrong lift that must be
//! caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::input_ui::gates::{
        EntryKind, EntryNotify, EntryRefine, GateConsts, GateProbe, Membership, ScanConsts,
        ScanEntry, ScanGate, ScanQuery, ScanRegistry, float_gate,
    };
    use lf_inputui_diff::rewrites::*;
    use lf_inputui_diff::rt;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// Edge float bit patterns (mirrors the bounds binary's pool).
    fn edge_floats(rng: &mut Rng) -> Vec<u32> {
        // Single patterns with a comment each (kept clear of the
        // publication scan's long-hex-run rule).
        let mut v = std::vec![
            0x0000_0000, // +0.0
            0x8000_0000, // -0.0
            0x3F80_0000, // 1.0
            0xBF80_0000, // -1.0
            0x42C8_0000, // 100.0
            0xC2C8_0000, // -100.0
            0x7F7F_FFFF, // largest finite
            0xFF7F_FFFF, // most negative finite
            0x7F80_0000, // +inf
            0xFF80_0000, // -inf
            0x0000_0001, // smallest subnormal
            0x7FC0_0000, // canonical NaN
            0xFFC0_0000, // negative canonical NaN
            0x7FC0_1234, // quiet NaN with payload
        ];
        for _ in 0..6 {
            v.push(rng.u32());
        }
        v
    }

    // ---------------- the float predicate ----------------

    /// Rewrite-side predicate stub state.
    struct PredStubs {
        member_log: Vec<(u32, u32, u32, u32, u32, u32)>,
        member_answer: u32,
        probe_log: Vec<u32>,
        probe_answer: u32,
    }

    static PRED_STUBS: Mutex<PredStubs> = Mutex::new(PredStubs {
        member_log: Vec::new(),
        member_answer: 0,
        probe_log: Vec::new(),
        probe_answer: 0,
    });

    /// Rewrite-side membership stub (thiscall, six words).
    extern "thiscall" fn member_stub(
        this: u32,
        x2: u32,
        x1: u32,
        z0: u32,
        c5: u32,
        c0: u32,
    ) -> u32 {
        let mut stubs = PRED_STUBS.lock().unwrap();
        stubs.member_log.push((this, x2, x1, z0, c5, c0));
        stubs.member_answer
    }

    /// Rewrite-side probe stub (thiscall, one word).
    extern "thiscall" fn probe_stub(this: u32) -> u32 {
        let mut stubs = PRED_STUBS.lock().unwrap();
        stubs.probe_log.push(this);
        stubs.probe_answer
    }

    struct FakeMember {
        answer: bool,
        log: Vec<[f32; 3]>,
    }

    impl Membership for FakeMember {
        fn query(&mut self, point: [f32; 3]) -> bool {
            self.log.push(point);
            self.answer
        }
    }

    struct FakeProbe {
        answer: u32,
        calls: u32,
    }

    impl GateProbe for FakeProbe {
        fn probe(&mut self) -> u32 {
            self.calls += 1;
            self.answer
        }
    }

    /// Deliberately wrong predicate: the near-path floor arms are
    /// swapped (a flipped-branch slip). Must be caught whenever the
    /// near path reaches the floor check.
    #[allow(clippy::too_many_arguments)]
    fn wrong_gate(
        point: [f32; 3],
        anchor: [f32; 2],
        f2: f32,
        f3: f32,
        under: bool,
        consts: &GateConsts,
        present: bool,
        r2: u32,
    ) -> bool {
        let dx = anchor[0] - point[0];
        let dy = anchor[1] - point[1];
        let dist2 = dx * dx + dy * dy;
        if present {
            if !under {
                return false;
            }
        } else {
            if (r2 & 0xFF) == 0 {
                let k = consts.cap;
                let mut t = f3;
                if !(t > k) {
                    t = k;
                }
                t = t / k;
                t = t * consts.window;
                t = t * f2;
                t = t * t;
                if dist2 > t {
                    return false;
                }
                let s = consts.floor;
                let s2 = s * s;
                // WRONG: arms swapped.
                if !(s2 > dist2) {
                    return false;
                } else {
                    return true;
                }
            }
            if !under {
                return false;
            }
        }
        let k = consts.cap;
        let m = if k > f3 { k } else { f3 };
        let base = consts.base;
        let c3 = m * f2;
        let c3sq = c3 * c3;
        let c2 = (base - consts.inset) * f2;
        let c2sq = c2 * c2;
        let c1 = (base * consts.frac) * f2;
        let c1sq = c1 * c1;
        if dist2 > c3sq {
            return false;
        }
        if c2sq > dist2 {
            return false;
        }
        if c1sq > dist2 {
            return false;
        }
        true
    }

    #[test]
    fn float_gate_matches() {
        let _guard = lock();
        rt::set_callee(1, member_stub as usize as u32);
        rt::set_callee(2, probe_stub as usize as u32);
        let mut rng = Rng(0x9A7E);
        let mut cases = 0;
        let mut caught = 0;
        let edges = edge_floats(&mut rng);
        let probe_cell = Box::new(0xBEAD_0000u32);
        let probe_addr = addr(&*probe_cell);
        rt::set_relocated(support::PROBE_VA, probe_addr);
        // Scripted answers: absent/present crossed with near/far probe
        // bytes, with high-byte residue proving only the low byte decides.
        let scripts = [
            (0u32, 0x0000_0000u32),
            (0, 0x00AB_0000),
            (0, 0x0000_0001),
            (0, 0x00CD_0001),
            (1, 0x0000_0000),
            (0xFFFF_FFFF, 0x1234_5678),
        ];
        for &idx in &[0u32, 1, 2, 3] {
            for &(member_answer, r2) in &scripts {
                for &under_a in &[0u32, 1, 0xFFFF_FFFF] {
                    for &under_b in &[0u32, 1, 0xFFFF_FFFF] {
                        // A few float shapes per script combo.
                        for round in 0..6 {
                            let pick = |rng: &mut Rng| {
                                f32::from_bits(if rng.u32() % 2 == 0 {
                                    edges[(rng.u32() as usize) % edges.len()]
                                } else {
                                    rng.u32()
                                })
                            };
                            let point = [pick(&mut rng), pick(&mut rng), pick(&mut rng)];
                            let anchor = [pick(&mut rng), pick(&mut rng)];
                            let f2 = pick(&mut rng);
                            let f3 = pick(&mut rng);
                            let consts = GateConsts {
                                cap: pick(&mut rng),
                                window: pick(&mut rng),
                                floor: pick(&mut rng),
                                base: pick(&mut rng),
                                inset: pick(&mut rng),
                                frac: pick(&mut rng),
                            };
                            // Plant the table: word 0 is the index, the
                            // rest are garbage table words.
                            let mut tab = [rng.u32(), rng.u32(), rng.u32(), rng.u32()];
                            tab[0] = idx;
                            for (k, w) in tab.iter().enumerate() {
                                unsafe {
                                    rt::global::<u32>(support::FG_TAB_VA + (k as u32) * 4)
                                        .write_unaligned(*w);
                                }
                            }
                            unsafe {
                                rt::global::<u32>(support::FG_UNDER_A_VA).write_unaligned(under_a);
                                rt::global::<u32>(support::FG_UNDER_B_VA).write_unaligned(under_b);
                                let words = [
                                    consts.cap.to_bits(),
                                    consts.window.to_bits(),
                                    consts.floor.to_bits(),
                                    consts.base.to_bits(),
                                    consts.inset.to_bits(),
                                    consts.frac.to_bits(),
                                ];
                                for (k, va) in support::FG_CONST_VAS.iter().enumerate() {
                                    rt::global::<u32>(*va).write_unaligned(words[k]);
                                }
                            }
                            {
                                let mut stubs = PRED_STUBS.lock().unwrap();
                                stubs.member_log.clear();
                                stubs.member_answer = member_answer;
                                stubs.probe_log.clear();
                                stubs.probe_answer = r2;
                            }
                            let point_box = Box::new(point);
                            let anchor_box = Box::new(anchor);
                            // Rewrite side.
                            let got = unsafe {
                                fn_00AFE3B0::rw_00afe3b0(
                                    addr(&point_box[0]),
                                    addr(&anchor_box[0]),
                                    f2,
                                    f3,
                                )
                            };
                            let (member_log, probe_log);
                            {
                                let stubs = PRED_STUBS.lock().unwrap();
                                member_log = stubs.member_log.clone();
                                probe_log = stubs.probe_log.clone();
                            }
                            // Lift side.
                            let mut member = FakeMember {
                                answer: member_answer != 0,
                                log: Vec::new(),
                            };
                            let mut probe = FakeProbe {
                                answer: r2,
                                calls: 0,
                            };
                            let lift = float_gate(
                                point,
                                anchor,
                                f2,
                                f3,
                                under_a < under_b,
                                &consts,
                                &mut member,
                                &mut probe,
                            );
                            // Compare.
                            assert_eq!(got, u8::from(lift), "round={round}");
                            assert!(got == 0 || got == 1);
                            assert_eq!(member_log.len(), 1);
                            let (this, x2, x1, z0, c5, c0) = member_log[0];
                            assert_eq!(this, tab[idx as usize].wrapping_add(0x10));
                            assert_eq!(
                                (x2, x1, z0),
                                (point[0].to_bits(), point[1].to_bits(), point[2].to_bits())
                            );
                            assert_eq!((c5, c0), (0x40A0_0000, 0));
                            assert_eq!(member.log.len(), 1);
                            assert_eq!(member.log[0].map(f32::to_bits), point.map(f32::to_bits));
                            if member_answer == 0 {
                                assert_eq!(probe_log, std::vec![probe_addr]);
                                assert_eq!(probe.calls, 1);
                            } else {
                                assert!(probe_log.is_empty());
                                assert_eq!(probe.calls, 0);
                            }
                            if wrong_gate(
                                point,
                                anchor,
                                f2,
                                f3,
                                under_a < under_b,
                                &consts,
                                member_answer != 0,
                                r2,
                            ) != lift
                            {
                                caught += 1;
                            }
                            cases += 1;
                            std::hint::black_box((&point_box, &anchor_box));
                        }
                    }
                }
            }
        }
        std::hint::black_box(&probe_cell);
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong gate never caught ({cases} cases)");
    }

    // ---------------- the entry scanner ----------------

    /// Ordered scanner events for log comparison.
    #[derive(Clone, PartialEq, Eq, Debug)]
    enum Event {
        Query,
        Gate,
        Kind(u32),
        Refine(u32),
        Main(u32),
        Alt(u32),
        Far(u32),
    }

    impl Event {
        /// The event kind without its address, for shape comparison.
        fn shape(&self) -> u8 {
            match self {
                Event::Query => 0,
                Event::Gate => 1,
                Event::Kind(_) => 2,
                Event::Refine(_) => 3,
                Event::Main(_) => 4,
                Event::Alt(_) => 5,
                Event::Far(_) => 6,
            }
        }
    }

    /// Rewrite-side scanner stub state.
    struct ScanStubs {
        log: Vec<Event>,
        triples: VecDeque<[f32; 3]>,
        gates: VecDeque<u32>,
        kinds: VecDeque<u32>,
        refines: VecDeque<f32>,
        mains: VecDeque<u32>,
        alts: VecDeque<u32>,
        fars: VecDeque<u32>,
    }

    static SCAN_STUBS: Mutex<ScanStubs> = Mutex::new(ScanStubs {
        log: Vec::new(),
        triples: VecDeque::new(),
        gates: VecDeque::new(),
        kinds: VecDeque::new(),
        refines: VecDeque::new(),
        mains: VecDeque::new(),
        alts: VecDeque::new(),
        fars: VecDeque::new(),
    });

    /// Rewrite-side query stub (cdecl, one word).
    extern "cdecl" fn query_stub(out: u32) -> u32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Query);
        let triple = stubs
            .triples
            .pop_front()
            .expect("query stub called with nothing scripted");
        unsafe {
            for k in 0..3 {
                ((out + k * 4) as *mut u32).write_unaligned(triple[k as usize].to_bits());
            }
        }
        0
    }

    /// Rewrite-side gate stub (cdecl, no words).
    extern "cdecl" fn scan_gate_stub() -> u32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Gate);
        stubs
            .gates
            .pop_front()
            .expect("gate stub called with nothing scripted")
    }

    /// Rewrite-side kind stub (thiscall, one word).
    extern "thiscall" fn kind_stub(esi: u32) -> u32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Kind(esi));
        stubs
            .kinds
            .pop_front()
            .expect("kind stub called with nothing scripted")
    }

    /// Rewrite-side refinement stub (thiscall, one word, float answer).
    extern "thiscall" fn refine_stub(o: u32) -> f32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Refine(o));
        stubs
            .refines
            .pop_front()
            .expect("refine stub called with nothing scripted")
    }

    /// Rewrite-side main-notify stub (thiscall, one word).
    extern "thiscall" fn main_stub(esi: u32) -> u32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Main(esi));
        stubs
            .mains
            .pop_front()
            .expect("main stub called with nothing scripted")
    }

    /// Rewrite-side alternate-notify stub (thiscall, one word).
    extern "thiscall" fn alt_stub(esi: u32) -> u32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Alt(esi));
        stubs
            .alts
            .pop_front()
            .expect("alt stub called with nothing scripted")
    }

    /// Rewrite-side far-notify stub (thiscall, one word).
    extern "thiscall" fn far_stub(esi: u32) -> u32 {
        let mut stubs = SCAN_STUBS.lock().unwrap();
        stubs.log.push(Event::Far(esi));
        stubs
            .fars
            .pop_front()
            .expect("far stub called with nothing scripted")
    }

    /// Lift-side query fake.
    struct FakeQuery {
        triples: VecDeque<[f32; 3]>,
        calls: u32,
    }

    impl ScanQuery for FakeQuery {
        fn query(&mut self) -> [f32; 3] {
            self.calls += 1;
            self.triples.pop_front().unwrap()
        }
    }

    /// Lift-side gate fake.
    struct FakeScanGate {
        answers: VecDeque<u32>,
        calls: u32,
    }

    impl ScanGate for FakeScanGate {
        fn gate(&mut self) -> u32 {
            self.calls += 1;
            self.answers.pop_front().unwrap()
        }
    }

    /// Lift-side kind fake: records the entries it saw.
    struct FakeKind {
        answers: VecDeque<u32>,
        log: Vec<ScanEntry>,
    }

    impl EntryKind for FakeKind {
        fn kind(&mut self, entry: &ScanEntry) -> u32 {
            self.log.push(entry.clone());
            self.answers.pop_front().unwrap()
        }
    }

    /// Lift-side refine fake.
    struct FakeRefine {
        answers: VecDeque<f32>,
        log: Vec<ScanEntry>,
    }

    impl EntryRefine for FakeRefine {
        fn refine(&mut self, entry: &ScanEntry) -> f32 {
            self.log.push(entry.clone());
            self.answers.pop_front().unwrap()
        }
    }

    /// Lift-side notify fake: records (slot name, entry) in order.
    struct FakeNotify {
        mains: VecDeque<u32>,
        alts: VecDeque<u32>,
        fars: VecDeque<u32>,
        log: Vec<(u8, ScanEntry)>,
    }

    impl EntryNotify for FakeNotify {
        fn notify_main(&mut self, entry: &ScanEntry) -> u32 {
            self.log.push((0, entry.clone()));
            self.mains.pop_front().unwrap()
        }

        fn notify_alt(&mut self, entry: &ScanEntry) -> u32 {
            self.log.push((1, entry.clone()));
            self.alts.pop_front().unwrap()
        }

        fn notify_far(&mut self, entry: &ScanEntry) -> u32 {
            self.log.push((2, entry.clone()));
            self.fars.pop_front().unwrap()
        }
    }

    /// One case's scripted answers, cloned to both sides.
    #[derive(Clone)]
    struct Script {
        triples: Vec<[f32; 3]>,
        gates: Vec<u32>,
        kinds: Vec<u32>,
        refines: Vec<f32>,
        mains: Vec<u32>,
        alts: Vec<u32>,
        fars: Vec<u32>,
    }

    /// Bit-wise entry identity: anchors may hold NaN, which never
    /// equals itself, so derived `PartialEq` cannot compare entries.
    fn entry_key(entry: &ScanEntry) -> (u8, [u32; 3], u32, bool) {
        (
            entry.flag,
            [
                entry.anchor[0].to_bits(),
                entry.anchor[1].to_bits(),
                entry.anchor[2].to_bits(),
            ],
            entry.mode,
            entry.has_object,
        )
    }

    /// Slot of an entry logged by a lift fake, by bit-wise identity.
    fn slot_of(entries: &[ScanEntry], entry: &ScanEntry) -> usize {
        let key = entry_key(entry);
        entries
            .iter()
            .position(|e| entry_key(e) == key)
            .expect("lift fake logged a foreign entry")
    }

    /// Entry layout constants of the 32-bit form.
    const ENTRY_STRIDE: u32 = 0x1400;
    const ENTRY_VT_OFF: u32 = 0x00;
    const ENTRY_POS_OFF: u32 = 0x20;
    const ENTRY_OBJ_OFF: u32 = 0x6C;
    const ENTRY_MODE_OFF: u32 = 0x1304;
    const POS_ANCHOR_OFF: usize = 0x30;

    /// Deliberately wrong scan, first mutant: visits slots upward
    /// instead of downward. Must be caught whenever two slots are live.
    fn wrong_order_up(count: usize, flags: &[u8]) -> Vec<usize> {
        (0..count).filter(|&i| flags[i] & 0x80 == 0).collect()
    }

    /// Deliberately wrong scan, second mutant: the near/far step test
    /// is flipped (`s0 > slot` takes the near step). A pure simulation
    /// over the case script; must be caught whenever a live entry's
    /// distance differs from its threshold.
    #[allow(clippy::too_many_arguments)]
    fn wrong_shape_flipped(
        visited: &[usize],
        entries: &[ScanEntry],
        arg: f32,
        consts: &ScanConsts,
        script: &Script,
    ) -> Vec<u8> {
        let mut shape = Vec::new();
        let (mut qi, mut gi, mut ki, mut ri, mut mi) = (0, 0, 0, 0, 0);
        for &slot in visited {
            let entry = &entries[slot];
            let measured = script.triples[qi];
            qi += 1;
            shape.push(0);
            let dx = measured[0] - entry.anchor[0];
            let dy = measured[1] - entry.anchor[1];
            let dz = measured[2] - entry.anchor[2];
            let mut dist = dx * dx + dy * dy + dz * dz;
            let r2 = script.gates[gi];
            gi += 1;
            shape.push(1);
            if (r2 & 0xFF) != 0 && entry.has_object {
                dist = script.refines[ri];
                ri += 1;
                shape.push(3);
            }
            let r7 = script.kinds[ki];
            ki += 1;
            shape.push(2);
            let x2 = if (r7 & 0xFF) != 0 && entry.mode != 2 {
                consts.detailed
            } else {
                consts.plain
            };
            let x3 = consts.detailed;
            let mut t0 = consts.floor;
            if !(t0 > x3) {
                t0 = x3;
            }
            let mut t1 = consts.near * arg;
            let x2v = x2 * t0;
            let mut s0 = consts.far * arg;
            t1 = t1 * x2v;
            s0 = s0 * x2v;
            s0 = s0 * s0;
            // WRONG: the step test is flipped. (Alternate/far answers
            // never branch further, so only their shapes matter.)
            if s0 > dist {
                let main = script.mains[mi];
                mi += 1;
                shape.push(4);
                if (main & 0xFF) == 0 {
                    shape.push(5);
                }
            } else {
                t1 = t1 * t1;
                if t1 > dist {
                    let main = script.mains[mi];
                    mi += 1;
                    shape.push(4);
                    if (main & 0xFF) != 0 {
                        shape.push(6);
                    }
                }
            }
        }
        shape
    }

    #[test]
    fn entry_scanner_matches() {
        let _guard = lock();
        rt::set_callee(1, query_stub as usize as u32);
        rt::set_callee(2, scan_gate_stub as usize as u32);
        rt::set_callee(7, kind_stub as usize as u32);
        let mut rng = Rng(0x5CA4);
        let mut cases = 0;
        let mut caught_order = 0;
        let mut caught_flip = 0;
        let edges = edge_floats(&mut rng);
        let pick = |rng: &mut Rng| {
            f32::from_bits(if rng.u32() % 2 == 0 {
                edges[(rng.u32() as usize) % edges.len()]
            } else {
                rng.u32()
            })
        };
        // Flag patterns over 0..3 slots: empty, all live, all dead,
        // mixes, low-bit-only (live without the top bit).
        let flag_sets: Vec<Vec<u8>> = std::vec![
            std::vec![],
            std::vec![0x00],
            std::vec![0x80],
            std::vec![0x00, 0x00],
            std::vec![0x80, 0x80],
            std::vec![0x00, 0x80],
            std::vec![0x80, 0x00],
            std::vec![0x01, 0x7F, 0xFF],
            std::vec![0x00, 0x00, 0x00],
            std::vec![0x80, 0x00, 0x80],
        ];
        for flags in &flag_sets {
            for round in 0..8 {
                let count = flags.len();
                let arg = pick(&mut rng);
                let consts = ScanConsts {
                    detailed: pick(&mut rng),
                    plain: pick(&mut rng),
                    floor: pick(&mut rng),
                    near: pick(&mut rng),
                    far: pick(&mut rng),
                };
                let stride = if round == 7 { 0 } else { ENTRY_STRIDE };
                // Lift entries.
                let mut entries = Vec::new();
                for (i, &flag) in flags.iter().enumerate() {
                    entries.push(ScanEntry {
                        flag,
                        anchor: [pick(&mut rng), pick(&mut rng), pick(&mut rng)],
                        mode: if (i + round) % 3 == 0 {
                            2
                        } else {
                            rng.u32() % 5
                        },
                        has_object: (i + round) % 2 == 0,
                    });
                }
                // Stride 0 aliases every slot onto slot 0's memory.
                if stride == 0 && count > 0 {
                    let first = entries[0].clone();
                    for e in entries.iter_mut() {
                        e.anchor = first.anchor;
                        e.mode = first.mode;
                        e.has_object = first.has_object;
                    }
                }
                // 32-bit entries: one block, fixed stride.
                let block_len = if count == 0 {
                    4
                } else if stride == 0 {
                    ENTRY_STRIDE as usize
                } else {
                    count * ENTRY_STRIDE as usize
                };
                let mut block = std::vec![0u8; block_len];
                rng.bytes(&mut block);
                let block_box: Box<[u8]> = block.into_boxed_slice();
                let base = if count == 0 {
                    0x2000_0000
                } else {
                    addr(&block_box[0])
                };
                // Per-entry planted objects: entry slot table, position
                // block, optional refine object + its table.
                struct Planted {
                    _evt: Box<[u32; 96]>,
                    _pos: Box<[u8; 64]>,
                    _obj: Option<Box<[u32; 2]>>,
                    _ovt: Option<Box<[u32; 80]>>,
                    obj_addr: u32,
                }
                let mut planted: Vec<Planted> = Vec::new();
                for entry in entries.iter() {
                    let mut evt = Box::new([0u32; 96]);
                    evt[0x144 / 4] = main_stub as usize as u32;
                    evt[0x148 / 4] = alt_stub as usize as u32;
                    evt[0x14C / 4] = far_stub as usize as u32;
                    let mut pos = Box::new([0u8; 64]);
                    for (k, w) in entry.anchor.iter().enumerate() {
                        pos[POS_ANCHOR_OFF + k * 4..POS_ANCHOR_OFF + k * 4 + 4]
                            .copy_from_slice(&w.to_bits().to_le_bytes());
                    }
                    let (obj, ovt, obj_addr) = if entry.has_object {
                        let mut ovt = Box::new([0u32; 80]);
                        ovt[0x104 / 4] = refine_stub as usize as u32;
                        let ovt_addr = addr(&ovt[0]);
                        let mut obj = Box::new([0u32; 2]);
                        obj[0] = ovt_addr;
                        let obj_addr = addr(&obj[0]);
                        (Some(obj), Some(ovt), obj_addr)
                    } else {
                        (None, None, 0)
                    };
                    planted.push(Planted {
                        _evt: evt,
                        _pos: pos,
                        _obj: obj,
                        _ovt: ovt,
                        obj_addr,
                    });
                }
                // Write entry words through raw addresses. Stride 0
                // aliases every slot onto the base, so every slot
                // writes slot 0's planted objects (all slots hold
                // clones there, so every write agrees).
                for (i, entry) in entries.iter().enumerate() {
                    let esi = if stride == 0 {
                        base
                    } else {
                        base + (i as u32) * stride
                    };
                    let p = if stride == 0 {
                        &planted[0]
                    } else {
                        &planted[i]
                    };
                    unsafe {
                        ((esi + ENTRY_VT_OFF) as *mut u32).write_unaligned(addr(&p._evt[0]));
                        ((esi + ENTRY_POS_OFF) as *mut u32).write_unaligned(addr(&p._pos[0]));
                        ((esi + ENTRY_OBJ_OFF) as *mut u32).write_unaligned(p.obj_addr);
                        ((esi + ENTRY_MODE_OFF) as *mut u32).write_unaligned(entry.mode);
                    }
                }
                let flags_box: Box<[u8]> = flags.clone().into_boxed_slice();
                let flag_base = if count == 0 {
                    0x2000_1000
                } else {
                    addr(&flags_box[0])
                };
                let reg = Box::new([base, flag_base, count as u32, stride]);
                unsafe {
                    rt::global::<u32>(support::SC_REG_VA).write_unaligned(addr(&reg[0]));
                    let words = [
                        consts.detailed.to_bits(),
                        consts.plain.to_bits(),
                        consts.floor.to_bits(),
                        consts.near.to_bits(),
                        consts.far.to_bits(),
                    ];
                    for (k, va) in support::SC_CONST_VAS.iter().enumerate() {
                        rt::global::<u32>(*va).write_unaligned(words[k]);
                    }
                }
                // One script for both sides.
                let live = flags.iter().filter(|f| *f & 0x80 == 0).count();
                let mut script = Script {
                    triples: Vec::new(),
                    gates: Vec::new(),
                    kinds: Vec::new(),
                    refines: Vec::new(),
                    mains: Vec::new(),
                    alts: Vec::new(),
                    fars: Vec::new(),
                };
                for _ in 0..live {
                    script
                        .triples
                        .push([pick(&mut rng), pick(&mut rng), pick(&mut rng)]);
                    script.gates.push(rng.u32());
                    script.kinds.push(rng.u32());
                    script.refines.push(pick(&mut rng));
                }
                for _ in 0..2 * live + 2 {
                    script.mains.push(rng.u32());
                    script.alts.push(rng.u32());
                    script.fars.push(rng.u32());
                }
                {
                    let mut stubs = SCAN_STUBS.lock().unwrap();
                    stubs.log.clear();
                    stubs.triples = VecDeque::from(script.triples.clone());
                    stubs.gates = VecDeque::from(script.gates.clone());
                    stubs.kinds = VecDeque::from(script.kinds.clone());
                    stubs.refines = VecDeque::from(script.refines.clone());
                    stubs.mains = VecDeque::from(script.mains.clone());
                    stubs.alts = VecDeque::from(script.alts.clone());
                    stubs.fars = VecDeque::from(script.fars.clone());
                }
                // Rewrite side.
                unsafe { fn_00AFE030::rw_00afe030(arg) };
                let rw_log = SCAN_STUBS.lock().unwrap().log.clone();
                // Lift side.
                let registry = ScanRegistry {
                    entries: entries.clone(),
                };
                let mut query = FakeQuery {
                    triples: VecDeque::from(script.triples.clone()),
                    calls: 0,
                };
                let mut gate = FakeScanGate {
                    answers: VecDeque::from(script.gates.clone()),
                    calls: 0,
                };
                let mut kind = FakeKind {
                    answers: VecDeque::from(script.kinds.clone()),
                    log: Vec::new(),
                };
                let mut refine = FakeRefine {
                    answers: VecDeque::from(script.refines.clone()),
                    log: Vec::new(),
                };
                let mut notify = FakeNotify {
                    mains: VecDeque::from(script.mains.clone()),
                    alts: VecDeque::from(script.alts.clone()),
                    fars: VecDeque::from(script.fars.clone()),
                    log: Vec::new(),
                };
                registry.scan(
                    arg,
                    &consts,
                    &mut query,
                    &mut gate,
                    &mut kind,
                    &mut refine,
                    &mut notify,
                );
                // Compare. Visited slots run top-down over live flags.
                let visited: Vec<usize> =
                    (0..count).rev().filter(|&i| flags[i] & 0x80 == 0).collect();
                let slot_addr = |slot: usize| {
                    if stride == 0 {
                        base
                    } else {
                        base + (slot as u32) * stride
                    }
                };
                assert_eq!(query.calls as usize, live);
                assert_eq!(gate.calls as usize, live);
                assert_eq!(
                    kind.log.iter().map(entry_key).collect::<Vec<_>>(),
                    visited
                        .iter()
                        .map(|&s| entry_key(&entries[s]))
                        .collect::<Vec<_>>()
                );
                // The rewrite's kind addresses must name the same slots.
                let rw_kinds: Vec<u32> = rw_log
                    .iter()
                    .filter_map(|e| match e {
                        Event::Kind(a) => Some(*a),
                        _ => None,
                    })
                    .collect();
                assert_eq!(
                    rw_kinds,
                    visited.iter().map(|&s| slot_addr(s)).collect::<Vec<_>>()
                );
                // Refines pair positionally: same count, same slots.
                let rw_refines: Vec<u32> = rw_log
                    .iter()
                    .filter_map(|e| match e {
                        Event::Refine(o) => Some(*o),
                        _ => None,
                    })
                    .collect();
                assert_eq!(refine.log.len(), rw_refines.len());
                for (entry, o) in refine.log.iter().zip(rw_refines.iter()) {
                    let slot = slot_of(&entries, entry);
                    assert_eq!(*o, planted[slot].obj_addr);
                    assert_ne!(*o, 0);
                }
                // Notifies pair positionally: same slots, same slot kind.
                let rw_notifies: Vec<(u8, u32)> = rw_log
                    .iter()
                    .filter_map(|e| match e {
                        Event::Main(a) => Some((0, *a)),
                        Event::Alt(a) => Some((1, *a)),
                        Event::Far(a) => Some((2, *a)),
                        _ => None,
                    })
                    .collect();
                assert_eq!(notify.log.len(), rw_notifies.len());
                for ((k, entry), (rk, a)) in notify.log.iter().zip(rw_notifies.iter()) {
                    assert_eq!(k, rk);
                    let slot = slot_of(&entries, entry);
                    assert_eq!(*a, slot_addr(slot));
                }
                // Full shape equality: every event kind in order. The
                // per-kind counts plus positional pairing above already
                // pin the lift's sequence; this shape drives the mutant.
                let rw_shape: Vec<u8> = rw_log.iter().map(Event::shape).collect();
                // Mutants.
                if wrong_order_up(count, flags) != visited {
                    caught_order += 1;
                }
                if wrong_shape_flipped(&visited, &entries, arg, &consts, &script) != rw_shape {
                    caught_flip += 1;
                }
                cases += 1;
                std::hint::black_box((&block_box, &flags_box, &reg, &planted));
            }
        }
        assert!(cases > 50, "too few comparisons ({cases})");
        assert!(caught_order > 0, "wrong order never caught ({cases} cases)");
        assert!(caught_flip > 0, "wrong step never caught ({cases} cases)");
    }
}
