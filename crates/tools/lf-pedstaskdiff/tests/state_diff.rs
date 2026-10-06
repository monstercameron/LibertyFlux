//! Differential cases, part 3: the shared ped-task state block.
//!
//! Each case plants real 32-bit objects and the state cells, runs the
//! rewrite and the lifted method on the same inputs, and compares the
//! answer, every state word and the callee call logs. Two deliberately
//! wrong lifts must be caught: unhalved build midpoints, and a widened
//! consume gate. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_core::boundary::Handle32;
    use lf_peds_tasks::peds_task::{
        GATE_LIMIT, ObjTag, TaskFloats, TaskStamp, TaskStateBlock, TaskTarget, TaskVec,
    };
    use lf_pedstaskdiff::rewrites::fn_00CB7CF0::rw_00cb7cf0;
    use lf_pedstaskdiff::rewrites::fn_00CB8020::rw_00cb8020;
    use lf_pedstaskdiff::{relocated, set_callee, set_relocated};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        CONSUMER_VA, Rng, ST_A, ST_A_PAD, ST_B, ST_EXTRA, ST_FLAG, ST_GATE_BIAS, ST_OUT_BIAS,
        ST_RANGE_HI, ST_RAW, ST_SRC_DWORD, addr, cell_u8, cell_u32, lock, put_u32, set_cell_u8,
        set_cell_u32,
    };

    const ENUM_CALLEE: u32 = 1;
    const WORKER_CALLEE: u32 = 2;
    const CHECK_CALLEE: u32 = 3;
    const COOKIE_CALLEE: u32 = 4;

    const RADIUS_BITS: u32 = 0x41A0_0000; // 20.0
    const QUARTER_BITS: u32 = 0x3E80_0000; // 0.25

    static ENUM_LOG: Mutex<Vec<([u32; 5], u32, u32, u32, u32)>> = Mutex::new(Vec::new());
    static ENUM_REENTER: Mutex<bool> = Mutex::new(false);
    static VTASK_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static VTASK_ANS: Mutex<VecDeque<[u32; 5]>> = Mutex::new(VecDeque::new());
    static WORKER_LOG: Mutex<Vec<[u32; 10]>> = Mutex::new(Vec::new());
    static CHECK_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static CHECK_ANS: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static COOKIE_CALLS: Mutex<u32> = Mutex::new(0);

    extern "cdecl" fn enum_stub(block: u32, consumer: u32, z: u32, four: u32, five: u32) -> u32 {
        // The block lives on the rewrite's stack: read it before returning.
        let words = unsafe { (block as *const [u32; 5]).read_unaligned() };
        ENUM_LOG
            .lock()
            .unwrap()
            .push((words, consumer, z, four, five));
        if *ENUM_REENTER.lock().unwrap() {
            unsafe {
                lf_pedstaskdiff::global::<u8>(ST_FLAG).write(1);
            }
        }
        0
    }

    extern "thiscall" fn vtask_stub(obj: u32, out: u32) -> u32 {
        VTASK_LOG.lock().unwrap().push((obj, out));
        let ans = VTASK_ANS.lock().unwrap().pop_front().unwrap();
        unsafe {
            (out as *mut [u32; 5]).write_unaligned(ans);
        }
        0
    }

    #[allow(clippy::too_many_arguments)]
    extern "thiscall" fn worker_stub(
        ecx: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
    ) -> u32 {
        WORKER_LOG
            .lock()
            .unwrap()
            .push([ecx, a0, a1, a2, a3, a4, a5, a6, a7, a8]);
        0
    }

    extern "stdcall" fn check_stub(a0: u32, a1: u32) -> u32 {
        CHECK_LOG.lock().unwrap().push((a0, a1));
        CHECK_ANS.lock().unwrap().pop_front().unwrap()
    }

    extern "cdecl" fn cookie_stub() -> u32 {
        *COOKIE_CALLS.lock().unwrap() += 1;
        0
    }

    fn plant_stubs() {
        set_callee(ENUM_CALLEE, support::fn_addr!(enum_stub));
        set_callee(WORKER_CALLEE, support::fn_addr!(worker_stub));
        set_callee(CHECK_CALLEE, support::fn_addr!(check_stub));
        set_callee(COOKIE_CALLEE, support::fn_addr!(cookie_stub));
        set_relocated(CONSUMER_VA, support::fn_addr!(rw_00cb7cf0));
    }

    fn reset_build_scripts(reenter: bool) {
        ENUM_LOG.lock().unwrap().clear();
        *ENUM_REENTER.lock().unwrap() = reenter;
        *COOKIE_CALLS.lock().unwrap() = 0;
    }

    fn reset_consume_scripts(vtask: [u32; 5], check: u32) {
        VTASK_LOG.lock().unwrap().clear();
        *VTASK_ANS.lock().unwrap() = VecDeque::from([vtask]);
        WORKER_LOG.lock().unwrap().clear();
        CHECK_LOG.lock().unwrap().clear();
        *CHECK_ANS.lock().unwrap() = VecDeque::from([check]);
        *COOKIE_CALLS.lock().unwrap() = 0;
    }

    /// Snapshots the modelled state words as raw bits.
    struct Snap {
        flag: u8,
        out_bias: u32,
        gate_bias: u32,
        range_hi: u32,
        gate: [u32; 3],
        work: [u32; 3],
        raw: [u32; 8],
    }

    fn snapshot() -> Snap {
        Snap {
            flag: cell_u8(ST_FLAG),
            out_bias: cell_u32(ST_OUT_BIAS),
            gate_bias: cell_u32(ST_GATE_BIAS),
            range_hi: cell_u32(ST_RANGE_HI),
            gate: ST_A.map(cell_u32),
            work: ST_B.map(cell_u32),
            raw: ST_RAW.map(cell_u32),
        }
    }

    fn lift_snap(s: &TaskStateBlock) -> Snap {
        Snap {
            flag: u8::from(s.flag),
            out_bias: s.out_bias.to_bits(),
            gate_bias: s.gate_bias.to_bits(),
            range_hi: s.range_hi.to_bits(),
            gate: s.gate.map(f32::to_bits),
            work: s.work.map(f32::to_bits),
            raw: s.raw.map(f32::to_bits),
        }
    }

    fn assert_snap_eq(a: &Snap, b: &Snap, what: &str) {
        assert_eq!(a.flag, b.flag, "{what}: flag");
        assert_eq!(a.out_bias, b.out_bias, "{what}: out bias {:#x}", a.out_bias);
        assert_eq!(a.gate_bias, b.gate_bias, "{what}: gate bias");
        assert_eq!(a.range_hi, b.range_hi, "{what}: range");
        assert_eq!(a.gate, b.gate, "{what}: gate vector");
        assert_eq!(a.work, b.work, "{what}: work vector");
        assert_eq!(a.raw, b.raw, "{what}: raw floats");
    }

    fn plant_state(rng: &mut Rng) -> TaskStateBlock {
        let f = |rng: &mut Rng| f32::from_bits(rng.float_bits());
        let st = TaskStateBlock {
            flag: rng.below(2) == 0,
            out_bias: f(rng),
            gate_bias: f(rng),
            range_hi: f(rng),
            gate: [f(rng), f(rng), f(rng)],
            work: [f(rng), f(rng), f(rng)],
            raw: [
                f(rng),
                f(rng),
                f(rng),
                f(rng),
                f(rng),
                f(rng),
                f(rng),
                f(rng),
            ],
            copy_source: rng.edge_word(),
        };
        set_cell_u8(ST_FLAG, u8::from(st.flag));
        set_cell_u32(ST_OUT_BIAS, st.out_bias.to_bits());
        set_cell_u32(ST_GATE_BIAS, st.gate_bias.to_bits());
        set_cell_u32(ST_RANGE_HI, st.range_hi.to_bits());
        for (i, va) in ST_A.iter().enumerate() {
            set_cell_u32(*va, st.gate[i].to_bits());
        }
        for (i, va) in ST_B.iter().enumerate() {
            set_cell_u32(*va, st.work[i].to_bits());
        }
        for (i, va) in ST_RAW.iter().enumerate() {
            set_cell_u32(*va, st.raw[i].to_bits());
        }
        set_cell_u32(ST_A_PAD, rng.u32());
        set_cell_u32(ST_EXTRA, rng.u32());
        set_cell_u32(ST_SRC_DWORD, st.copy_source);
        st
    }

    // Wrong implementation of the build with unhalved midpoints: the
    // enumeration receives (a+b, c+e, f+d) instead of the midpoints.
    #[allow(clippy::too_many_lines)]
    fn wrong_build(
        state: &mut TaskStateBlock,
        floats: &TaskFloats,
        stamp: &mut TaskStamp,
        reenter: bool,
    ) -> (bool, [f32; 3]) {
        let (a, c, f) = (floats.a, floats.c, floats.f);
        let (b, e, d) = (floats.b, floats.e, floats.d);
        // Wrong: no halving.
        let mid = [a + b, c + e, f + d];
        let d0 = b - a;
        let d1 = e - c;
        let d2 = d - f;
        let sumsq = (d1 * d1 + d0 * d0) + d2 * d2;
        let mult = if sumsq == 0.0 {
            0.0
        } else {
            1.0f32 / sumsq.sqrt()
        };
        let (s0, s1, s2) = (d0 * mult, d1 * mult, d2 * mult);
        let t2 = s0 * 0.0 - s1 * 0.0;
        let t0 = s1 - s2 * 0.0;
        let t1 = s2 * 0.0 - s0;
        state.raw = [a, c, f, floats.raw4, b, e, d, floats.raw7];
        state.gate = [s0, s1, s2];
        state.gate_bias = f32::from_bits(((c * s1 + a * s0) + f * s2).to_bits() ^ 0x8000_0000);
        state.work = [t0, t1, t2];
        state.range_hi = ((e - c) * (e - c) + (b - a) * (b - a)).sqrt();
        state.flag = false;
        state.out_bias = f32::from_bits(((c * t1 + a * t0) + f * t2).to_bits() ^ 0x8000_0000);
        if reenter {
            state.flag = true;
        }
        stamp.dword = state.copy_source;
        stamp.id = 2000;
        stamp.flag = 1;
        (!state.flag, mid)
    }

    const F_OFF: [usize; 8] = [0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c];
    const STAMP_DWORD: usize = 0x2c;
    const STAMP_ID: usize = 0x30;
    const STAMP_FLAG: usize = 0x34;

    fn run_build_case(floats: &[u32; 8], reenter: bool, wrong_caught: &mut u32) {
        let mut task = Box::new([0u8; 0x60]);
        for (i, v) in floats.iter().enumerate() {
            put_u32(task.as_mut(), F_OFF[i], *v);
        }
        // Stamp area starts nonzero so the writes are observed.
        put_u32(task.as_mut(), STAMP_DWORD, 0x1111_1111);
        put_u32(task.as_mut(), STAMP_ID, 0x2222_2222);
        task.as_mut()[STAMP_FLAG] = 0x33;
        let task_a = addr(task.as_ref());

        let mut rng = Rng(floats[0].wrapping_add(u32::from(reenter)));
        let planted = plant_state(&mut rng);
        reset_build_scripts(reenter);
        let ret_rw = unsafe { rw_00cb8020(task_a, 0) };
        let snap_rw = snapshot();
        let stamp_rw = (
            unsafe { (task.as_ptr().add(STAMP_DWORD) as *const u32).read_unaligned() },
            unsafe { (task.as_ptr().add(STAMP_ID) as *const u32).read_unaligned() },
            unsafe { *task.as_ptr().add(STAMP_FLAG) },
        );
        let enum_rw = ENUM_LOG.lock().unwrap().clone();
        assert_eq!(enum_rw.len(), 1, "enumeration runs once");
        // The two scratch words read back zero.
        assert_eq!(cell_u32(ST_A_PAD), 0, "pad word pinned to zero");
        assert_eq!(cell_u32(ST_EXTRA), 0, "extra word pinned to zero");

        let tf = TaskFloats {
            a: f32::from_bits(floats[0]),
            c: f32::from_bits(floats[1]),
            f: f32::from_bits(floats[2]),
            raw4: f32::from_bits(floats[3]),
            b: f32::from_bits(floats[4]),
            e: f32::from_bits(floats[5]),
            d: f32::from_bits(floats[6]),
            raw7: f32::from_bits(floats[7]),
        };
        let mut state = planted.clone();
        let mut stamp = TaskStamp::default();
        let mut mid_lift = [0.0f32; 3];
        let ret_lift = TaskStateBlock::build(
            &mut state,
            &tf,
            &mut stamp,
            &mut |s: &mut TaskStateBlock, mid: [f32; 3]| {
                mid_lift = mid;
                if reenter {
                    s.flag = true;
                }
            },
        );
        assert_eq!(u32::from(ret_lift), ret_rw, "answer agrees");
        assert_snap_eq(&lift_snap(&state), &snap_rw, "state");
        assert_eq!(
            (stamp.dword, stamp.id, stamp.flag),
            stamp_rw,
            "stamp agrees"
        );
        // The enumeration block, read through the stub pointer on the
        // rewrite side, carries the lifted midpoints plus constants.
        let (words, consumer, z, four, five) = enum_rw[0];
        assert_eq!(words[0], mid_lift[0].to_bits(), "mid 0");
        assert_eq!(words[1], mid_lift[1].to_bits(), "mid 1");
        assert_eq!(words[2], mid_lift[2].to_bits(), "mid 2");
        assert_eq!(words[3], 0, "block zero word");
        assert_eq!(words[4], RADIUS_BITS, "block radius word");
        assert_eq!(consumer, support::fn_addr!(rw_00cb7cf0), "consumer address");
        assert_eq!((z, four, five), (0, 4, 5), "trailing words");

        let mut state_w = planted.clone();
        let mut stamp_w = TaskStamp::default();
        let (ret_w, mid_w) = wrong_build(&mut state_w, &tf, &mut stamp_w, reenter);
        if u32::from(ret_w) != ret_rw
            || mid_w.map(f32::to_bits) != [words[0], words[1], words[2]]
            || lift_snap(&state_w).gate != snap_rw.gate
        {
            *wrong_caught += 1;
        }
    }

    #[test]
    fn state_build_matches_rewrite() {
        let _held = lock();
        plant_stubs();
        let mut wrong_caught = 0u32;
        let mut compared = 0u32;
        let mut one = |floats: &[u32; 8], reenter: bool| {
            run_build_case(floats, reenter, &mut wrong_caught);
            compared += 1;
        };
        // All zeros: the squared sum is exactly zero, multiplier 0.
        one(&[0; 8], false);
        // A re-entering enumerator flips the answer.
        one(&[0x3F80_0000; 8], true);
        // Small integers.
        one(
            &[
                0x3F80_0000,
                0x4000_0000,
                0x4040_0000,
                0x4080_0000,
                0x40A0_0000,
                0x40C0_0000,
                0x40E0_0000,
                0x4100_0000,
            ],
            false,
        );
        // Equal pairs: zero differences, zero multiplier.
        one(
            &[
                0x3F80_0000,
                0x4000_0000,
                0x4040_0000,
                0,
                0x3F80_0000,
                0x4000_0000,
                0x4040_0000,
                0,
            ],
            false,
        );
        // NaN, infinities, subnormals, signed zeros.
        one(
            &[
                0x7FC0_0000,
                0x7F80_0000,
                0xFF80_0000,
                0x0000_0001,
                0x8000_0000,
                0x8000_0001,
                0x7F7F_FFFF,
                0xFF7F_FFFF,
            ],
            false,
        );
        one(
            &[
                0x7FC0_0000,
                0x7F80_0000,
                0xFF80_0000,
                0x0000_0001,
                0x8000_0000,
                0x8000_0001,
                0x7F7F_FFFF,
                0xFF7F_FFFF,
            ],
            true,
        );
        let mut rng = Rng(0xCB80_2071);
        for _ in 0..200 {
            let mut floats = [0u32; 8];
            for v in floats.iter_mut() {
                *v = rng.float_bits();
            }
            let reenter = rng.below(4) == 0;
            one(&floats, reenter);
        }
        assert!(wrong_caught > 0, "wrong build was never caught");
        assert!(compared >= 200, "ran {compared} comparisons");
    }

    // The consume with the first gate widened to `>=`: a gap of exactly
    // 4.0 proceeds instead of returning.
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    fn wrong_consume(
        state: &mut TaskStateBlock,
        vec: &[f32; 3],
        o: &[f32; 3],
        check_ans: u32,
    ) -> (u8, bool, u32) {
        let [v0, v1, v2] = *vec;
        let gap = f32::from_bits((v2 - state.raw[2]).to_bits() & 0x7FFF_FFFF);
        let mut calls = 0u32;
        // Wrong: `>=` instead of `>`.
        if GATE_LIMIT >= gap {
            let gate =
                ((state.gate[1] * v1 + state.gate[0] * v0) + state.gate[2] * v2) + state.gate_bias;
            if !(0.0 > gate) && !(gate > state.range_hi) {
                let rout = ((state.work[1] * v1 + state.work[0] * v0) + state.work[2] * v2)
                    + state.out_bias;
                let rwork = (state.work[1] * o[1] + state.work[0] * o[0]) + state.work[2] * o[2];
                calls = 3;
                let ok = check_ans & 0xff;
                let mut raise = false;
                if ok == 0 && 0.5 > rwork {
                    raise = true;
                }
                if !raise {
                    if rout > 0.0 {
                        if -0.5 > rwork {
                            raise = true;
                        }
                    }
                    if !raise && 0.0 > rout && rwork > 0.5 {
                        raise = true;
                    }
                }
                if raise {
                    state.flag = true;
                }
            }
        }
        (1, state.flag, calls)
    }

    struct ConsumeCase {
        vec: [u32; 3],
        vtask: [u32; 5],
        check: u32,
    }

    fn run_consume_case(seed_state: &TaskStateBlock, c: &ConsumeCase, wrong_caught: &mut u32) {
        // Plant the seeded state over the cells.
        set_cell_u8(ST_FLAG, u8::from(seed_state.flag));
        set_cell_u32(ST_OUT_BIAS, seed_state.out_bias.to_bits());
        set_cell_u32(ST_GATE_BIAS, seed_state.gate_bias.to_bits());
        set_cell_u32(ST_RANGE_HI, seed_state.range_hi.to_bits());
        for (i, va) in ST_A.iter().enumerate() {
            set_cell_u32(*va, seed_state.gate[i].to_bits());
        }
        for (i, va) in ST_B.iter().enumerate() {
            set_cell_u32(*va, seed_state.work[i].to_bits());
        }
        for (i, va) in ST_RAW.iter().enumerate() {
            set_cell_u32(*va, seed_state.raw[i].to_bits());
        }
        // Object: vtable pointer, vector-record pointer; the vtable
        // carries the task stub address at its slot.
        let mut obj = Box::new([0u8; 0x24]);
        let mut vtable = Box::new([0u8; 0xf0]);
        let mut record = Box::new([0u8; 0x3c]);
        let (obj_a, vt_a, rec_a) = (
            addr(obj.as_ref()),
            addr(vtable.as_ref()),
            addr(record.as_ref()),
        );
        put_u32(obj.as_mut(), 0x00, vt_a);
        put_u32(obj.as_mut(), 0x20, rec_a);
        put_u32(vtable.as_mut(), 0xec, support::fn_addr!(vtask_stub));
        for (i, v) in c.vec.iter().enumerate() {
            put_u32(record.as_mut(), 0x30 + i * 4, *v);
        }

        reset_consume_scripts(c.vtask, c.check);
        let ret_rw = unsafe { rw_00cb7cf0(obj_a) };
        let snap_rw = snapshot();
        let vtask_rw = VTASK_LOG.lock().unwrap().clone();
        let worker_rw = WORKER_LOG.lock().unwrap().clone();
        let check_rw = CHECK_LOG.lock().unwrap().clone();
        let cookie_rw = *COOKIE_CALLS.lock().unwrap();
        assert_eq!(ret_rw & 0xff, 1, "low byte always 1");
        assert_eq!(cookie_rw, 1, "cookie check runs on every exit");
        assert!(vtask_rw.len() <= 1 && worker_rw.len() <= 1 && check_rw.len() <= 1);
        assert_eq!(vtask_rw.len(), worker_rw.len(), "vtask/worker pair up");
        assert_eq!(worker_rw.len(), check_rw.len(), "worker/check pair up");
        // Pinned call shapes on the rewrite side.
        for (obj_seen, _out) in &vtask_rw {
            assert_eq!(*obj_seen, obj_a, "vtask object");
        }
        for w in &worker_rw {
            assert_eq!(w[1], obj_a, "worker object");
            assert_eq!(w[2], c.vec[2], "worker v2 word");
            assert_eq!(w[3], QUARTER_BITS, "worker quarter word");
            assert_eq!(&w[4..], &[0u32, 0, 0, 0, 0, 0], "worker zero words");
        }
        for (a0, a1) in &check_rw {
            assert_eq!(*a0, relocated(ST_RAW[0]), "check first address");
            assert_eq!(*a1, relocated(ST_RAW[4]), "check second address");
        }

        let target = TaskTarget {
            obj: Handle32::<ObjTag>::new(obj_a).unwrap(),
            vec: TaskVec {
                v: c.vec.map(f32::from_bits),
            },
        };
        let mut state = seed_state.clone();
        let mut vtask_n = 0u32;
        let mut worker_seen: Vec<(u32, u32)> = Vec::new();
        let mut check_n = 0u32;
        let ret_lift = state.consume(
            &target,
            &mut |obj: Handle32<ObjTag>| {
                vtask_n += 1;
                assert_eq!(obj.get(), obj_a, "lift vtask object");
                [
                    f32::from_bits(c.vtask[0]),
                    f32::from_bits(c.vtask[1]),
                    f32::from_bits(c.vtask[2]),
                ]
            },
            &mut |obj: Handle32<ObjTag>, v2: f32| {
                worker_seen.push((obj.get(), v2.to_bits()));
            },
            &mut || {
                check_n += 1;
                c.check
            },
        );
        assert_eq!(ret_lift, 1, "lift always answers 1");
        assert_snap_eq(&lift_snap(&state), &snap_rw, "state");
        assert_eq!(vtask_n as usize, vtask_rw.len(), "vtask count agrees");
        assert_eq!(worker_seen.len(), worker_rw.len(), "worker count agrees");
        assert_eq!(check_n as usize, check_rw.len(), "check count agrees");
        for ((obj_seen, v2), w) in worker_seen.iter().zip(worker_rw.iter()) {
            assert_eq!(*obj_seen, obj_a, "lift worker object");
            assert_eq!(*v2, w[2], "lift worker v2");
        }

        let mut state_w = seed_state.clone();
        let o = [
            f32::from_bits(c.vtask[0]),
            f32::from_bits(c.vtask[1]),
            f32::from_bits(c.vtask[2]),
        ];
        let (ret_w, flag_w, calls_w) = wrong_consume(&mut state_w, &target.vec.v, &o, c.check);
        let calls_rw = (vtask_rw.len() + worker_rw.len() + check_rw.len()) as u32;
        if ret_w != 1 || flag_w != (snap_rw.flag != 0) || calls_w != calls_rw {
            *wrong_caught += 1;
        }
    }

    fn seeded(rng: &mut Rng) -> TaskStateBlock {
        plant_state(rng)
    }

    #[test]
    fn state_consume_matches_rewrite() {
        let _held = lock();
        plant_stubs();
        let mut wrong_caught = 0u32;
        let mut compared = 0u32;
        // Gap exactly 4.0 past an open gate: the rewrite makes no calls
        // and the widened mutant makes three: the mutant catcher.
        let mut rng = Rng(0xCB7C_F071);
        let open = TaskStateBlock {
            flag: false,
            out_bias: 0.0,
            gate_bias: 0.0,
            range_hi: 10.0,
            gate: [0.0, 0.0, 0.0],
            work: [0.0, 0.0, 0.0],
            raw: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            copy_source: 0,
        };
        run_consume_case(
            &open,
            &ConsumeCase {
                vec: [0x3F80_0000, 0x4000_0000, 0x40A0_0000],
                vtask: [0, 0, 0, 0xAAAA_AAAA, 0xBBBB_BBBB],
                check: 1,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // Gap just inside 4.0 runs the calls.
        run_consume_case(
            &open,
            &ConsumeCase {
                vec: [0x3F80_0000, 0x4000_0000, 0x409F_FFFF],
                vtask: [0, 0, 0, 0, 0],
                check: 1,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // NaN gate keeps running; low byte of the check steers; flag raises.
        let nan_gate = TaskStateBlock {
            flag: false,
            out_bias: 0.0,
            gate_bias: 0.0,
            range_hi: 10.0,
            gate: [f32::NAN, 0.0, 0.0],
            work: [0.0, 0.0, 0.0],
            raw: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            copy_source: 0,
        };
        run_consume_case(
            &nan_gate,
            &ConsumeCase {
                vec: [0x3F80_0000, 0, 0],
                vtask: [0, 0, 0, 0, 0],
                check: 0x100,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // Tail arm B: positive out-dot, work-dot below -0.5.
        let arm_b = TaskStateBlock {
            flag: false,
            out_bias: 1.0,
            gate_bias: 0.0,
            range_hi: 10.0,
            gate: [0.0, 0.0, 0.0],
            work: [1.0, 0.0, 0.0],
            raw: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            copy_source: 0,
        };
        run_consume_case(
            &arm_b,
            &ConsumeCase {
                vec: [0, 0, 0],
                vtask: [0xBF80_0000, 0, 0, 0, 0],
                check: 1,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // Tail arm C: negative out-dot, work-dot above 0.5.
        let arm_c = TaskStateBlock {
            flag: false,
            out_bias: -1.0,
            gate_bias: 0.0,
            range_hi: 10.0,
            gate: [0.0, 0.0, 0.0],
            work: [1.0, 0.0, 0.0],
            raw: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            copy_source: 0,
        };
        run_consume_case(
            &arm_c,
            &ConsumeCase {
                vec: [0, 0, 0],
                vtask: [0x3F80_0000, 0, 0, 0, 0],
                check: 0xFFFF_FFFF,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // Zero out-dot with a nonzero check: no arm fires.
        run_consume_case(
            &open,
            &ConsumeCase {
                vec: [0, 0, 0],
                vtask: [0, 0, 0, 0, 0],
                check: 7,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // NaN third component: the first gate fails, no calls.
        run_consume_case(
            &open,
            &ConsumeCase {
                vec: [0, 0, 0x7FC0_0000],
                vtask: [0, 0, 0, 0, 0],
                check: 0,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // A set flag survives a quiet case.
        let mut flagged = open.clone();
        flagged.flag = true;
        run_consume_case(
            &flagged,
            &ConsumeCase {
                vec: [0, 0, 0x7FC0_0000],
                vtask: [0, 0, 0, 0, 0],
                check: 0,
            },
            &mut wrong_caught,
        );
        compared += 1;
        for _ in 0..200 {
            let st = seeded(&mut rng);
            let c = ConsumeCase {
                vec: [rng.float_bits(), rng.float_bits(), rng.float_bits()],
                vtask: [
                    rng.float_bits(),
                    rng.float_bits(),
                    rng.float_bits(),
                    rng.u32(),
                    rng.u32(),
                ],
                check: rng.u32(),
            };
            run_consume_case(&st, &c, &mut wrong_caught);
            compared += 1;
        }
        assert!(wrong_caught > 0, "wrong consume was never caught");
        assert!(compared >= 200, "ran {compared} comparisons");
    }
}
