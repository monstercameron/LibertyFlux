//! Differential case: the route slot against its verified rewrite on
//! the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs, the two
//! owner polls, the block dispatch and the two factory calls (slot and
//! arguments in order) against the lift's trait calls, both sides given
//! the same scripted answers. A deliberately wrong lift must be caught
//! at least once. One binary per slot shape, so the stub registry is
//! never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee, set_float_a, set_float_b, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventDispatch, EventHandler, EventPayload, EventProbe, EventRef,
        EventSource, FactoryHandle, FactoryState, KIND_CLEAR, KIND_RESET_B, KIND_ROUTE_BUILD,
        Owner, ROUTE_BLOCK, RouteInput, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_KIND, E_VTABLE, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, addr, event_blob_wide,
        fake_table, handler_blob,
    };

    // Poll stub (event table slot 0x34) and its recording.
    static POLL_THIS: AtomicU32 = AtomicU32::new(0);
    static POLL_ANS: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
    static POLL_SEEN: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
    static POLL_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn poll_stub(this: u32) -> u32 {
        POLL_THIS.store(this, Ordering::SeqCst);
        let i = POLL_COUNT.fetch_add(1, Ordering::SeqCst) as usize;
        let a = POLL_ANS[i.min(1)].load(Ordering::SeqCst);
        if i < 2 {
            POLL_SEEN[i].store(a, Ordering::SeqCst);
        }
        a
    }

    fn poll_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = poll_stub;
        f as usize as u32
    }

    // Dispatch stub (handler table slot 0x130) and its recording.
    static DISP_THIS: AtomicU32 = AtomicU32::new(0);
    static DISP_KIND: AtomicU32 = AtomicU32::new(0);
    static DISP_BLOCK: AtomicU32 = AtomicU32::new(0);
    static DISP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn dispatch_stub(this: u32, kind: u32, block: u32) -> u32 {
        DISP_THIS.store(this, Ordering::SeqCst);
        DISP_KIND.store(kind, Ordering::SeqCst);
        DISP_BLOCK.store(block, Ordering::SeqCst);
        DISP_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn dispatch_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = dispatch_stub;
        f as usize as u32
    }

    // Lookup stub (slot 3) and its recording.
    static LOOKUP_MGR: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_ANS: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn lookup_stub(mgr: u32) -> u32 {
        LOOKUP_MGR.store(mgr, Ordering::SeqCst);
        LOOKUP_COUNT.fetch_add(1, Ordering::SeqCst);
        LOOKUP_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    // Build stub (slot 4: block address then the two float words) and
    // its recording.
    static BUILD_H: AtomicU32 = AtomicU32::new(0);
    static BUILD_BLOCK: AtomicU32 = AtomicU32::new(0);
    static BUILD_FIRST: AtomicU32 = AtomicU32::new(0);
    static BUILD_SECOND: AtomicU32 = AtomicU32::new(0);
    static BUILD_ANS: AtomicU32 = AtomicU32::new(0);
    static BUILD_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn build_stub(h: u32, block: u32, first: u32, second: u32) -> u32 {
        BUILD_H.store(h, Ordering::SeqCst);
        BUILD_BLOCK.store(block, Ordering::SeqCst);
        BUILD_FIRST.store(first, Ordering::SeqCst);
        BUILD_SECOND.store(second, Ordering::SeqCst);
        BUILD_COUNT.fetch_add(1, Ordering::SeqCst);
        BUILD_ANS.load(Ordering::SeqCst)
    }

    fn build_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = build_stub;
        f as usize as u32
    }

    struct FakeSource {
        polls: [u32; 2],
        poll_calls: u32,
    }

    impl EventSource for FakeSource {
        fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
            let a = self.polls[(self.poll_calls as usize).min(1)];
            self.poll_calls += 1;
            Handle32::new(a)
        }

        fn clone_task(&mut self) -> Option<Handle32<Task>> {
            panic!("route cases never clone");
        }

        fn probe_kind(&mut self) -> u32 {
            panic!("route cases never probe kinds");
        }

        fn readiness(&mut self) -> u32 {
            panic!("route cases never check readiness");
        }

        fn probe(&mut self) -> Option<Handle32<EventProbe>> {
            panic!("route cases never probe");
        }
    }

    struct FakeDispatch {
        dispatches: Vec<(u32, u32, u32)>,
    }

    impl EventDispatch for FakeDispatch {
        fn dispatch_event(&mut self, _kind: u32, _payload: Option<Handle32<EventPayload>>) {
            panic!("route cases never dispatch events");
        }

        fn dispatch_block(&mut self, kind: u32, event: Handle32<EventRef>, block_offset: u32) {
            self.dispatches.push((kind, event.get(), block_offset));
        }
    }

    struct FakeFactory {
        lookup_answer: u32,
        convert_answer: u32,
        lookups: Vec<u32>,
        converts: Vec<(u32, ConvertRequest)>,
    }

    impl TaskFactory for FakeFactory {
        fn lookup(
            &mut self,
            manager: Option<Handle32<TaskManager>>,
        ) -> Option<Handle32<FactoryHandle>> {
            self.lookups.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.lookup_answer)
        }

        fn convert(
            &mut self,
            handle: Handle32<FactoryHandle>,
            request: ConvertRequest,
        ) -> Option<Handle32<Task>> {
            self.converts.push((handle.get(), request));
            Handle32::new(self.convert_answer)
        }
    }

    /// Wrong lift: skips the build on the build kind, leaving the task
    /// slot alone.
    fn wrong_route(
        h: &mut EventHandler,
        input: RouteInput,
        s: &mut FakeSource,
        d: &mut FakeDispatch,
        f: &mut FakeFactory,
        st: &FactoryState,
    ) {
        if input.kind == KIND_ROUTE_BUILD {
            let _ = (h, s, d, f, st);
            return;
        }
        h.route_by_owner_and_kind(input, s, d, f, st);
    }

    #[test]
    fn route_matches() {
        set_callee(3, lookup_addr());
        set_callee(4, build_addr());
        let mut rng = Rng(0xB071);
        let mut caught = 0;
        let mut cases = 0;
        // Float words as bits: zeros, ones, quiet values and payloads.
        let floats = [
            0u32,
            1,
            0x3F80_0000,
            0xBF80_0000,
            0x7FC0_0001,
            0xFF80_0000,
            rng.u32(),
        ];
        let others = [0u32, 1, 0xC7, 0xC9, 0x397, 0x399, 0x3A6, 0x3A8, rng.u32()];
        let mut inputs = Vec::new();
        for &owner in &[0u32, 1, rng.u32() | 1] {
            for &kind in &[KIND_CLEAR, KIND_RESET_B, KIND_ROUTE_BUILD] {
                for &o in &others {
                    let _ = o;
                    inputs.push((owner, kind));
                }
            }
            for &o in &others {
                inputs.push((owner, o));
            }
        }
        for (owner, kind) in inputs {
            for &poll_second in &[0u32, 1] {
                let other = rng.u32() | 1;
                let second = if poll_second == 0 { owner } else { other };
                // Poll scripts: early null, owner match, and pass-through.
                for script in [[0, second], [other, owner], [other, second]] {
                    let pending = rng.u32();
                    let manager = rng.u32();
                    let h = rng.u32();
                    let ans = rng.u32();
                    let fb = floats[(rng.u32() as usize) % floats.len()];
                    let fa = floats[(rng.u32() as usize) % floats.len()];
                    let mut hh = handler_blob();
                    let dtable = fake_table(0x140 / 4, 0x130 / 4, dispatch_addr());
                    hh[H_VTABLE] = addr(&dtable[0]);
                    hh[H_OWNER] = owner;
                    hh[H_PAD] = rng.u32();
                    hh[H_PENDING] = pending;
                    let mut e = event_blob_wide();
                    for w in e.iter_mut() {
                        *w = rng.u32();
                    }
                    let ptable = fake_table(0x40 / 4, 0x34 / 4, poll_addr());
                    e[E_VTABLE] = addr(&ptable[0]);
                    e[E_KIND] = kind;
                    let h_before = *hh;
                    let e_before = *e;
                    let ev = addr(&e[0]);
                    set_manager(manager);
                    set_float_a(fa);
                    set_float_b(fb);
                    POLL_ANS[0].store(script[0], Ordering::SeqCst);
                    POLL_ANS[1].store(script[1], Ordering::SeqCst);
                    LOOKUP_ANS.store(h, Ordering::SeqCst);
                    BUILD_ANS.store(ans, Ordering::SeqCst);
                    POLL_COUNT.store(0, Ordering::SeqCst);
                    DISP_COUNT.store(0, Ordering::SeqCst);
                    LOOKUP_COUNT.store(0, Ordering::SeqCst);
                    BUILD_COUNT.store(0, Ordering::SeqCst);
                    let got = unsafe { fn_00CA7BC0::rw_00ca7bc0(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
                    assert_eq!(got, 0, "kind {kind:#x}");
                    let polls = POLL_COUNT.load(Ordering::SeqCst);
                    assert!((1..=2).contains(&polls), "poll called {polls}x");
                    assert_eq!(POLL_THIS.load(Ordering::SeqCst), ev);
                    let mut stub_polls = Vec::new();
                    for i in 0..polls {
                        stub_polls.push(POLL_SEEN[i as usize].load(Ordering::SeqCst));
                    }
                    let dispatches = DISP_COUNT.load(Ordering::SeqCst);
                    assert!(dispatches <= 1);
                    let lookups = LOOKUP_COUNT.load(Ordering::SeqCst);
                    assert!(lookups <= 1);
                    let builds = BUILD_COUNT.load(Ordering::SeqCst);
                    assert!(builds <= 1);
                    if lookups == 1 {
                        assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
                    }
                    if builds == 1 {
                        assert_eq!(BUILD_H.load(Ordering::SeqCst), h);
                        assert_eq!(
                            BUILD_BLOCK.load(Ordering::SeqCst),
                            ev.wrapping_add(ROUTE_BLOCK)
                        );
                        assert_eq!(BUILD_FIRST.load(Ordering::SeqCst), fb);
                        assert_eq!(BUILD_SECOND.load(Ordering::SeqCst), fa);
                    }
                    if dispatches == 1 {
                        assert_eq!(DISP_THIS.load(Ordering::SeqCst), addr(&hh[0]));
                        assert_eq!(DISP_KIND.load(Ordering::SeqCst), kind);
                        assert_eq!(
                            DISP_BLOCK.load(Ordering::SeqCst),
                            ev.wrapping_add(ROUTE_BLOCK)
                        );
                    }
                    let mut lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
                    let state = FactoryState::new(Handle32::new(manager));
                    let mut source = FakeSource {
                        polls: script,
                        poll_calls: 0,
                    };
                    let mut dispatch = FakeDispatch {
                        dispatches: Vec::new(),
                    };
                    let mut factory = FakeFactory {
                        lookup_answer: h,
                        convert_answer: ans,
                        lookups: Vec::new(),
                        converts: Vec::new(),
                    };
                    let event = Handle32::new(ev).expect("test event address is live");
                    let input = RouteInput {
                        kind,
                        event,
                        first: f32::from_bits(fb),
                        second: f32::from_bits(fa),
                    };
                    lift.route_by_owner_and_kind(
                        input,
                        &mut source,
                        &mut dispatch,
                        &mut factory,
                        &state,
                    );
                    assert_eq!(source.poll_calls, polls, "kind {kind:#x}");
                    let mut fake_polls = Vec::new();
                    for i in 0..polls {
                        fake_polls.push(source.polls[(i as usize).min(1)]);
                    }
                    assert_eq!(fake_polls, stub_polls, "kind {kind:#x}");
                    assert_eq!(
                        dispatch.dispatches.len() as u32,
                        dispatches,
                        "kind {kind:#x}"
                    );
                    if dispatches == 1 {
                        assert_eq!(dispatch.dispatches[0], (kind, ev, ROUTE_BLOCK));
                    }
                    assert_eq!(factory.lookups, vec![manager; lookups as usize]);
                    assert_eq!(factory.converts.len() as u32, builds, "kind {kind:#x}");
                    if builds == 1 {
                        assert_eq!(
                            factory.converts[0],
                            (
                                h,
                                ConvertRequest::BlockFloats {
                                    block_offset: ROUTE_BLOCK,
                                    first_bits: fb,
                                    second_bits: fa,
                                }
                            )
                        );
                    }
                    let mut expect_h = h_before;
                    expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
                    assert_eq!(*hh, expect_h, "kind {kind:#x}");
                    assert_eq!(*e, e_before, "kind {kind:#x}");
                    // The wrong lift skips the build on the build kind:
                    // its pending word must differ from the stub's
                    // whenever a build ran, or its call log must miss
                    // the build.
                    let mut wlift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
                    let mut wsource = FakeSource {
                        polls: script,
                        poll_calls: 0,
                    };
                    let mut wdispatch = FakeDispatch {
                        dispatches: Vec::new(),
                    };
                    let mut wfactory = FakeFactory {
                        lookup_answer: h,
                        convert_answer: ans,
                        lookups: Vec::new(),
                        converts: Vec::new(),
                    };
                    wrong_route(
                        &mut wlift,
                        input,
                        &mut wsource,
                        &mut wdispatch,
                        &mut wfactory,
                        &state,
                    );
                    if builds == 1
                        && (Handle32::raw_or_zero(wlift.pending()) != hh[H_PENDING]
                            || wfactory.converts.len() != 1)
                    {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(caught > 0, "wrong route never caught ({cases} cases)");
    }
}
