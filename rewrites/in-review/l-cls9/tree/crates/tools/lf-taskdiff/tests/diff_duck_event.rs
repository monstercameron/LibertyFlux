//! Differential case: the duck event slot against its verified rewrite
//! on the same generated inputs.
//!
//! Compares the return value, the full task and object blobs, and the
//! announce, query and trigger calls against the lift's trait calls,
//! both sides given the same scripted answers. The query and trigger
//! stubs live in a fake object table. A deliberately wrong lift must be
//! caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{
        DuckEvent, DuckQuery, DuckSink, DuckTask, QUERY_CODE, QUERY_STATE,
    };
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::set_callee;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        D_FLAGGED_BYTE, D_LEVEL, Q_STATE, Q_VTABLE, Rng, U32_EDGE, addr, blob_byte, duck_blob,
        fake_table2, ped_blob, query_blob, set_blob_byte,
    };

    // Announce stub (slot 1) and its recording.
    static ANN_SINK: AtomicU32 = AtomicU32::new(0);
    static ANN_ZERO: AtomicU32 = AtomicU32::new(0);
    static ANN_MASK: AtomicU32 = AtomicU32::new(0);
    static ANN_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn announce_stub(sink: u32, zero: u32, mask: u32) -> u32 {
        ANN_SINK.store(sink, Ordering::SeqCst);
        ANN_ZERO.store(zero, Ordering::SeqCst);
        ANN_MASK.store(mask, Ordering::SeqCst);
        ANN_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Query stub (object slot +4) and its recording.
    static QUERY_OBJ: AtomicU32 = AtomicU32::new(0);
    static QUERY_ANS: AtomicU32 = AtomicU32::new(0);
    static QUERY_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn query_stub(obj: u32) -> u32 {
        QUERY_OBJ.store(obj, Ordering::SeqCst);
        QUERY_COUNT.fetch_add(1, Ordering::SeqCst);
        QUERY_ANS.load(Ordering::SeqCst)
    }

    // Trigger stub (object slot +0x34) and its recording.
    static TRIGGER_OBJ: AtomicU32 = AtomicU32::new(0);
    static TRIGGER_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn trigger_stub(obj: u32) -> u32 {
        TRIGGER_OBJ.store(obj, Ordering::SeqCst);
        TRIGGER_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn announce_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = announce_stub;
        f as usize as u32
    }

    fn query_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = query_stub;
        f as usize as u32
    }

    fn trigger_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = trigger_stub;
        f as usize as u32
    }

    struct Fake {
        query_answer: u32,
        announces: Vec<u32>,
        queries: Vec<u32>,
        triggers: Vec<u32>,
    }

    impl DuckEvent for Fake {
        fn announce(&mut self, sink: Option<Handle32<DuckSink>>) {
            self.announces.push(Handle32::raw_or_zero(sink));
        }

        fn query(&mut self, obj: Handle32<DuckQuery>) -> u32 {
            self.queries.push(obj.get());
            self.query_answer
        }

        fn trigger(&mut self, obj: Handle32<DuckQuery>) {
            self.triggers.push(obj.get());
        }
    }

    /// Triggers on the code alone, forgetting the state word.
    fn wrong_event(
        task: &mut DuckTask,
        sink: Option<Handle32<DuckSink>>,
        event: u32,
        obj: Option<Handle32<DuckQuery>>,
        target: &mut Fake,
    ) -> u32 {
        if event == 2 {
            target.announce(sink);
            return 1;
        }
        if event == 1 {
            if task.level() > -1 {
                if let Some(obj) = obj {
                    if target.query(obj) == QUERY_CODE {
                        target.trigger(obj);
                    }
                }
            }
            target.announce(sink);
            return 1;
        }
        // The flag set is modelled through the real slot shape.
        task.handle_event(sink, event, None, 0, target)
    }

    /// Events: edge words, the handled pair with neighbours, random.
    fn events(rng: &mut Rng) -> Vec<u32> {
        let mut v = U32_EDGE.to_vec();
        v.extend([0, 1, 2, 3]);
        for _ in 0..8 {
            v.push(rng.u32());
        }
        v
    }

    #[test]
    fn duck_event_matches() {
        // Query slot byte offset 0x4, trigger slot 0x34, as word indexes.
        const QUERY_SLOT: usize = 0x4 / 4;
        const TRIGGER_SLOT: usize = 0x34 / 4;
        set_callee(1, announce_addr());
        let mut rng = Rng(0xD0CE);
        let mut caught = 0;
        let mut cases = 0;
        // Levels around the -1 gate plus extremes and random halves.
        let levels = [
            -2i16, -1, 0, 1, 2, i16::MIN, i16::MAX, 0x1234, -0x1234,
        ];
        let queries = [0x30u32, QUERY_CODE, QUERY_CODE + 1, 0, u32::MAX, rng.u32()];
        let states = [
            QUERY_STATE - 1,
            QUERY_STATE,
            QUERY_STATE + 1,
            0,
            u32::MAX,
            rng.u32(),
        ];
        let mut inputs = Vec::new();
        for &event in &events(&mut rng) {
            for &level in &levels {
                for &null_obj in &[false, true] {
                    for &query in &queries {
                        for &state in &states {
                            inputs.push((event, level, null_obj, query, state));
                        }
                    }
                }
            }
        }
        for (event, level, null_obj, query, state) in inputs {
            let table = fake_table2(0x40 / 4, QUERY_SLOT, query_addr(), TRIGGER_SLOT, trigger_addr());
            let sink_blob = ped_blob();
            let sink_addr = if rng.u32() & 1 == 0 { 0 } else { addr(&sink_blob[0]) };
            let mut q = query_blob();
            for w in q.iter_mut() {
                *w = rng.u32();
            }
            q[Q_VTABLE] = addr(&table[0]);
            q[Q_STATE] = state;
            let q_before = *q;
            let obj_addr = if null_obj { 0 } else { addr(&q[0]) };
            let mut blob = *duck_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[D_LEVEL] = (blob[D_LEVEL] & 0xFFFF_0000) | u32::from(level as u16);
            let flagged_before = blob_byte(&blob, D_FLAGGED_BYTE);
            let boxed = Box::new(blob);
            let before = *boxed;
            ANN_COUNT.store(0, Ordering::SeqCst);
            QUERY_COUNT.store(0, Ordering::SeqCst);
            TRIGGER_COUNT.store(0, Ordering::SeqCst);
            QUERY_ANS.store(query, Ordering::SeqCst);
            let got = unsafe {
                fn_00D4DF30::rw_00d4df30(addr(&boxed[0]), sink_addr, event, obj_addr)
            };
            let probes = event == 1 && level > -1 && !null_obj;
            let want_announces = u32::from(event == 1 || event == 2);
            assert_eq!(ANN_COUNT.load(Ordering::SeqCst), want_announces);
            if want_announces == 1 {
                assert_eq!(ANN_SINK.load(Ordering::SeqCst), sink_addr);
                assert_eq!(ANN_ZERO.load(Ordering::SeqCst), 0);
                assert_eq!(ANN_MASK.load(Ordering::SeqCst), 0xFFFF_FFFF);
            }
            assert_eq!(QUERY_COUNT.load(Ordering::SeqCst), u32::from(probes));
            if probes {
                assert_eq!(QUERY_OBJ.load(Ordering::SeqCst), obj_addr);
            }
            let want_triggers = u32::from(probes && query == QUERY_CODE && state == QUERY_STATE);
            assert_eq!(TRIGGER_COUNT.load(Ordering::SeqCst), want_triggers);
            if want_triggers == 1 {
                assert_eq!(TRIGGER_OBJ.load(Ordering::SeqCst), obj_addr);
            }
            let expect_ret = if event == 1 || event == 2 { 1 } else { 0 };
            assert_eq!(got, expect_ret, "event {event:#x}");
            let mut lift = DuckTask::new(0, 0, 0, level, false, flagged_before != 0, 0);
            let mut fake = Fake {
                query_answer: query,
                announces: Vec::new(),
                queries: Vec::new(),
                triggers: Vec::new(),
            };
            let lift_ret = lift.handle_event(
                Handle32::new(sink_addr),
                event,
                Handle32::new(obj_addr),
                state,
                &mut fake,
            );
            assert_eq!(lift_ret, got, "event {event:#x}");
            assert_eq!(fake.announces.len() as u32, want_announces);
            if want_announces == 1 {
                assert_eq!(fake.announces[0], sink_addr);
            }
            assert_eq!(fake.queries.len() as u32, u32::from(probes));
            assert_eq!(fake.triggers.len() as u32, want_triggers);
            // Only the flag byte may change, and only off the handled pair.
            let mut expect = before;
            if event != 1 && event != 2 {
                set_blob_byte(&mut expect, D_FLAGGED_BYTE, 1);
            }
            assert_eq!(*boxed, expect, "event {event:#x}");
            assert_eq!(*q, q_before);
            assert_eq!(sink_blob[..], [0u32; 4][..]);
            let mut wrong = DuckTask::new(0, 0, 0, level, false, flagged_before != 0, 0);
            let mut wfake = Fake {
                query_answer: query,
                announces: Vec::new(),
                queries: Vec::new(),
                triggers: Vec::new(),
            };
            let w_ret = wrong_event(
                &mut wrong,
                Handle32::new(sink_addr),
                event,
                Handle32::new(obj_addr),
                &mut wfake,
            );
            if w_ret != got
                || wfake.triggers.len() as u32 != want_triggers
                || wrong.flagged() != lift.flagged()
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong duck event never caught ({cases} cases)");
    }
}
