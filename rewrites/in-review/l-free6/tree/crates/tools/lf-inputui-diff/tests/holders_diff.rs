//! Differential cases, part 1: the six holder creation routines.
//!
//! Each case plants one counter global, one relocated table/helper word,
//! one garbage-filled block and one scripted callee/poll sequence, runs
//! the rewrite and the lifted method on the same inputs, and compares
//! the return, the block bytes (the table word rebuilt from the lifted
//! kind), the counter, and every call in order with its arguments. Each
//! method has a deliberately wrong lift that must be caught. 32-bit
//! target only.
//!
//! The 32-bit form faults on a null block in the four creates (it loads
//! the table through null); the lift panics there instead. That path is
//! out of the proof domain and pinned by a host test.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::input_ui::holders::{
        Construct, ConstructedBody, CounterState, FreshBlock, HolderAlloc, HolderKind, HolderSink,
        PollSlot, UiHolder,
    };
    use lf_inputui_diff::rewrites::*;
    use lf_inputui_diff::rt;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// Script + logs shared by the rewrite-side stubs of one case.
    /// The test mutates it only while holding the binary lock.
    struct StubState {
        next_slot: u32,
        /// Queued alloc answers: block garbage, or `None` for failure.
        queued: VecDeque<Option<Vec<u8>>>,
        /// Live blocks (kept alive past the rewrite call).
        live: Vec<Box<[u8]>>,
        /// (block address, slot) pairs handed out, in order.
        blocks: Vec<(u32, u32)>,
        /// Alloc calls: (size, tag).
        alloc_log: Vec<(u32, u32)>,
        /// Sink calls: block address or 0.
        sink_log: Vec<u32>,
        /// Scripted sink answer.
        sink_answer: u32,
        /// Constructor calls: (raw, helper, args).
        ctor_log: Vec<(u32, u32, Vec<u32>)>,
        /// Scripted id word the constructor leaves at `+4`.
        ctor_link: u32,
        /// Table address the constructor plants at `+0`.
        vt_addr: u32,
        /// Poll calls: holder address.
        poll_log: Vec<u32>,
        /// Scripted poll answers, popped in order.
        poll_answers: VecDeque<u32>,
    }

    impl StubState {
        fn new() -> Self {
            StubState {
                next_slot: 1,
                queued: VecDeque::new(),
                live: Vec::new(),
                blocks: Vec::new(),
                alloc_log: Vec::new(),
                sink_log: Vec::new(),
                sink_answer: 0,
                ctor_log: Vec::new(),
                ctor_link: 0,
                vt_addr: 0,
                poll_log: Vec::new(),
                poll_answers: VecDeque::new(),
            }
        }

        fn reset(&mut self) {
            *self = StubState::new();
        }
    }

    static STATE: Mutex<StubState> = Mutex::new(StubState {
        next_slot: 1,
        queued: VecDeque::new(),
        live: Vec::new(),
        blocks: Vec::new(),
        alloc_log: Vec::new(),
        sink_log: Vec::new(),
        sink_answer: 0,
        ctor_log: Vec::new(),
        ctor_link: 0,
        vt_addr: 0,
        poll_log: Vec::new(),
        poll_answers: VecDeque::new(),
    });

    fn state() -> std::sync::MutexGuard<'static, StubState> {
        STATE.lock().unwrap()
    }

    /// Rewrite-side allocator stub (cdecl, two words).
    extern "cdecl" fn alloc_stub(size: u32, tag: u32) -> u32 {
        let mut st = state();
        st.alloc_log.push((size, tag));
        match st.queued.pop_front() {
            Some(Some(garbage)) => {
                assert_eq!(
                    garbage.len() as u32,
                    size,
                    "alloc stub queued block of another size"
                );
                let block: Box<[u8]> = garbage.into_boxed_slice();
                let address = addr(&block[0]);
                let slot = st.next_slot;
                st.next_slot += 1;
                st.blocks.push((address, slot));
                st.live.push(block);
                address
            }
            Some(None) => 0,
            None => panic!("alloc stub called with nothing queued"),
        }
    }

    /// Rewrite-side sink stub (cdecl, one word).
    extern "cdecl" fn sink_stub(obj: u32) -> u32 {
        let mut st = state();
        st.sink_log.push(obj);
        st.sink_answer
    }

    /// Shared constructor-stub tail: logs, plants table + id word.
    fn ctor_tail(raw: u32, helper: u32, args: Vec<u32>) -> u32 {
        let mut st = state();
        st.ctor_log.push((raw, helper, args));
        let vt = st.vt_addr;
        let link = st.ctor_link;
        drop(st);
        unsafe {
            ((raw) as *mut u32).write_unaligned(vt);
            ((raw + 4) as *mut u32).write_unaligned(link);
        }
        raw
    }

    /// Rewrite-side five-argument constructor stub (thiscall, 7 words).
    extern "thiscall" fn ctor5_stub(
        raw: u32,
        helper: u32,
        p0: u32,
        p1: u32,
        rect: u32,
        st_arg: u32,
        flag: u32,
    ) -> u32 {
        ctor_tail(raw, helper, std::vec![p0, p1, rect, st_arg, flag])
    }

    /// Rewrite-side six-argument constructor stub (thiscall, 8 words).
    extern "thiscall" fn ctor6_stub(
        raw: u32,
        helper: u32,
        p0: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
    ) -> u32 {
        ctor_tail(raw, helper, std::vec![p0, a0, a1, a2, a3, a4])
    }

    /// Rewrite-side texture-blit constructor stub (thiscall, 6 words).
    extern "thiscall" fn ctor_tex_stub(
        raw: u32,
        helper: u32,
        p0: u32,
        a0: u32,
        a1: u32,
        a2: u32,
    ) -> u32 {
        ctor_tail(raw, helper, std::vec![p0, a0, a1, a2])
    }

    /// Rewrite-side slot-poll stub (thiscall, one word).
    extern "thiscall" fn poll_stub(obj: u32) -> u32 {
        let mut st = state();
        st.poll_log.push(obj);
        st.poll_answers
            .pop_front()
            .expect("poll stub called with nothing scripted")
    }

    /// Lift-side allocator: replays one queued (slot, garbage id word).
    struct FakeAlloc {
        queue: VecDeque<Option<(u32, u32)>>,
        log: Vec<u32>,
    }

    impl HolderAlloc for FakeAlloc {
        fn alloc(&mut self, size: u32) -> Option<FreshBlock> {
            self.log.push(size);
            match self.queue.pop_front() {
                Some(Some((slot, link0))) => Some(FreshBlock { slot, link0 }),
                Some(None) => None,
                None => panic!("lift alloc called with nothing queued"),
            }
        }
    }

    /// Lift-side sink: records the holder it saw.
    struct FakeSink {
        seen: Vec<Option<UiHolder>>,
        answer: u32,
    }

    impl HolderSink for FakeSink {
        fn receive(&mut self, holder: Option<&UiHolder>) -> u32 {
            self.seen.push(holder.cloned());
            self.answer
        }
    }

    /// Lift-side constructor: echoes the slot, hands back the kind + id.
    struct FakeCtor {
        kind: HolderKind,
        link: u32,
        log: Vec<(u32, Vec<u32>)>,
    }

    impl Construct for FakeCtor {
        fn construct(&mut self, slot: u32, args: &[u32]) -> ConstructedBody {
            self.log.push((slot, args.to_vec()));
            ConstructedBody {
                kind: self.kind,
                link: self.link,
            }
        }
    }

    /// Lift-side poll: replays scripted answers, records holders.
    struct FakePoll {
        answers: VecDeque<u32>,
        log: Vec<UiHolder>,
    }

    impl PollSlot for FakePoll {
        fn poll(&mut self, holder: &UiHolder) -> u32 {
            self.log.push(holder.clone());
            self.answers
                .pop_front()
                .expect("lift poll called with nothing scripted")
        }
    }

    /// Reads one word of a rewrite-side block through a raw pointer, so
    /// the compiler cannot forward the pre-call value.
    unsafe fn block_word(address: u32, off: u32) -> u32 {
        unsafe { ((address + off) as *const u32).read_unaligned() }
    }

    /// Deliberately wrong stamp: masks 12 bits instead of 14. Must be
    /// caught whenever bits 12..14 of `old ^ counter` are set.
    fn wrong_stamp(old: u32, counter: u32) -> u32 {
        old ^ ((old ^ counter) & 0x0FFF)
    }

    /// Deliberately wrong fold: unsigned `% 16` / `/ 16` instead of the
    /// signed pair. Must be caught whenever a poll answer is negative.
    fn wrong_fold_unsigned(link: u32, v1: u32, v2: u32) -> (u32, u32) {
        let r1 = v1 % 16;
        let t = (16u32.wrapping_sub(r1)) % 16;
        let q = v2.wrapping_add(t) / 16;
        let m = ((q << 14) ^ link) & lf_input_frontend::input_ui::holders::FOLD_MASK;
        (link ^ m, m)
    }

    /// Deliberately wrong fold: shifts by 13 instead of 14. Must be
    /// caught whenever the quotient is nonzero.
    fn wrong_fold_shift(link: u32, v1: u32, v2: u32) -> (u32, u32) {
        let r1 = (v1 as i32) % 16;
        let t = (16 - r1) % 16;
        let q = (v2 as i32).wrapping_add(t) / 16;
        let m = (((q as u32).wrapping_shl(13)) ^ link) & 0x01FF_C000;
        (link ^ m, m)
    }

    /// Edge words for id/counter/payload slots.
    fn edge_words(rng: &mut Rng) -> Vec<u32> {
        let mut words = std::vec![
            0x0000_0000,
            0x0000_0001,
            0x0000_2FFF,
            0x0000_3FFF,
            0x0000_4000,
            0x0000_FFFF,
            0x0001_0000,
            0x1234_5678,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFE,
            0xFFFF_FFFF,
        ];
        for _ in 0..6 {
            words.push(rng.u32());
        }
        words
    }

    /// Edge poll answers: zero, small, sign corners, extremes.
    fn poll_words(rng: &mut Rng) -> Vec<u32> {
        let mut words = std::vec![
            0x0000_0000,
            0x0000_0001,
            0x0000_000F,
            0x0000_0010,
            0x0000_0011,
            0x7FFF_FFF0,
            0x7FFF_FFFF,
            0x8000_0000,
            0x8000_0001,
            0xFFFF_FFF0,
            0xFFFF_FFFF,
        ];
        for _ in 0..5 {
            words.push(rng.u32());
        }
        words
    }

    /// Runs the two factory instances. Returns (comparisons, caught).
    fn run_factory(kind: HolderKind, table_va: u32, seed: u32) -> (u32, u32) {
        assert!(kind.payload_len() == 1 || kind.payload_len() == 2);
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        rt::set_callee(1, alloc_stub as *const () as usize as u32);
        rt::set_callee(2, sink_stub as *const () as usize as u32);
        // The planted table: one word; its address is what relocated()
        // answers and what the rewrite must write at +0.
        let table_cell = Box::new(0x71A8_1000u32);
        let table_addr = addr(&*table_cell);
        rt::set_relocated(table_va, table_addr);
        let link0s = edge_words(&mut rng);
        let counters = edge_words(&mut rng);
        let payloads = edge_words(&mut rng);
        // (ok, link0, counter0, payload word, head word, sink answer).
        for &link0 in &link0s {
            for &counter0 in &counters {
                for (pi, &pay) in payloads.iter().enumerate() {
                    for &null in &[false, true] {
                        if pi > 3 && (link0 ^ counter0 ^ pay ^ (pi as u32)) % 3 != 0 {
                            continue; // thin the full cross product
                        }
                        let head = rng.u32();
                        let sink_answer = rng.u32();
                        let size = kind.alloc_size() as usize;
                        let mut garbage = std::vec![0u8; size];
                        rng.bytes(&mut garbage);
                        garbage[4..8].copy_from_slice(&link0.to_le_bytes());
                        {
                            let mut st = state();
                            st.reset();
                            st.sink_answer = sink_answer;
                            if null {
                                st.queued.push_back(None);
                            } else {
                                st.queued.push_back(Some(garbage));
                            }
                        }
                        unsafe {
                            rt::global::<u32>(support::COUNTER_VA).write_unaligned(counter0);
                        }
                        let head_cell = Box::new(head);
                        let head_addr = addr(&*head_cell);
                        // Rewrite side.
                        let got = match kind {
                            HolderKind::Callback16 => unsafe {
                                fn_005EEC50::rw_005eec50(pay, head_addr)
                            },
                            HolderKind::Callback12 => unsafe { fn_005EECB0::rw_005eecb0(pay) },
                            _ => unreachable!(),
                        };
                        let (alloc_log, sink_log, blocks);
                        let counter_rw;
                        {
                            let st = state();
                            alloc_log = st.alloc_log.clone();
                            sink_log = st.sink_log.clone();
                            blocks = st.blocks.clone();
                            counter_rw =
                                unsafe { rt::global::<u32>(support::COUNTER_VA).read_unaligned() };
                        }
                        // Lift side.
                        let mut alloc = FakeAlloc {
                            queue: VecDeque::from(if null {
                                std::vec![None]
                            } else {
                                std::vec![Some((1, link0))]
                            }),
                            log: Vec::new(),
                        };
                        let mut sink = FakeSink {
                            seen: Vec::new(),
                            answer: sink_answer,
                        };
                        let mut counter = CounterState::new(counter0);
                        let payload = if kind.payload_len() == 2 {
                            std::vec![pay, head]
                        } else {
                            std::vec![pay]
                        };
                        let lift = UiHolder::create_callback(
                            &mut alloc,
                            &mut sink,
                            &mut counter,
                            kind,
                            &payload,
                        );
                        // Compare.
                        assert_eq!(got, lift, "kind={kind} null={null}");
                        assert_eq!(got, sink_answer, "kind={kind} null={null}");
                        assert_eq!(alloc_log, std::vec![(kind.alloc_size(), 0)]);
                        assert_eq!(alloc.log, std::vec![kind.alloc_size()]);
                        assert_eq!(counter_rw, counter.next, "kind={kind} null={null}");
                        if null {
                            assert_eq!(sink_log, std::vec![0]);
                            assert_eq!(sink.seen, std::vec![None]);
                            assert_eq!(counter.next, counter0);
                        } else {
                            let (address, slot) = blocks[0];
                            assert_eq!(slot, 1);
                            assert_eq!(sink_log, std::vec![address]);
                            let expect_link = UiHolder::stamp_link(link0, counter0);
                            let holder =
                                UiHolder::from_parts(1, kind, expect_link, payload.clone());
                            assert_eq!(sink.seen, std::vec![Some(holder)]);
                            unsafe {
                                assert_eq!(block_word(address, 0), table_addr);
                                assert_eq!(block_word(address, 4), expect_link);
                                assert_eq!(block_word(address, 8), pay);
                                if kind.payload_len() == 2 {
                                    assert_eq!(block_word(address, 12), head);
                                }
                            }
                            assert_eq!(counter.next, counter0.wrapping_add(1));
                            if wrong_stamp(link0, counter0) != expect_link {
                                caught += 1;
                            }
                        }
                        cases += 1;
                        std::hint::black_box(&table_cell);
                    }
                }
            }
        }
        (cases, caught)
    }

    /// Runs one constructor-create instance. Returns (comparisons, caught).
    fn run_ctor(
        kind: HolderKind,
        helper_va: u32,
        which: u8,
        seed: u32,
        wrong: fn(u32, u32, u32) -> (u32, u32),
    ) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        rt::set_callee(1, alloc_stub as *const () as usize as u32);
        let ctor_stub: u32 = match which {
            5 => ctor5_stub as *const () as usize as u32,
            6 => ctor6_stub as *const () as usize as u32,
            _ => ctor_tex_stub as *const () as usize as u32,
        };
        rt::set_callee(2, ctor_stub);
        // Planted vtable: slot +8 (index 2) is the poll stub.
        let vt = Box::new([
            0xDEAD_0000u32,
            0xDEAD_0004,
            poll_stub as *const () as usize as u32,
        ]);
        let vt_addr = addr(&vt[0]);
        let helper_cell = Box::new(0x71C0_0000u32);
        let helper_addr = addr(&*helper_cell);
        rt::set_relocated(helper_va, helper_addr);
        let links = edge_words(&mut rng);
        let polls = poll_words(&mut rng);
        // Words after ecx: five, six, and four (the tex form's p0
        // arrives in edx with three more on the stack).
        let arity = match which {
            5 => 5,
            6 => 6,
            _ => 4,
        };
        for &ctor_link in &links {
            for &v1 in &polls {
                for &v2 in &polls {
                    if (ctor_link ^ v1 ^ v2 ^ seed) % 4 != 0 {
                        continue; // thin the full cross product
                    }
                    let mut args = Vec::new();
                    for _ in 0..arity {
                        args.push(rng.u32());
                    }
                    let size = kind.alloc_size() as usize;
                    let mut garbage = std::vec![0u8; size];
                    rng.bytes(&mut garbage);
                    {
                        let mut st = state();
                        st.reset();
                        st.queued.push_back(Some(garbage));
                        st.ctor_link = ctor_link;
                        st.vt_addr = vt_addr;
                        st.poll_answers.push_back(v1);
                        st.poll_answers.push_back(v2);
                    }
                    // Rewrite side.
                    let got = match which {
                        5 => unsafe {
                            fn_005EED10::rw_005eed10(0, args[0], args[1], args[2], args[3], args[4])
                        },
                        6 => unsafe {
                            fn_005EEDA0::rw_005eeda0(
                                0, args[0], args[1], args[2], args[3], args[4], args[5],
                            )
                        },
                        _ => unsafe {
                            fn_005EEEF0::rw_005eeef0(0, args[0], args[1], args[2], args[3])
                        },
                    };
                    let (alloc_log, ctor_log, poll_log, blocks);
                    {
                        let st = state();
                        alloc_log = st.alloc_log.clone();
                        ctor_log = st.ctor_log.clone();
                        poll_log = st.poll_log.clone();
                        blocks = st.blocks.clone();
                    }
                    // Lift side.
                    let mut alloc = FakeAlloc {
                        queue: VecDeque::from(std::vec![Some((1, 0))]),
                        log: Vec::new(),
                    };
                    let mut ctor = FakeCtor {
                        kind,
                        link: ctor_link,
                        log: Vec::new(),
                    };
                    let mut poll = FakePoll {
                        answers: VecDeque::from(std::vec![v1, v2]),
                        log: Vec::new(),
                    };
                    let (holder, lift) = UiHolder::create_through_ctor(
                        &mut alloc, &mut ctor, &mut poll, kind, &args,
                    );
                    // Compare.
                    assert_eq!(got, lift, "kind={kind} v1={v1:#x} v2={v2:#x}");
                    assert_eq!(alloc_log, std::vec![(kind.alloc_size(), 0)]);
                    assert_eq!(alloc.log, std::vec![kind.alloc_size()]);
                    let (address, slot) = blocks[0];
                    assert_eq!(slot, 1);
                    assert_eq!(ctor_log.len(), 1);
                    assert_eq!(ctor_log[0].0, address);
                    assert_eq!(ctor_log[0].1, helper_addr, "kind={kind}: helper rebuilt");
                    assert_eq!(ctor_log[0].2, args);
                    assert_eq!(ctor.log, std::vec![(1, args.clone())]);
                    assert_eq!(poll_log, std::vec![address, address]);
                    assert_eq!(poll.log.len(), 2);
                    assert!(poll.log.iter().all(|h| h.slot() == 1 && h.kind() == kind));
                    let (expect_link, expect_m) = {
                        let r1 = (v1 as i32) % 16;
                        let t = (16 - r1) % 16;
                        let q = (v2 as i32).wrapping_add(t) / 16;
                        let m = (((q as u32).wrapping_shl(14)) ^ ctor_link) & 0x01FF_C000;
                        (ctor_link ^ m, m)
                    };
                    assert_eq!(lift, expect_m);
                    assert_eq!(holder.link(), expect_link);
                    unsafe {
                        assert_eq!(block_word(address, 0), vt_addr);
                        assert_eq!(block_word(address, 4), expect_link);
                    }
                    if wrong(ctor_link, v1, v2) != (expect_link, expect_m) {
                        caught += 1;
                    }
                    cases += 1;
                    std::hint::black_box((&vt, &helper_cell));
                }
            }
        }
        (cases, caught)
    }

    /// Runs the inline create. Returns (comparisons, caught).
    fn run_inline(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        rt::set_callee(1, alloc_stub as *const () as usize as u32);
        let vt = Box::new([
            0xDEAD_1000u32,
            0xDEAD_1004,
            poll_stub as *const () as usize as u32,
        ]);
        let vt_addr = addr(&vt[0]);
        let helper_cell = Box::new(0x71C0_1000u32);
        let helper_addr = addr(&*helper_cell);
        rt::set_relocated(support::VTINLINE_VA, vt_addr);
        rt::set_relocated(support::FNHELPER_VA, helper_addr);
        let link0s = edge_words(&mut rng);
        let counters = edge_words(&mut rng);
        let polls = poll_words(&mut rng);
        for &link0 in &link0s {
            for &counter0 in &counters {
                for &v1 in &polls {
                    for &v2 in &polls {
                        if (link0 ^ counter0 ^ v1 ^ v2 ^ seed) % 16 != 0 {
                            continue; // thin the full cross product
                        }
                        let head = rng.u32();
                        let w = [rng.u32(), rng.u32(), rng.u32()];
                        let size = HolderKind::Inline28.alloc_size() as usize;
                        let mut garbage = std::vec![0u8; size];
                        rng.bytes(&mut garbage);
                        garbage[4..8].copy_from_slice(&link0.to_le_bytes());
                        {
                            let mut st = state();
                            st.reset();
                            st.queued.push_back(Some(garbage));
                            st.poll_answers.push_back(v1);
                            st.poll_answers.push_back(v2);
                        }
                        unsafe {
                            rt::global::<u32>(support::COUNTER_VA).write_unaligned(counter0);
                        }
                        let head_cell = Box::new(head);
                        let w_cells = Box::new(w);
                        let head_addr = addr(&*head_cell);
                        let w_addrs = [addr(&w_cells[0]), addr(&w_cells[1]), addr(&w_cells[2])];
                        // Rewrite side.
                        let got = unsafe {
                            fn_005EEE30::rw_005eee30(
                                0, head_addr, w_addrs[0], w_addrs[1], w_addrs[2],
                            )
                        };
                        let (alloc_log, poll_log, blocks, counter_rw);
                        {
                            let st = state();
                            alloc_log = st.alloc_log.clone();
                            poll_log = st.poll_log.clone();
                            blocks = st.blocks.clone();
                            counter_rw =
                                unsafe { rt::global::<u32>(support::COUNTER_VA).read_unaligned() };
                        }
                        // Lift side.
                        let mut alloc = FakeAlloc {
                            queue: VecDeque::from(std::vec![Some((1, link0))]),
                            log: Vec::new(),
                        };
                        let mut counter = CounterState::new(counter0);
                        let mut poll = FakePoll {
                            answers: VecDeque::from(std::vec![v1, v2]),
                            log: Vec::new(),
                        };
                        let (holder, lift) =
                            UiHolder::create_inline(&mut alloc, &mut counter, &mut poll, head, w);
                        // Compare.
                        assert_eq!(got, lift, "v1={v1:#x} v2={v2:#x}");
                        assert_eq!(alloc_log, std::vec![(0x1C, 0)]);
                        assert_eq!(alloc.log, std::vec![0x1C]);
                        assert_eq!(counter_rw, counter.next);
                        assert_eq!(counter.next, counter0.wrapping_add(1));
                        let (address, slot) = blocks[0];
                        assert_eq!(slot, 1);
                        assert_eq!(poll_log, std::vec![address, address]);
                        assert_eq!(poll.log.len(), 2);
                        let stamped = UiHolder::stamp_link(link0, counter0);
                        let r1 = (v1 as i32) % 16;
                        let t = (16 - r1) % 16;
                        let q = (v2 as i32).wrapping_add(t) / 16;
                        let m = (((q as u32).wrapping_shl(14)) ^ stamped) & 0x01FF_C000;
                        let expect_link = stamped ^ m;
                        assert_eq!(lift, m);
                        assert_eq!(holder.link(), expect_link);
                        assert_eq!(holder.words(), &[head, w[0], w[1], w[2]]);
                        unsafe {
                            assert_eq!(block_word(address, 0), vt_addr);
                            assert_eq!(block_word(address, 4), expect_link);
                            assert_eq!(block_word(address, 8), helper_addr);
                            assert_eq!(block_word(address, 12), head);
                            assert_eq!(block_word(address, 16), w[0]);
                            assert_eq!(block_word(address, 20), w[1]);
                            assert_eq!(block_word(address, 24), w[2]);
                        }
                        if wrong_fold_shift(stamped, v1, v2) != (expect_link, m) {
                            caught += 1;
                        }
                        cases += 1;
                        std::hint::black_box((&vt, &helper_cell));
                    }
                }
            }
        }
        (cases, caught)
    }

    #[test]
    fn factory_10_matches() {
        let _guard = lock();
        let (cases, caught) = run_factory(HolderKind::Callback16, support::VT16_VA, 0xF010);
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong stamp never caught ({cases} cases)");
    }

    #[test]
    fn factory_0c_matches() {
        let _guard = lock();
        let (cases, caught) = run_factory(HolderKind::Callback12, support::VT12_VA, 0xF00C);
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong stamp never caught ({cases} cases)");
    }

    #[test]
    fn create_5arg_matches() {
        let _guard = lock();
        let (cases, caught) = run_ctor(
            HolderKind::Ctor5,
            support::CB5_VA,
            5,
            0xC705,
            wrong_fold_unsigned,
        );
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong fold never caught ({cases} cases)");
    }

    #[test]
    fn create_6arg_matches() {
        let _guard = lock();
        let (cases, caught) = run_ctor(
            HolderKind::Ctor6,
            support::CB6_VA,
            6,
            0xC706,
            wrong_fold_unsigned,
        );
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong fold never caught ({cases} cases)");
    }

    #[test]
    fn create_tex_matches() {
        let _guard = lock();
        let (cases, caught) = run_ctor(
            HolderKind::CtorTex,
            support::CBTEX_VA,
            3,
            0xC7E7,
            wrong_fold_unsigned,
        );
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong fold never caught ({cases} cases)");
    }

    #[test]
    fn create_inline_matches() {
        let _guard = lock();
        let (cases, caught) = run_inline(0x1CE0);
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong fold never caught ({cases} cases)");
    }
}
