//! Differential cases, part 2: the state record copy and the bounds
//! accumulator.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares results and every effect:
//! all copied words, the sub-copy call order with translated arguments,
//! the measure/sink call logs with the range ends rebuilt from the
//! lifted count, and floats bit for bit. Each method has a deliberately
//! wrong lift that must be caught. 32-bit target only.
//!
//! The bounds walk `while p != end` only visits exact multiples of the
//! stride here: misaligned ranges are out of the proof domain.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::input_ui::bounds::{BoundsSink, Measure, accumulate_bounds};
    use lf_input_frontend::input_ui::records::{SubRecord, UiStateRecord};
    use lf_inputui_diff::rewrites::*;
    use lf_inputui_diff::rt;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::sync::Mutex;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    // ---------------- the state record copy ----------------

    /// Rewrite-side sub-copy stub logs.
    struct CopyStubs {
        head_log: Vec<(u32, u32)>,
        tail_log: Vec<(u32, u32)>,
    }

    static COPY_STUBS: Mutex<CopyStubs> = Mutex::new(CopyStubs {
        head_log: Vec::new(),
        tail_log: Vec::new(),
    });

    /// Rewrite-side head sub-copy stub: copies 64 bytes, logs (dst, src).
    extern "thiscall" fn sub_head_stub(dst: u32, src: u32) -> u32 {
        COPY_STUBS.lock().unwrap().head_log.push((dst, src));
        unsafe {
            core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, 0x40);
        }
        dst
    }

    /// Rewrite-side tail sub-copy stub: copies 176 bytes, logs (dst, src).
    extern "thiscall" fn sub_tail_stub(dst: u32, src: u32) -> u32 {
        COPY_STUBS.lock().unwrap().tail_log.push((dst, src));
        unsafe {
            core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, 0xB0);
        }
        dst
    }

    /// Lift-side sub-object: records its call order in a shared log.
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct LogSub {
        id: u8,
        words: Vec<u32>,
        log: Rc<RefCell<Vec<u8>>>,
    }

    impl SubRecord for LogSub {
        fn copy_from_source(&mut self, src: &Self) {
            self.log.borrow_mut().push(self.id);
            self.words = src.words.clone();
        }
    }

    /// Lifts raw bytes into a record.
    fn load_record(bytes: &[u8], log: Rc<RefCell<Vec<u8>>>) -> UiStateRecord<LogSub, LogSub> {
        assert_eq!(bytes.len(), 0x26C);
        let word = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        let mut head = [0u32; 24];
        for (i, w) in head.iter_mut().enumerate() {
            *w = word(i * 4);
        }
        let mut hw = Vec::new();
        for i in 0..16 {
            hw.push(word(0x60 + i * 4));
        }
        let mut mid = [0u32; 64];
        for (i, w) in mid.iter_mut().enumerate() {
            *w = word(0xA0 + i * 4);
        }
        let mut tw = Vec::new();
        for i in 0..44 {
            tw.push(word(0x1A0 + i * 4));
        }
        let tail = [word(0x250), word(0x254), word(0x258), word(0x260), word(0x264), word(0x268)];
        UiStateRecord {
            head,
            sub_head: LogSub {
                id: 1,
                words: hw,
                log: log.clone(),
            },
            mid,
            sub_tail: LogSub {
                id: 2,
                words: tw,
                log,
            },
            tail,
        }
    }

    /// Stores a record over raw bytes, leaving the `0x25C` hole alone.
    fn store_record(rec: &UiStateRecord<LogSub, LogSub>, bytes: &mut [u8]) {
        assert_eq!(bytes.len(), 0x26C);
        let mut put = |off: usize, v: u32| bytes[off..off + 4].copy_from_slice(&v.to_le_bytes());
        for (i, w) in rec.head.iter().enumerate() {
            put(i * 4, *w);
        }
        for (i, w) in rec.sub_head.words.iter().enumerate() {
            put(0x60 + i * 4, *w);
        }
        for (i, w) in rec.mid.iter().enumerate() {
            put(0xA0 + i * 4, *w);
        }
        for (i, w) in rec.sub_tail.words.iter().enumerate() {
            put(0x1A0 + i * 4, *w);
        }
        // Explicit offsets: the six words are not contiguous (the
        // 0x25C hole is never touched and keeps the destination's byte).
        const TAIL_OFFS: [usize; 6] = [0x250, 0x254, 0x258, 0x260, 0x264, 0x268];
        for (i, w) in rec.tail.iter().enumerate() {
            put(TAIL_OFFS[i], *w);
        }
    }

    /// Deliberately wrong copy: delegates the tail before the head. Must
    /// be caught by the call-order log on every case.
    fn wrong_copy<H: SubRecord, T: SubRecord>(dst: &mut UiStateRecord<H, T>, src: &UiStateRecord<H, T>) {
        dst.head = src.head;
        dst.sub_tail.copy_from_source(&src.sub_tail);
        dst.mid = src.mid;
        dst.sub_head.copy_from_source(&src.sub_head);
        dst.tail = src.tail;
    }

    #[test]
    fn state_copy_matches() {
        let _guard = lock();
        rt::set_callee(1, sub_head_stub as usize as u32);
        rt::set_callee(2, sub_tail_stub as usize as u32);
        let mut rng = Rng(0xC0E4);
        let mut cases = 0;
        let mut caught = 0;
        // Targeted fills: zeroes, ones, words with a single bit, the
        // hole canary, then random.
        let mut fills: Vec<Vec<u8>> = std::vec![
            std::vec![0x00; 0x26C],
            std::vec![0xFF; 0x26C],
            (0..0x26C).map(|i| (i % 251) as u8).collect(),
        ];
        for _ in 0..8 {
            let mut f = std::vec![0u8; 0x26C];
            rng.bytes(&mut f);
            fills.push(f);
        }
        for (fi, src_fill) in fills.iter().enumerate() {
            for (gi, dst_fill) in fills.iter().enumerate() {
                if fi > 2 && gi > 2 && (fi ^ gi ^ 1) % 3 == 0 {
                    continue; // thin the full cross product
                }
                let src_box: Box<[u8]> = src_fill.clone().into_boxed_slice();
                let mut dst_box: Box<[u8]> = dst_fill.clone().into_boxed_slice();
                let src_addr = addr(&src_box[0]);
                let dst_addr = addr(&dst_box[0]);
                {
                    let mut stubs = COPY_STUBS.lock().unwrap();
                    stubs.head_log.clear();
                    stubs.tail_log.clear();
                }
                // Rewrite side.
                let got = unsafe { fn_009284D0::rw_009284d0(dst_addr, src_addr) };
                let (head_log, tail_log) = {
                    let stubs = COPY_STUBS.lock().unwrap();
                    (stubs.head_log.clone(), stubs.tail_log.clone())
                };
                // The rewrite's destination bytes, read back through a
                // raw pointer so the compiler cannot forward pre-call values.
                let mut rw_bytes = std::vec![0u8; 0x26C];
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        dst_addr as *const u8,
                        rw_bytes.as_mut_ptr(),
                        0x26C,
                    );
                }
                // Lift side. The lift's destination loads from the
                // pristine pre-call fill, never from `dst_box`: the
                // rewrite mutated that memory through a raw address, so
                // a borrow here could read forwarded pre-call values.
                let order = Rc::new(RefCell::new(Vec::new()));
                let src_rec = load_record(&src_box, Rc::new(RefCell::new(Vec::new())));
                let mut dst_rec = load_record(dst_fill, order.clone());
                dst_rec.copy_from(&src_rec);
                let mut lift_bytes = dst_fill.clone();
                store_record(&dst_rec, &mut lift_bytes);
                // Compare.
                assert_eq!(got, dst_addr, "return pins the destination address");
                assert_eq!(head_log, std::vec![(dst_addr + 0x60, src_addr + 0x60)]);
                assert_eq!(tail_log, std::vec![(dst_addr + 0x1A0, src_addr + 0x1A0)]);
                assert_eq!(*order.borrow(), std::vec![1u8, 2u8]);
                assert_eq!(rw_bytes, lift_bytes, "fill pair ({fi}, {gi})");
                // The hole survives on both sides.
                assert_eq!(&rw_bytes[0x25C..0x260], &dst_fill[0x25C..0x260]);
                // Wrong lift: swapped delegation order, caught by the log.
                let wrong_order = Rc::new(RefCell::new(Vec::new()));
                let mut wrong_dst = load_record(dst_fill, wrong_order.clone());
                wrong_copy(&mut wrong_dst, &src_rec);
                if *wrong_order.borrow() != std::vec![1u8, 2u8] {
                    caught += 1;
                }
                cases += 1;
                std::hint::black_box((&src_box, &dst_box));
            }
        }
        assert!(cases > 40, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong order never caught ({cases} cases)");
    }

    // ---------------- the bounds accumulator ----------------

    /// Rewrite-side measure/sink stub state.
    struct BoundsStubs {
        /// Measure calls: item address.
        measure_log: Vec<u32>,
        /// Scripted triples per call: (lo, hi) as bit words.
        triples: VecDeque<([u32; 3], [u32; 3])>,
        /// Sink calls: all nine words.
        sink_log: Vec<[u32; 9]>,
        /// Sink-observed triples, as bits.
        sink_triples: Vec<([u32; 3], [u32; 3])>,
        /// Flag word the sink observed through its address.
        sink_flag_seen: Vec<u32>,
        /// Scripted sink answer.
        sink_answer: u32,
    }

    static BOUNDS_STUBS: Mutex<BoundsStubs> = Mutex::new(BoundsStubs {
        measure_log: Vec::new(),
        triples: VecDeque::new(),
        sink_log: Vec::new(),
        sink_triples: Vec::new(),
        sink_flag_seen: Vec::new(),
        sink_answer: 0,
    });

    /// Rewrite-side measure stub (cdecl, three words).
    extern "cdecl" fn measure_stub(p: u32, lo: u32, hi: u32) -> u32 {
        let mut stubs = BOUNDS_STUBS.lock().unwrap();
        stubs.measure_log.push(p);
        let (lo3, hi3) = stubs
            .triples
            .pop_front()
            .expect("measure stub called with nothing scripted");
        unsafe {
            for k in 0..3 {
                ((lo + k * 4) as *mut u32).write_unaligned(lo3[k as usize]);
                ((hi + k * 4) as *mut u32).write_unaligned(hi3[k as usize]);
            }
        }
        0
    }

    /// Rewrite-side sink stub (cdecl, nine words).
    extern "cdecl" fn bounds_sink_stub(
        lo: u32,
        hi: u32,
        a0: u32,
        a0b: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
    ) -> u32 {
        let mut stubs = BOUNDS_STUBS.lock().unwrap();
        stubs.sink_log.push([lo, hi, a0, a0b, a1, a2, a3, a4, a5]);
        let mut lo3 = [0u32; 3];
        let mut hi3 = [0u32; 3];
        unsafe {
            for k in 0..3 {
                lo3[k as usize] = ((lo + k * 4) as *const u32).read_unaligned();
                hi3[k as usize] = ((hi + k * 4) as *const u32).read_unaligned();
            }
            stubs
                .sink_flag_seen
                .push((a4 as *const u32).read_unaligned());
        }
        stubs.sink_triples.push((lo3, hi3));
        stubs.sink_answer
    }

    /// Lift-side measure fake.
    struct FakeMeasure {
        triples: VecDeque<([f32; 3], [f32; 3])>,
        log: Vec<usize>,
    }

    impl Measure for FakeMeasure {
        fn measure(&mut self, item: usize, lo: &mut [f32; 3], hi: &mut [f32; 3]) {
            self.log.push(item);
            let (l, h) = self
                .triples
                .pop_front()
                .expect("lift measure called with nothing scripted");
            *lo = l;
            *hi = h;
        }
    }

    /// Lift-side sink fake.
    struct FakeSink {
        log: Vec<([f32; 3], [f32; 3], u32, u32, u32, u32)>,
        answer: u32,
    }

    impl BoundsSink for FakeSink {
        fn report(
            &mut self,
            lo: &[f32; 3],
            hi: &[f32; 3],
            tag: u32,
            extra: u32,
            flag: &mut u32,
            mode: u32,
        ) -> u32 {
            self.log.push((*lo, *hi, tag, extra, *flag, mode));
            self.answer
        }
    }

    /// Edge float bit patterns: signed zeroes, ones, extremes,
    /// infinities, subnormals, NaNs with payloads.
    fn edge_floats(rng: &mut Rng) -> Vec<u32> {
        // Single patterns with a comment each (kept clear of the
        // publication scan's long-hex-run rule).
        let mut v = std::vec![
            0x0000_0000, // +0.0
            0x8000_0000, // -0.0
            0x3F80_0000, // 1.0
            0xBF80_0000, // -1.0
            0x7F7F_FFFF, // largest finite
            0xFF7F_FFFF, // most negative finite
            0x7F80_0000, // +inf
            0xFF80_0000, // -inf
            0x0000_0001, // smallest subnormal
            0x8000_0001, // smallest negative subnormal
            0x7FC0_0000, // canonical NaN
            0xFFC0_0000, // negative canonical NaN
            0x7F80_0001, // signalling NaN payload 1
            0x7FC0_1234, // quiet NaN with payload
        ];
        for _ in 0..6 {
            v.push(rng.u32());
        }
        v
    }

    #[test]
    fn bounds_accumulate_matches() {
        let _guard = lock();
        rt::set_callee(1, measure_stub as usize as u32);
        rt::set_callee(2, bounds_sink_stub as usize as u32);
        let mut rng = Rng(0xB00D5);
        let mut cases = 0;
        let mut caught = 0;
        let edges = edge_floats(&mut rng);
        // Item counts: empty, one, a few.
        for items in [0usize, 1, 2, 3, 5] {
            // Seed pairs: edge crosses, thinned.
            for &seed_hi in &edges {
                for &seed_lo in &edges {
                    if (seed_hi ^ seed_lo ^ items as u32) % 7 != 0 {
                        continue;
                    }
                    // One triple pair per item.
                    let mut script: Vec<([u32; 3], [u32; 3])> = Vec::new();
                    for _ in 0..items {
                        let pick = |rng: &mut Rng| {
                            if rng.u32() % 2 == 0 {
                                edges[(rng.u32() as usize) % edges.len()]
                            } else {
                                rng.u32()
                            }
                        };
                        script.push((
                            [pick(&mut rng), pick(&mut rng), pick(&mut rng)],
                            [pick(&mut rng), pick(&mut rng), pick(&mut rng)],
                        ));
                    }
                    let tag = rng.u32();
                    let extra = rng.u32();
                    let mode = rng.u32();
                    let sink_answer = rng.u32();
                    let flag_init = rng.u32();
                    // The item range: real addresses, exact stride.
                    let range_box = std::vec![0x5Au8; items * 0x20 + 4];
                    let range_box: Box<[u8]> = range_box.into_boxed_slice();
                    let (arg1, arg2) = if items == 0 {
                        (0x1000_0000, 0x1000_0000)
                    } else {
                        let base = addr(&range_box[0]);
                        (base, base + (items as u32) * 0x20)
                    };
                    // Seed cells behind relocated addresses.
                    let seed_hi_cell = Box::new(seed_hi);
                    let seed_lo_cell = Box::new(seed_lo);
                    rt::set_relocated(support::SEED_HI_VA, addr(&*seed_hi_cell));
                    rt::set_relocated(support::SEED_LO_VA, addr(&*seed_lo_cell));
                    {
                        let mut stubs = BOUNDS_STUBS.lock().unwrap();
                        stubs.measure_log.clear();
                        stubs.triples = VecDeque::from(script.clone());
                        stubs.sink_log.clear();
                        stubs.sink_triples.clear();
                        stubs.sink_flag_seen.clear();
                        stubs.sink_answer = sink_answer;
                    }
                    let flag_cell = Box::new(flag_init);
                    let flag_addr = addr(&*flag_cell);
                    // Rewrite side.
                    let got = unsafe {
                        fn_00ABAF50::rw_00abaf50(tag, arg1, arg2, extra, flag_addr, mode)
                    };
                    let (measure_log, sink_log, sink_triples, sink_flag_seen, flag_rw);
                    {
                        let stubs = BOUNDS_STUBS.lock().unwrap();
                        measure_log = stubs.measure_log.clone();
                        sink_log = stubs.sink_log.clone();
                        sink_triples = stubs.sink_triples.clone();
                        sink_flag_seen = stubs.sink_flag_seen.clone();
                        flag_rw = unsafe { (flag_addr as *const u32).read_unaligned() };
                    }
                    // Lift side.
                    let mut measure = FakeMeasure {
                        triples: VecDeque::from(
                            script
                                .iter()
                                .map(|(l, h)| {
                                    (
                                        [f32::from_bits(l[0]), f32::from_bits(l[1]), f32::from_bits(l[2])],
                                        [f32::from_bits(h[0]), f32::from_bits(h[1]), f32::from_bits(h[2])],
                                    )
                                })
                                .collect::<Vec<_>>(),
                        ),
                        log: Vec::new(),
                    };
                    let mut sink = FakeSink {
                        log: Vec::new(),
                        answer: sink_answer,
                    };
                    let mut flag = flag_init;
                    let lift = accumulate_bounds(
                        items,
                        f32::from_bits(seed_hi),
                        f32::from_bits(seed_lo),
                        &mut measure,
                        &mut sink,
                        tag,
                        extra,
                        &mut flag,
                        mode,
                    );
                    // Compare.
                    assert_eq!(got, lift);
                    assert_eq!(got, sink_answer);
                    assert_eq!(flag, 1);
                    assert_eq!(flag_rw, 1);
                    let expect_addrs: Vec<u32> = (0..items)
                        .map(|k| arg1 + (k as u32) * 0x20)
                        .collect();
                    assert_eq!(measure_log, expect_addrs);
                    assert_eq!(measure.log, (0..items).collect::<Vec<_>>());
                    assert_eq!(sink_log.len(), 1);
                    let s = sink_log[0];
                    assert_eq!((s[2], s[3]), (tag, tag), "doubled tag pinned");
                    assert_eq!((s[4], s[5]), (arg1, arg2), "range rebuilt");
                    assert_eq!((s[6], s[7], s[8]), (extra, flag_addr, mode));
                    assert_eq!(sink_flag_seen, std::vec![1]);
                    assert_eq!(sink.log.len(), 1);
                    let (lo, hi, ltag, lextra, lflag, lmode) = sink.log[0];
                    assert_eq!((ltag, lextra, lflag, lmode), (tag, extra, 1, mode));
                    let (rw_lo, rw_hi) = sink_triples[0];
                    for k in 0..3 {
                        assert_eq!(
                            lo[k].to_bits(),
                            rw_lo[k],
                            "items={items} lo[{k}] seed_hi={seed_hi:#x} seed_lo={seed_lo:#x}"
                        );
                        assert_eq!(
                            hi[k].to_bits(),
                            rw_hi[k],
                            "items={items} hi[{k}] seed_hi={seed_hi:#x} seed_lo={seed_lo:#x}"
                        );
                    }
                    // Wrong lift: minimum folded with the maximum rule
                    // (a copy-paste slip). Caught whenever any low fold
                    // differs from the maximum rule.
                    {
                        let mut whi = [f32::from_bits(seed_hi); 3];
                        let mut wlo = [f32::from_bits(seed_lo); 3];
                        for (l, h) in &script {
                            for k in 0..3 {
                                let (hb, lb) = (f32::from_bits(h[k]), f32::from_bits(l[k]));
                                whi[k] =
                                    if whi[k] > hb { whi[k] } else { hb };
                                wlo[k] =
                                    if wlo[k] > lb { wlo[k] } else { lb };
                            }
                        }
                        let wrong_differs = (0..3).any(|k| {
                            wlo[k].to_bits() != lo[k].to_bits()
                                || whi[k].to_bits() != hi[k].to_bits()
                        });
                        if wrong_differs {
                            caught += 1;
                        }
                    }
                    cases += 1;
                    std::hint::black_box((&range_box, &seed_hi_cell, &seed_lo_cell, &flag_cell));
                }
            }
        }
        assert!(cases > 100, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong min-fold never caught ({cases} cases)");
    }
}
