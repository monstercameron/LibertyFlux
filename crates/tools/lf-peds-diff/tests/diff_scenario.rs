//! Differential cases for the move-between-points scenario methods.
//!
//! Each lifted method runs against its verified 32-bit rewrite on the same generated
//! inputs. The comparison covers the return value, every call in order with its
//! arguments (the callees and collaborators are stubs on the 32-bit side and host
//! calls on the lifted side), and every word or byte the method writes. Each lifted
//! method also has deliberately wrong versions built here; every one must be caught.
//! 32-bit target only: the rewrites take real addresses.

// Test-only harness. The verified rewrites take raw addresses, so the harness gives
// them the addresses of its own buffers and stubs. The lifted crate has no unsafe.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, MutexGuard, OnceLock};

    use lf_peds_diff::rewrites::{fn_00c7a880, fn_00c7b220, fn_00c7fc70, fn_00c819e0};
    use lf_peds_diff::rt::{set_callee, set_global};
    use lf_peds_tasks::move_between_points::{
        MoveBetweenPointsHost, MoveBetweenPointsScenario as Scen, SubTask,
    };

    /// The scenario-global word the heap object pointer is read from, as the rewrites name it.
    const HEAP_WORD_VA: u32 = 0x0167_E2A0;
    /// Tokens the lifted host answers with. They stand for the 32-bit addresses the
    /// rewrites see, and the log maps each back to the same symbol on both sides.
    const HEAP_TOKEN: u32 = 0x0000_0A11;
    const NEW_TOKEN: u32 = 0x0000_0B22;
    const PAYLOAD_TOKEN: u32 = 0x0000_0C33;

    /// One argument in the call log: a named object, or a plain word.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Arg {
        Sym(&'static str),
        Val(u32),
    }

    /// One call in the log: the callee's name and its arguments in push order.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Call {
        name: &'static str,
        args: Vec<Arg>,
    }

    /// Everything a case produces: the return value (as bits), the calls, and the effects.
    #[derive(Debug, PartialEq, Eq)]
    struct Outcome {
        ret: u64,
        calls: Vec<Call>,
        effects: Vec<Arg>,
    }

    /// A scenario object on the 32-bit side: 64 bytes, 8-byte aligned.
    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Obj([u8; 64]);

    /// A sub-task object on the 32-bit side: a vtable word, then the latch byte at +0xc.
    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct SubObj([u8; 16]);

    /// Log of the 32-bit stubs and of the lifted host, shared by both sides.
    static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
    /// Address-to-name table for the 32-bit side; set per case.
    static SYMS: Mutex<Vec<(u32, &'static str)>> = Mutex::new(Vec::new());
    /// Serialises the cases: the stubs and the log are process-wide.
    static SERIAL: Mutex<()> = Mutex::new(());

    /// Scripted answers of the 32-bit stubs, set per case.
    static ALLOC_RET: AtomicU32 = AtomicU32::new(0);
    static CTOR_RET: AtomicU32 = AtomicU32::new(0);
    static RUN_RET: AtomicU32 = AtomicU32::new(0);

    /// Buffers the 32-bit side needs for the whole run: the heap object, the heap word
    /// the rewrites read through the scenario-global address, a freshly allocated
    /// scenario, a payload object, and the sub-task vtable (slot 5 answers the run stub).
    struct Fixture {
        heap_obj: u32,
        new_obj: u32,
        payload_obj: u32,
        sub_vt: u32,
    }

    static FIX: OnceLock<Fixture> = OnceLock::new();

    fn serial() -> MutexGuard<'static, ()> {
        SERIAL.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn fixture() -> &'static Fixture {
        FIX.get_or_init(|| {
            let heap_obj: &'static mut [u32; 4] = Box::leak(Box::new([
                0x1111_0001,
                0x1111_0002,
                0x1111_0003,
                0x1111_0004,
            ]));
            let heap_obj = heap_obj.as_ptr() as usize as u32;
            let heap_word: &'static mut [u32; 1] = Box::leak(Box::new([heap_obj]));
            set_global(HEAP_WORD_VA, heap_word.as_ptr() as usize as u32);
            let new_obj: &'static mut [u32; 16] = Box::leak(Box::new([0x2222_0000; 16]));
            let payload_obj: &'static mut [u32; 4] = Box::leak(Box::new([0x3333_0000; 4]));
            let mut vt = [0u32; 8];
            // Slot 5 (byte offset 0x14) is the sub-task's run operation.
            vt[5] = run_stub as *const () as usize as u32;
            let vt: &'static mut [u32; 8] = Box::leak(Box::new(vt));
            Fixture {
                heap_obj,
                new_obj: new_obj.as_ptr() as usize as u32,
                payload_obj: payload_obj.as_ptr() as usize as u32,
                sub_vt: vt.as_ptr() as usize as u32,
            }
        })
    }

    fn record(name: &'static str, args: &[u32]) {
        let args = args.iter().map(|v| sym_of(*v)).collect();
        LOG.lock().unwrap().push(Call { name, args });
    }

    fn sym_of(v: u32) -> Arg {
        let syms = SYMS.lock().unwrap();
        match syms.iter().find(|(a, _)| *a == v) {
            Some((_, name)) => Arg::Sym(name),
            None => Arg::Val(v),
        }
    }

    fn set_syms(list: &[(u32, &'static str)]) {
        let mut syms = SYMS.lock().unwrap();
        syms.clear();
        syms.extend_from_slice(list);
    }

    fn take_log() -> Vec<Call> {
        std::mem::take(&mut *LOG.lock().unwrap())
    }

    /// The lifted side's name for a token: the same symbol the 32-bit side maps to.
    fn tok(v: u32) -> Arg {
        match v {
            HEAP_TOKEN => Arg::Sym("heap"),
            NEW_TOKEN => Arg::Sym("new"),
            PAYLOAD_TOKEN => Arg::Sym("payload"),
            _ => Arg::Val(v),
        }
    }

    fn put32(b: &mut [u8], off: usize, v: u32) {
        b[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn get32(b: &[u8], off: usize) -> u32 {
        u32::from_le_bytes(b[off..off + 4].try_into().expect("four bytes"))
    }

    // ---- 32-bit stubs: the callees and the sub-task's run operation ----

    /// Class destructor (callee 1 of the deleting destructor).
    extern "thiscall" fn class_dtor_stub(task: u32) -> u32 {
        record("class_destructor", &[task]);
        0
    }

    /// Heap free (callee 2 of the deleting destructor): heap in ECX, task pushed.
    extern "thiscall" fn free_stub(heap: u32, task: u32) -> u32 {
        record("heap_free", &[heap, task]);
        0
    }

    /// Heap allocation (callee 1 of the clone factory): heap in ECX.
    extern "thiscall" fn alloc_stub(heap: u32) -> u32 {
        record("heap_alloc", &[heap]);
        ALLOC_RET.load(Ordering::SeqCst)
    }

    /// Constructor (callee 2 of the clone factory): slot in ECX, three words pushed.
    extern "thiscall" fn ctor_stub(slot: u32, a: u32, b: u32, c: u32) -> u32 {
        record("construct", &[slot, a, b, c]);
        CTOR_RET.load(Ordering::SeqCst)
    }

    /// The sub-task's run operation, reached through its vtable slot 5.
    extern "thiscall" fn run_stub(sub: u32, a1: u32, a2: u32, a3: u32) -> u32 {
        record("run", &[sub, a1, a2, a3]);
        RUN_RET.load(Ordering::SeqCst)
    }

    /// Payload release (callee 2 of the run tick): cdecl, one word pushed.
    extern "cdecl" fn release_stub(payload: u32) -> u32 {
        record("release", &[payload]);
        0
    }

    // ---- lifted-side host and sub-task ----

    /// The lifted host. It records the same calls the 32-bit stubs record.
    struct Host {
        log: Rc<RefCell<Vec<Call>>>,
        alloc_ret: u32,
        ctor_ret: u32,
    }

    impl Host {
        fn push(&self, name: &'static str, args: Vec<Arg>) {
            self.log.borrow_mut().push(Call { name, args });
        }
    }

    impl MoveBetweenPointsHost for Host {
        fn heap_singleton(&mut self) -> u32 {
            HEAP_TOKEN
        }

        fn class_destructor(&mut self, _task: &mut Scen) {
            self.push("class_destructor", vec![Arg::Sym("task")]);
        }

        fn heap_free(&mut self, heap: u32, _task: &mut Scen) {
            self.push("heap_free", vec![tok(heap), Arg::Sym("task")]);
        }

        fn heap_alloc(&mut self, heap: u32) -> u32 {
            self.push("heap_alloc", vec![tok(heap)]);
            self.alloc_ret
        }

        fn construct(&mut self, slot: u32, aux_id: u32, aux_word: u32, pending_guard: u32) -> u32 {
            self.push(
                "construct",
                vec![
                    tok(slot),
                    Arg::Val(aux_id),
                    Arg::Val(aux_word),
                    Arg::Val(pending_guard),
                ],
            );
            self.ctor_ret
        }

        fn release_payload(&mut self, payload: u32) {
            self.push("release", vec![tok(payload)]);
        }
    }

    /// The lifted sub-task: a latch byte and a run answer, logging its runs.
    struct FakeSub {
        latch: u8,
        run_ret: u32,
        log: Rc<RefCell<Vec<Call>>>,
    }

    impl SubTask for FakeSub {
        fn latch(&self) -> u8 {
            self.latch
        }

        fn set_latch(&mut self, value: u8) {
            self.latch = value;
        }

        fn run(&mut self, a1: u32, a2: u32, a3: u32) -> u32 {
            self.log.borrow_mut().push(Call {
                name: "run",
                args: vec![Arg::Sym("sub"), Arg::Val(a1), Arg::Val(a2), Arg::Val(a3)],
            });
            self.run_ret
        }
    }

    fn lifted_log() -> Rc<RefCell<Vec<Call>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    fn drain(log: &Rc<RefCell<Vec<Call>>>) -> Vec<Call> {
        std::mem::take(&mut *log.borrow_mut())
    }

    // ---- blend_weight (verified rewrite: the scenario's seventh virtual method) ----

    fn blend_trials() -> Vec<(u32, u8, u32, u32)> {
        let mut out = Vec::new();
        for status in [0u32, 1, 2, 3, 0xFFFF_FFFD] {
            for active in [0u8, 1, 0x80] {
                for pending in [0u32, 0x5A5A, 0xFFFF_FFFF] {
                    for state in [0u32, 1, 2, 0x8000_0000, 0xFFFF_FFFF] {
                        out.push((status, active, pending, state));
                    }
                }
            }
        }
        out
    }

    fn blend_32(status: u32, active: u8, pending: u32, state: u32) -> Outcome {
        let mut obj = Obj([0u8; 64]);
        put32(&mut obj.0, 0x0C, status);
        obj.0[0x20] = active;
        put32(&mut obj.0, 0x1C, pending);
        put32(&mut obj.0, 0x28, state);
        let before = obj;
        let addr = obj.0.as_ptr() as usize as u32;
        set_syms(&[(addr, "task")]);
        let r = fn_00c7fc70::rw_00c7fc70(addr, 0);
        Outcome {
            ret: u64::from(r.to_bits()),
            calls: take_log(),
            effects: vec![Arg::Val(u32::from(obj.0 == before.0))],
        }
    }

    fn blend_lifted(
        status: u32,
        active: u8,
        pending: u32,
        state: u32,
        f: fn(&Scen) -> f32,
    ) -> Outcome {
        let s = Scen {
            status,
            active,
            pending_guard: pending,
            state,
            ..Scen::default()
        };
        Outcome {
            ret: u64::from(f(&s).to_bits()),
            calls: Vec::new(),
            effects: vec![Arg::Val(1)],
        }
    }

    /// Wrong versions of the blend weight, each changing one thing.
    fn blend_wrong_state_zero_only(s: &Scen) -> f32 {
        if (s.status >> 1) & 1 != 0 {
            return 25.0;
        }
        if s.active != 0 && s.pending_guard == 0 {
            return -1.0;
        }
        if s.state == 0 { 25.0 } else { -1.0 }
    }

    fn blend_wrong_bit0(s: &Scen) -> f32 {
        if s.status & 1 != 0 {
            return 25.0;
        }
        if s.active != 0 && s.pending_guard == 0 {
            return -1.0;
        }
        if s.state == 0 || s.state == 1 {
            25.0
        } else {
            -1.0
        }
    }

    fn blend_wrong_guard_polarity(s: &Scen) -> f32 {
        if (s.status >> 1) & 1 != 0 {
            return 25.0;
        }
        if s.active != 0 && s.pending_guard != 0 {
            return -1.0;
        }
        if s.state == 0 || s.state == 1 {
            25.0
        } else {
            -1.0
        }
    }

    #[test]
    fn blend_weight_matches_rewrite() {
        let _guard = serial();
        let _ = fixture();
        for (status, active, pending, state) in blend_trials() {
            let want = blend_32(status, active, pending, state);
            let got = blend_lifted(status, active, pending, state, Scen::blend_weight);
            assert_eq!(
                got, want,
                "blend weight {status:#x} {active} {pending:#x} {state:#x}"
            );
        }
    }

    #[test]
    fn blend_weight_wrong_versions_are_caught() {
        let _guard = serial();
        let _ = fixture();
        let wrong: [(&str, fn(&Scen) -> f32); 3] = [
            ("state zero only", blend_wrong_state_zero_only),
            ("bit zero of status", blend_wrong_bit0),
            ("guard polarity", blend_wrong_guard_polarity),
        ];
        for (name, f) in wrong {
            let caught = blend_trials()
                .into_iter()
                .filter(|(st, ac, pe, sa)| {
                    blend_32(*st, *ac, *pe, *sa) != blend_lifted(*st, *ac, *pe, *sa, f)
                })
                .count();
            eprintln!(
                "blend_weight wrong version ({name}): caught in {caught} of {} trials",
                blend_trials().len()
            );
            assert!(caught > 0, "wrong blend weight ({name}) was not caught");
        }
    }

    // ---- deleting destructor (verified rewrite: the scenario's first virtual method) ----

    const DTOR_FLAGS: [u32; 7] = [0, 1, 2, 3, 0x8000_0000, 0xFFFF_FFFF, 0xFFFF_FFFE];

    fn dtor_32(flags: u32) -> Outcome {
        let fx = fixture();
        let obj = Obj([0x5A; 64]);
        let before = obj;
        let addr = obj.0.as_ptr() as usize as u32;
        set_syms(&[(addr, "task"), (fx.heap_obj, "heap")]);
        set_callee(1, class_dtor_stub as *const () as usize as u32);
        set_callee(2, free_stub as *const () as usize as u32);
        let r = fn_00c7a880::rw_00c7a880(addr, flags);
        assert_eq!(r, addr, "the rewrite returns its argument");
        Outcome {
            ret: 0,
            calls: take_log(),
            effects: vec![Arg::Val(u32::from(obj.0 == before.0))],
        }
    }

    fn dtor_lifted(flags: u32, f: fn(&mut Scen, &mut Host, u32)) -> Outcome {
        let log = lifted_log();
        let mut host = Host {
            log: log.clone(),
            alloc_ret: 0,
            ctor_ret: 0,
        };
        let mut s = Scen::default();
        f(&mut s, &mut host, flags);
        Outcome {
            ret: 0,
            calls: drain(&log),
            effects: vec![Arg::Val(1)],
        }
    }

    fn dtor_wrong_bit1(s: &mut Scen, h: &mut Host, flags: u32) {
        h.class_destructor(s);
        if flags & 2 != 0 {
            let heap = h.heap_singleton();
            h.heap_free(heap, s);
        }
    }

    fn dtor_wrong_no_base(s: &mut Scen, h: &mut Host, flags: u32) {
        if flags & 1 != 0 {
            let heap = h.heap_singleton();
            h.heap_free(heap, s);
        }
    }

    fn dtor_wrong_order(s: &mut Scen, h: &mut Host, flags: u32) {
        if flags & 1 != 0 {
            let heap = h.heap_singleton();
            h.heap_free(heap, s);
        }
        h.class_destructor(s);
    }

    #[test]
    fn deleting_destructor_matches_rewrite() {
        let _guard = serial();
        let _ = fixture();
        let mut freed = 0;
        for flags in DTOR_FLAGS {
            let want = dtor_32(flags);
            let got = dtor_lifted(flags, Scen::deleting_destructor::<Host>);
            if want.calls.iter().any(|c| c.name == "heap_free") {
                freed += 1
            }
            assert_eq!(got, want, "deleting destructor, flags {flags:#x}");
        }
        assert!(freed > 0 && freed < DTOR_FLAGS.len());
    }

    #[test]
    fn deleting_destructor_wrong_versions_are_caught() {
        let _guard = serial();
        let _ = fixture();
        let wrong: [(&str, fn(&mut Scen, &mut Host, u32)); 3] = [
            ("bit one of flags", dtor_wrong_bit1),
            ("no base destructor", dtor_wrong_no_base),
            ("free before destructor", dtor_wrong_order),
        ];
        for (name, f) in wrong {
            let caught = DTOR_FLAGS
                .iter()
                .filter(|flags| dtor_32(**flags) != dtor_lifted(**flags, f))
                .count();
            eprintln!(
                "deleting_destructor wrong version ({name}): caught in {caught} of {} trials",
                DTOR_FLAGS.len()
            );
            assert!(caught > 0, "wrong destructor ({name}) was not caught");
        }
    }

    // ---- clone factory (verified rewrite: the scenario's second virtual method) ----

    fn clone_trials() -> Vec<(u32, u32, u32, bool, u32)> {
        let mut out = Vec::new();
        for aux_id in [0u32, 1, 0x8000_0000, 0x7FFF_FFFF, 0xFFFF_FFFF] {
            for aux_word in [0u32, 0x0BAD_F00D, 0xFFFF_FFFE] {
                for guard in [0u32, 7] {
                    for alloc_ok in [false, true] {
                        for ctor_ret in [0u32, 0x4000_0001, 0xDEAD_BEEF] {
                            out.push((aux_id, aux_word, guard, alloc_ok, ctor_ret));
                        }
                    }
                }
            }
        }
        out
    }

    fn clone_32(aux_id: u32, aux_word: u32, guard: u32, alloc_ok: bool, ctor_ret: u32) -> Outcome {
        let fx = fixture();
        let mut obj = Obj([0u8; 64]);
        put32(&mut obj.0, 0x14, aux_id);
        put32(&mut obj.0, 0x18, aux_word);
        put32(&mut obj.0, 0x1C, guard);
        let before = obj;
        let addr = obj.0.as_ptr() as usize as u32;
        set_syms(&[(addr, "task"), (fx.heap_obj, "heap"), (fx.new_obj, "new")]);
        ALLOC_RET.store(if alloc_ok { fx.new_obj } else { 0 }, Ordering::SeqCst);
        CTOR_RET.store(ctor_ret, Ordering::SeqCst);
        set_callee(1, alloc_stub as *const () as usize as u32);
        set_callee(2, ctor_stub as *const () as usize as u32);
        let r = fn_00c7b220::rw_00c7b220(addr);
        Outcome {
            ret: u64::from(r),
            calls: take_log(),
            effects: vec![Arg::Val(u32::from(obj.0 == before.0))],
        }
    }

    fn clone_lifted(
        aux_id: u32,
        aux_word: u32,
        guard: u32,
        alloc_ok: bool,
        ctor_ret: u32,
        f: fn(&Scen, &mut Host) -> u32,
    ) -> Outcome {
        let log = lifted_log();
        let mut host = Host {
            log: log.clone(),
            alloc_ret: if alloc_ok { NEW_TOKEN } else { 0 },
            ctor_ret,
        };
        let s = Scen {
            aux_id,
            aux_word,
            pending_guard: guard,
            ..Scen::default()
        };
        let r = f(&s, &mut host);
        Outcome {
            ret: u64::from(r),
            calls: drain(&log),
            effects: vec![Arg::Val(1)],
        }
    }

    fn clone_wrong_alloc_sense(s: &Scen, h: &mut Host) -> u32 {
        let heap = h.heap_singleton();
        let slot = h.heap_alloc(heap);
        if slot != 0 {
            return 0;
        }
        h.construct(slot, s.aux_id, s.aux_word, s.pending_guard)
    }

    fn clone_wrong_arg_order(s: &Scen, h: &mut Host) -> u32 {
        let heap = h.heap_singleton();
        let slot = h.heap_alloc(heap);
        if slot == 0 {
            return 0;
        }
        h.construct(slot, s.aux_word, s.aux_id, s.pending_guard)
    }

    fn clone_wrong_return(s: &Scen, h: &mut Host) -> u32 {
        let heap = h.heap_singleton();
        let slot = h.heap_alloc(heap);
        if slot == 0 {
            return 0;
        }
        let _ = h.construct(slot, s.aux_id, s.aux_word, s.pending_guard);
        slot
    }

    #[test]
    fn clone_scenario_matches_rewrite() {
        let _guard = serial();
        let _ = fixture();
        let (mut failed_alloc, mut constructed) = (0, 0);
        for (aux_id, aux_word, guard, alloc_ok, ctor_ret) in clone_trials() {
            let want = clone_32(aux_id, aux_word, guard, alloc_ok, ctor_ret);
            let got = clone_lifted(
                aux_id,
                aux_word,
                guard,
                alloc_ok,
                ctor_ret,
                Scen::clone_scenario::<Host>,
            );
            if want.calls.iter().any(|c| c.name == "construct") {
                constructed += 1
            } else {
                failed_alloc += 1
            }
            assert_eq!(
                got, want,
                "clone {aux_id:#x} {aux_word:#x} {guard:#x} {alloc_ok} {ctor_ret:#x}"
            );
        }
        eprintln!("clone_scenario: {constructed} constructed, {failed_alloc} allocation failures");
        assert!(constructed > 0 && failed_alloc > 0);
    }

    #[test]
    fn clone_scenario_wrong_versions_are_caught() {
        let _guard = serial();
        let _ = fixture();
        let wrong: [(&str, fn(&Scen, &mut Host) -> u32); 3] = [
            ("allocation sense inverted", clone_wrong_alloc_sense),
            ("constructor words swapped", clone_wrong_arg_order),
            ("slot returned, not constructor word", clone_wrong_return),
        ];
        for (name, f) in wrong {
            let caught = clone_trials()
                .into_iter()
                .filter(|(a, b, g, ok, c)| {
                    clone_32(*a, *b, *g, *ok, *c) != clone_lifted(*a, *b, *g, *ok, *c, f)
                })
                .count();
            eprintln!(
                "clone_scenario wrong version ({name}): caught in {caught} of {} trials",
                clone_trials().len()
            );
            assert!(caught > 0, "wrong clone ({name}) was not caught");
        }
    }

    // ---- run tick (verified rewrite: the scenario's sixth virtual method) ----

    /// One run-tick case: sub-task latch, run answer, scenario state, payload present, run arguments.
    type TickCase = (u8, u32, u32, bool, (u32, u32, u32));

    fn tick_trials() -> Vec<TickCase> {
        let mut out = Vec::new();
        let args = [(1u32, 2u32, 3u32), (0x8000_0001, 0xFFFF_FFFF, 0)];
        for latch in [0u8, 1, 2, 3, 0x80, 0x81] {
            for run_ret in [0u32, 0x100, 1, 0xFF, 0x1234_5600] {
                for state in [0u32, 1, 2, 0xFFFF_FFFF] {
                    for payload in [false, true] {
                        for a in args {
                            out.push((latch, run_ret, state, payload, a));
                        }
                    }
                }
            }
        }
        out
    }

    fn tick_32(case: TickCase) -> Outcome {
        let (latch, run_ret, state, payload_on, (a1, a2, a3)) = case;
        let fx = fixture();
        let mut sub = SubObj([0u8; 16]);
        put32(&mut sub.0, 0, fx.sub_vt);
        sub.0[0x0C] = latch;
        let sub_before = sub;
        let sub_addr = sub.0.as_ptr() as usize as u32;
        let mut obj = Obj([0u8; 64]);
        put32(&mut obj.0, 0x08, sub_addr);
        put32(&mut obj.0, 0x28, state);
        put32(
            &mut obj.0,
            0x2C,
            if payload_on { fx.payload_obj } else { 0 },
        );
        let obj_before = obj;
        let addr = obj.0.as_ptr() as usize as u32;
        set_syms(&[
            (addr, "task"),
            (sub_addr, "sub"),
            (fx.payload_obj, "payload"),
        ]);
        RUN_RET.store(run_ret, Ordering::SeqCst);
        set_callee(2, release_stub as *const () as usize as u32);
        let r = fn_00c819e0::rw_00c819e0(addr, a1, a2, a3);
        let obj_words_ok =
            obj.0[..0x28] == obj_before.0[..0x28] && obj.0[0x30..] == obj_before.0[0x30..];
        let sub_ok = sub.0[..0x0C] == sub_before.0[..0x0C] && sub.0[0x0D..] == sub_before.0[0x0D..];
        Outcome {
            ret: u64::from(r),
            calls: take_log(),
            effects: vec![
                Arg::Val(u32::from(sub.0[0x0C])),
                Arg::Val(get32(&obj.0, 0x28)),
                sym_of(get32(&obj.0, 0x2C)),
                Arg::Val(u32::from(obj_words_ok)),
                Arg::Val(u32::from(sub_ok)),
            ],
        }
    }

    fn tick_lifted(
        case: TickCase,
        f: fn(&mut Scen, &mut FakeSub, &mut Host, u32, u32, u32) -> bool,
    ) -> Outcome {
        let (latch, run_ret, state, payload_on, (a1, a2, a3)) = case;
        let log = lifted_log();
        let mut s = Scen {
            state,
            payload: if payload_on { PAYLOAD_TOKEN } else { 0 },
            ..Scen::default()
        };
        let mut sub = FakeSub {
            latch,
            run_ret,
            log: log.clone(),
        };
        let mut host = Host {
            log: log.clone(),
            alloc_ret: 0,
            ctor_ret: 0,
        };
        let ok = f(&mut s, &mut sub, &mut host, a1, a2, a3);
        let payload_effect = if s.payload == 0 {
            Arg::Val(0)
        } else {
            tok(s.payload)
        };
        Outcome {
            ret: u64::from(ok),
            calls: drain(&log),
            effects: vec![
                Arg::Val(u32::from(sub.latch)),
                Arg::Val(s.state),
                payload_effect,
                Arg::Val(1),
                Arg::Val(1),
            ],
        }
    }

    fn tick_wrong_gate_bit1(
        s: &mut Scen,
        sub: &mut FakeSub,
        h: &mut Host,
        a1: u32,
        a2: u32,
        a3: u32,
    ) -> bool {
        if sub.latch() & 2 == 0 {
            if sub.run(a1, a2, a3) & 0xff == 0 {
                return false;
            }
            let latch = sub.latch();
            sub.set_latch(latch | 2);
        }
        if s.state == 1 {
            s.state = 0;
            let payload = s.payload;
            if payload != 0 {
                h.release_payload(payload);
            }
            s.payload = 0;
        }
        true
    }

    fn tick_wrong_latch_bit0(
        s: &mut Scen,
        sub: &mut FakeSub,
        h: &mut Host,
        a1: u32,
        a2: u32,
        a3: u32,
    ) -> bool {
        if sub.latch() & 1 == 0 {
            if sub.run(a1, a2, a3) & 0xff == 0 {
                return false;
            }
            let latch = sub.latch();
            sub.set_latch(latch | 1);
        }
        if s.state == 1 {
            s.state = 0;
            let payload = s.payload;
            if payload != 0 {
                h.release_payload(payload);
            }
            s.payload = 0;
        }
        true
    }

    fn tick_wrong_no_reset(
        s: &mut Scen,
        sub: &mut FakeSub,
        h: &mut Host,
        a1: u32,
        a2: u32,
        a3: u32,
    ) -> bool {
        if sub.latch() & 1 == 0 {
            if sub.run(a1, a2, a3) & 0xff == 0 {
                return false;
            }
            let latch = sub.latch();
            sub.set_latch(latch | 2);
        }
        if s.state == 1 {
            let payload = s.payload;
            if payload != 0 {
                h.release_payload(payload);
            }
            s.payload = 0;
        }
        true
    }

    fn tick_wrong_args(
        s: &mut Scen,
        sub: &mut FakeSub,
        h: &mut Host,
        a1: u32,
        a2: u32,
        a3: u32,
    ) -> bool {
        if sub.latch() & 1 == 0 {
            if sub.run(a2, a1, a3) & 0xff == 0 {
                return false;
            }
            let latch = sub.latch();
            sub.set_latch(latch | 2);
        }
        if s.state == 1 {
            s.state = 0;
            let payload = s.payload;
            if payload != 0 {
                h.release_payload(payload);
            }
            s.payload = 0;
        }
        true
    }

    fn tick_wrong_keeps_payload(
        s: &mut Scen,
        sub: &mut FakeSub,
        h: &mut Host,
        a1: u32,
        a2: u32,
        a3: u32,
    ) -> bool {
        if sub.latch() & 1 == 0 {
            if sub.run(a1, a2, a3) & 0xff == 0 {
                return false;
            }
            let latch = sub.latch();
            sub.set_latch(latch | 2);
        }
        if s.state == 1 {
            s.state = 0;
            let payload = s.payload;
            if payload != 0 {
                h.release_payload(payload);
            }
        }
        true
    }

    #[test]
    fn run_tick_matches_rewrite() {
        let _guard = serial();
        let _ = fixture();
        // Coverage of the rewrite's paths: a completed run, a failed run, a release, and no release.
        let (mut completed, mut failed, mut released, mut not_released) = (0, 0, 0, 0);
        for case in tick_trials() {
            let want = tick_32(case);
            let got = tick_lifted(case, Scen::run_tick::<FakeSub, Host>);
            if want.ret == 1 {
                completed += 1
            } else {
                failed += 1
            }
            if want.calls.iter().any(|c| c.name == "release") {
                released += 1
            } else {
                not_released += 1
            }
            assert_eq!(
                got,
                want,
                "run tick case {:?}",
                (case.0, case.1, case.2, case.3)
            );
        }
        eprintln!(
            "run_tick: {completed} completed, {failed} failed, {released} with release, {not_released} without"
        );
        assert!(completed > 0 && failed > 0 && released > 0 && not_released > 0);
    }

    #[test]
    fn run_tick_wrong_versions_are_caught() {
        let _guard = serial();
        let _ = fixture();
        let wrong: [(
            &str,
            fn(&mut Scen, &mut FakeSub, &mut Host, u32, u32, u32) -> bool,
        ); 5] = [
            ("gate on bit one of latch", tick_wrong_gate_bit1),
            (
                "latch bit zero set instead of bit one",
                tick_wrong_latch_bit0,
            ),
            ("state not reset", tick_wrong_no_reset),
            ("run arguments swapped", tick_wrong_args),
            ("payload not cleared", tick_wrong_keeps_payload),
        ];
        for (name, f) in wrong {
            let caught = tick_trials()
                .into_iter()
                .filter(|case| tick_32(*case) != tick_lifted(*case, f))
                .count();
            eprintln!(
                "run_tick wrong version ({name}): caught in {caught} of {} trials",
                tick_trials().len()
            );
            assert!(caught > 0, "wrong run tick ({name}) was not caught");
        }
    }
}
