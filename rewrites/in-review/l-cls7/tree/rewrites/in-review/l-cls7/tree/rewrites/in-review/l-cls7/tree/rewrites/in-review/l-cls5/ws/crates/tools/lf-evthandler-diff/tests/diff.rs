//! Differential cases: each lifted event-handler method against its
//! verified rewrite on the same generated inputs.
//!
//! Every case compares the return value and the full handler and event
//! blobs (catching stray writes), plus, for the three slots that call
//! out, the recorded calls against the lift's trait calls. Each method
//! also runs a deliberately wrong lift, which must be caught at least
//! once. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_peds_tasks::event_handler::{
        EventDispatch, EventHandler, EventPayload, EventSource, Owner, Task,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_KIND, E_PAYLOAD, E_VTABLE, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr,
        event_blob, fake_table, handler_blob,
    };

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_core::Handle32;
        use lf_peds_tasks::event_handler::{
            EventDispatch, EventHandler, EventPayload, EventSource, KIND_CLEAR, Task,
        };

        /// Forgets the second clearing kind.
        pub fn type_clear_forgets_b(h: &mut EventHandler, kind: u32) -> u32 {
            if kind == KIND_CLEAR {
                h.clear_pending_on_type(kind);
            }
            kind
        }

        /// Forgets the payload gate.
        pub fn reset_ignores_payload(h: &mut EventHandler, kind: u32) {
            h.reset_pending_on_kind(kind, true);
        }

        /// Always forwards, never clears.
        pub fn forward_always(
            h: &mut EventHandler,
            kind: u32,
            payload: Option<Handle32<EventPayload>>,
            disp: &mut impl EventDispatch,
        ) {
            let _ = h;
            disp.dispatch_event(kind, payload);
        }

        /// Stops after two polls, skipping the third.
        pub fn settle_two(h: &EventHandler, ev: &mut impl EventSource) {
            if ev.poll_owner().is_none() {
                return;
            }
            if ev.poll_owner() == h.owner() {
                return;
            }
        }

        /// Stores the clone but answers the previous task.
        pub fn adopt_answers_old(
            pending: &mut Option<Handle32<Task>>,
            task: Option<Handle32<Task>>,
        ) -> Option<Handle32<Task>> {
            let old = *pending;
            *pending = task;
            old
        }
    }

    /// Event kinds: edge words, the clearing kinds with neighbours, random.
    fn kinds(rng: &mut Rng) -> Vec<u32> {
        let mut v = U32_EDGE.to_vec();
        // The shared clearing kind and its neighbours.
        v.extend([0xC7, 0xC8, 0xC9]);
        // The type-clear slot's second kind and its neighbours.
        v.extend([0x200, 0x201, 0x202]);
        // The reset slot's second kind and its neighbours.
        v.extend([0x3A6, 0x3A7, 0x3A8]);
        for _ in 0..64 {
            v.push(rng.u32());
        }
        v
    }

    /// A handler blob with random filler and the given owner and task.
    fn handler(rng: &mut Rng, owner: u32, pending: u32) -> Box<[u32; 4]> {
        let mut h = handler_blob();
        h[H_VTABLE] = rng.u32();
        h[H_OWNER] = owner;
        h[H_PAD] = rng.u32();
        h[H_PENDING] = pending;
        h
    }

    #[test]
    fn type_clear_matches() {
        let mut rng = Rng(0xE0E7);
        let mut caught = 0;
        let mut cases = 0;
        let ks = kinds(&mut rng);
        let mut inputs = Vec::new();
        for &kind in &ks {
            for &pending in &U32_EDGE {
                inputs.push((kind, rng.u32(), pending));
            }
        }
        let mut run = |kind: u32, owner: u32, pending: u32, caught: &mut u32| {
            let h = handler(&mut rng, owner, pending);
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_KIND] = kind;
            let h_before = *h;
            let e_before = *e;
            let got = unsafe { fn_00CA6D60::rw_00ca6d60(addr(&h[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            let mut lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let lift_ret = lift.clear_pending_on_type(kind);
            assert_eq!(got, lift_ret, "kind {kind:#x}");
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*h, expect_h, "kind {kind:#x}");
            assert_eq!(*e, e_before, "kind {kind:#x}");
            let mut w = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let w_ret = wrong::type_clear_forgets_b(&mut w, kind);
            if w_ret != got || w.pending() != lift.pending() {
                *caught += 1;
            }
            cases += 1;
        };
        for (kind, owner, pending) in inputs {
            run(kind, owner, pending, &mut caught);
        }
        assert!(caught > 0, "wrong type-clear never caught ({cases} cases)");
    }

    #[test]
    fn reset_on_kind_matches() {
        let mut rng = Rng(0x1EC7);
        let mut caught = 0;
        let mut cases = 0;
        let ks = kinds(&mut rng);
        let payloads = [0u32, 1, 0x8000_0000, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &kind in &ks {
            for &payload in &payloads {
                for &pending in &U32_EDGE {
                    inputs.push((kind, payload, pending));
                }
            }
        }
        let mut run = |kind: u32, payload: u32, pending: u32, caught: &mut u32| {
            let o = rng.u32();
            let h = handler(&mut rng, o, pending);
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_KIND] = kind;
            e[E_PAYLOAD] = payload;
            let h_before = *h;
            let e_before = *e;
            let got = unsafe { fn_00CA7C70::rw_00ca7c70(addr(&h[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            assert_eq!(got, 0, "kind {kind:#x} payload {payload:#x}");
            let mut lift =
                EventHandler::new(Handle32::new(h_before[H_OWNER]), Handle32::new(pending));
            lift.reset_pending_on_kind(kind, payload != 0);
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*h, expect_h, "kind {kind:#x} payload {payload:#x}");
            assert_eq!(*e, e_before, "kind {kind:#x}");
            let mut w = EventHandler::new(Handle32::new(h_before[H_OWNER]), Handle32::new(pending));
            wrong::reset_ignores_payload(&mut w, kind);
            if w.pending() != lift.pending() {
                *caught += 1;
            }
            cases += 1;
        };
        for (kind, payload, pending) in inputs {
            run(kind, payload, pending, &mut caught);
        }
        assert!(caught > 0, "wrong reset never caught ({cases} cases)");
    }

    // The forward-or-clear slot's dispatch stub and its recording.
    static DISP_THIS: AtomicU32 = AtomicU32::new(0);
    static DISP_KIND: AtomicU32 = AtomicU32::new(0);
    static DISP_PAYLOAD: AtomicU32 = AtomicU32::new(0);
    static DISP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn dispatch_stub(this: u32, kind: u32, payload: u32) -> u32 {
        DISP_THIS.store(this, Ordering::SeqCst);
        DISP_KIND.store(kind, Ordering::SeqCst);
        DISP_PAYLOAD.store(payload, Ordering::SeqCst);
        DISP_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn dispatch_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = dispatch_stub;
        f as usize as u32
    }

    struct DispatchRec {
        calls: Vec<(u32, u32)>,
    }

    impl EventDispatch for DispatchRec {
        fn dispatch_event(&mut self, kind: u32, payload: Option<Handle32<EventPayload>>) {
            self.calls.push((kind, Handle32::raw_or_zero(payload)));
        }
    }

    #[test]
    fn forward_or_clear_matches() {
        // Dispatch slot byte offset 0x134, as a word index.
        const SLOT: usize = 0x134 / 4;
        let mut rng = Rng(0xF0A1);
        let mut caught = 0;
        let mut cases = 0;
        let ks = kinds(&mut rng);
        let payloads = [0u32, 1, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &kind in &ks {
            for &payload in &payloads {
                for &pending in &U32_EDGE {
                    inputs.push((kind, payload, pending));
                }
            }
        }
        let mut run = |kind: u32, payload: u32, pending: u32, caught: &mut u32| {
            let table = fake_table(0x140 / 4, SLOT, dispatch_addr());
            let o = rng.u32();
            let mut h = handler(&mut rng, o, pending);
            h[H_VTABLE] = addr(&table[0]);
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_KIND] = kind;
            e[E_PAYLOAD] = payload;
            let h_before = *h;
            let e_before = *e;
            DISP_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA79C0::rw_00ca79c0(addr(&h[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            assert_eq!(got, 0, "kind {kind:#x}");
            let stub_calls = DISP_COUNT.load(Ordering::SeqCst);
            assert!(stub_calls <= 1, "dispatch called {stub_calls}x");
            if stub_calls == 1 {
                assert_eq!(DISP_THIS.load(Ordering::SeqCst), addr(&h[0]));
            }
            let mut lift =
                EventHandler::new(Handle32::new(h_before[H_OWNER]), Handle32::new(pending));
            let mut rec = DispatchRec { calls: Vec::new() };
            lift.forward_or_clear(kind, Handle32::new(payload), &mut rec);
            assert_eq!(rec.calls.len() as u32, stub_calls, "kind {kind:#x}");
            if stub_calls == 1 {
                assert_eq!(rec.calls[0].0, DISP_KIND.load(Ordering::SeqCst));
                assert_eq!(rec.calls[0].1, DISP_PAYLOAD.load(Ordering::SeqCst));
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*h, expect_h, "kind {kind:#x}");
            assert_eq!(*e, e_before, "kind {kind:#x}");
            let mut w = EventHandler::new(Handle32::new(h_before[H_OWNER]), Handle32::new(pending));
            let mut wrec = DispatchRec { calls: Vec::new() };
            wrong::forward_always(&mut w, kind, Handle32::new(payload), &mut wrec);
            if w.pending() != lift.pending() || wrec.calls.len() as u32 != stub_calls {
                *caught += 1;
            }
            cases += 1;
        };
        for (kind, payload, pending) in inputs {
            run(kind, payload, pending, &mut caught);
        }
        assert!(caught > 0, "wrong forward never caught ({cases} cases)");
    }

    // The settle slot's poll stub: scripted answers with a recorded trail.
    static POLL_ANS: [AtomicU32; 3] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];
    static POLL_SEEN: [AtomicU32; 3] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];
    static POLL_IDX: AtomicU32 = AtomicU32::new(0);
    static POLL_THIS: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn poll_stub(this: u32) -> u32 {
        POLL_THIS.store(this, Ordering::SeqCst);
        let i = POLL_IDX.fetch_add(1, Ordering::SeqCst) as usize;
        let a = POLL_ANS[i.min(2)].load(Ordering::SeqCst);
        if i < 3 {
            POLL_SEEN[i].store(a, Ordering::SeqCst);
        }
        a
    }

    fn poll_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = poll_stub;
        f as usize as u32
    }

    struct Script {
        answers: [u32; 3],
        idx: usize,
        seen: Vec<u32>,
    }

    impl EventSource for Script {
        fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
            let a = self.answers[self.idx.min(2)];
            self.idx += 1;
            self.seen.push(a);
            Handle32::new(a)
        }

        fn clone_task(&mut self) -> Option<Handle32<Task>> {
            panic!("settle cases never clone");
        }
    }

    /// Poll scripts: every shape the three-way branch can take.
    fn scripts(owner: u32, other: u32, rng: &mut Rng) -> Vec<[u32; 3]> {
        let mut v = vec![
            [0, 0, 0],
            [other, owner, 0],
            [other, 1 ^ owner ^ 0x5EED, other],
            [other, 0, 0],
            [other, other, other],
            [other, other, owner],
            [owner, owner, owner],
        ];
        for _ in 0..8 {
            v.push([rng.u32(), rng.u32(), rng.u32()]);
        }
        v
    }

    #[test]
    fn settle_poll_matches() {
        // Poll slot byte offset 0x34, as a word index.
        const SLOT: usize = 0x34 / 4;
        let mut rng = Rng(0x9E77);
        let mut caught = 0;
        let mut cases = 0;
        let owners = [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &owner in &owners {
            let other = rng.u32() | 1;
            for script in scripts(owner, other, &mut rng) {
                inputs.push((owner, 0u32, script));
                inputs.push((owner, 7u32, script));
                inputs.push((owner, rng.u32(), script));
            }
        }
        let mut run = |owner: u32, pending: u32, script: [u32; 3], caught: &mut u32| {
            let table = fake_table(0x40 / 4, SLOT, poll_addr());
            let h = handler(&mut rng, owner, pending);
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_VTABLE] = addr(&table[0]);
            let h_before = *h;
            let e_before = *e;
            for (i, a) in script.iter().enumerate() {
                POLL_ANS[i].store(*a, Ordering::SeqCst);
            }
            POLL_IDX.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA79F0::rw_00ca79f0(addr(&h[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            assert_eq!(got, 0);
            let stub_count = POLL_IDX.load(Ordering::SeqCst) as usize;
            assert!((1..=3).contains(&stub_count), "poll called {stub_count}x");
            assert_eq!(POLL_THIS.load(Ordering::SeqCst), addr(&e[0]));
            let mut stub_seen = Vec::new();
            for i in 0..stub_count {
                stub_seen.push(POLL_SEEN[i].load(Ordering::SeqCst));
            }
            let lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut fake = Script {
                answers: script,
                idx: 0,
                seen: Vec::new(),
            };
            lift.settle_poll(&mut fake);
            assert_eq!(fake.seen, stub_seen, "script {script:?} owner {owner:#x}");
            // The slot stores nothing: both blobs survive byte for byte.
            assert_eq!(*h, h_before);
            assert_eq!(*e, e_before);
            let mut wfake = Script {
                answers: script,
                idx: 0,
                seen: Vec::new(),
            };
            wrong::settle_two(&lift, &mut wfake);
            if wfake.seen != stub_seen {
                *caught += 1;
            }
            cases += 1;
        };
        for (owner, pending, script) in inputs {
            run(owner, pending, script, &mut caught);
        }
        assert!(caught > 0, "wrong settle never caught ({cases} cases)");
    }

    // The adopt slot's clone stub and its recording.
    static CLONE_ANS: AtomicU32 = AtomicU32::new(0);
    static CLONE_THIS: AtomicU32 = AtomicU32::new(0);
    static CLONE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn clone_stub(this: u32) -> u32 {
        CLONE_THIS.store(this, Ordering::SeqCst);
        CLONE_COUNT.fetch_add(1, Ordering::SeqCst);
        CLONE_ANS.load(Ordering::SeqCst)
    }

    fn clone_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = clone_stub;
        f as usize as u32
    }

    struct Cloner {
        answer: u32,
        calls: u32,
    }

    impl EventSource for Cloner {
        fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
            panic!("adopt cases never poll");
        }

        fn clone_task(&mut self) -> Option<Handle32<Task>> {
            self.calls += 1;
            Handle32::new(self.answer)
        }
    }

    #[test]
    fn adopt_cloned_task_matches() {
        // Clone slot byte offset 0x54, as a word index.
        const SLOT: usize = 0x54 / 4;
        let mut rng = Rng(0xAD07);
        let mut caught = 0;
        let mut cases = 0;
        let mut inputs = Vec::new();
        for &answer in &U32_EDGE {
            for &pending in &U32_EDGE {
                inputs.push((answer, pending));
            }
        }
        for _ in 0..64 {
            inputs.push((rng.u32(), rng.u32()));
        }
        let mut run = |answer: u32, pending: u32, caught: &mut u32| {
            let table = fake_table(0x60 / 4, SLOT, clone_addr());
            let o = rng.u32();
            let h = handler(&mut rng, o, pending);
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_VTABLE] = addr(&table[0]);
            let h_before = *h;
            let e_before = *e;
            CLONE_ANS.store(answer, Ordering::SeqCst);
            CLONE_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA8E00::rw_00ca8e00(addr(&h[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            assert_eq!(CLONE_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(CLONE_THIS.load(Ordering::SeqCst), addr(&e[0]));
            let mut lift =
                EventHandler::new(Handle32::new(h_before[H_OWNER]), Handle32::new(pending));
            let mut fake = Cloner { answer, calls: 0 };
            let lift_ret = lift.adopt_cloned_task(&mut fake);
            assert_eq!(fake.calls, 1);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*h, expect_h);
            assert_eq!(*e, e_before);
            let mut wpending = Handle32::new(pending);
            let w_ret = wrong::adopt_answers_old(&mut wpending, Handle32::new(answer));
            if Handle32::raw_or_zero(w_ret) != got {
                *caught += 1;
            }
            cases += 1;
        };
        for (answer, pending) in inputs {
            run(answer, pending, &mut caught);
        }
        assert!(caught > 0, "wrong adopt never caught ({cases} cases)");
    }
}
