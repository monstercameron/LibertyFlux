//! Differential cases, part 2: the facing reaction code.
//!
//! Each case builds a real 32-bit task object and ped, runs the rewrite
//! and the lifted method on the same inputs, and compares the answer and
//! the state-probe call. A deliberately wrong lift (the mid window
//! answers the near pair) must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_peds_tasks::peds_task::{
        CODE_FAR_HI, CODE_FAR_LO, CODE_NEAR_HI, CODE_NEAR_LO, FacingQuery, ReactTuning,
    };
    use lf_pedstaskdiff::rewrites::fn_00BE4F60::rw_00be4f60;
    use lf_pedstaskdiff::set_callee;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        C_FAR, C_LOWER, C_MID, C_NEAR, Rng, ST_SRC_DWORD, addr, cell_u32, lock, put_u32,
        set_cell_u32, set_cell_u64,
    };

    const STATE_CALLEE: u32 = 1;

    const TASK_INNER: usize = 0x14;
    const INNER_MATRIX: usize = 0x20;
    const INLINE_POS: usize = 0x10;
    const MATRIX_POS: usize = 0x30;
    const PED_MATRIX: usize = 0x20;
    const PED_STATE: u32 = 0x2b0;
    const PED_MODE: usize = 0xb80;
    const PED_FLAG: usize = 0x219;
    const PED_COUNTER_PTR: usize = 0x228;
    const COUNTER_VALUE: usize = 0x568;

    static STATE_ANS: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static STATE_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

    extern "thiscall" fn state_stub(this: u32, arg: u32) -> u32 {
        STATE_LOG.lock().unwrap().push((this, arg));
        STATE_ANS.lock().unwrap().pop_front().unwrap()
    }

    fn plant_stubs() {
        set_callee(STATE_CALLEE, support::fn_addr!(state_stub));
    }

    fn reset_scripts(ans: u32) {
        *STATE_ANS.lock().unwrap() = VecDeque::from([ans]);
        STATE_LOG.lock().unwrap().clear();
    }

    struct Case {
        obj: [u32; 3],
        use_matrix: bool,
        dir: [u32; 3],
        ped: [u32; 3],
        mode: u32,
        state_ans: u32,
        flag: u8,
        counter: u32,
        threshold: u32,
        lower: u32,
        far: u32,
        mid: u64,
        near: u32,
    }

    /// The classifier with the mid window answering the near pair.
    fn wrong_code(q: &FacingQuery, state_ans: u32, tuning: &ReactTuning) -> u8 {
        let dy = q.ped[1] - q.obj[1];
        let dx = q.ped[0] - q.obj[0];
        let dz = q.ped[2] - q.obj[2];
        let dist = (q.dir[1] * dy + q.dir[0] * dx) + q.dir[2] * dz;
        let pair = |lo: u8, hi: u8| if q.mode_hi { hi } else { lo };
        if state_ans & 0xff != 0 {
            return pair(CODE_NEAR_LO, CODE_NEAR_HI);
        }
        if q.flag && q.counter > tuning.threshold {
            return pair(CODE_FAR_LO, CODE_FAR_HI);
        }
        if tuning.lower > dist && dist > tuning.far {
            return pair(CODE_NEAR_LO, CODE_NEAR_HI);
        }
        if f64::from(dist) > tuning.mid && tuning.near > dist {
            // Wrong: the near pair instead of the mid pair.
            return pair(CODE_NEAR_LO, CODE_NEAR_HI);
        }
        pair(CODE_FAR_LO, CODE_FAR_HI)
    }

    fn run_case(c: &Case, wrong_caught: &mut u32) {
        // Images: task, inner object, object matrix, ped, ped matrix,
        // counter holder. Fresh boxes per case, so nothing leaks across.
        let mut task = Box::new([0u8; 0x20]);
        let mut inner = Box::new([0u8; 0x24]);
        let mut objmat = Box::new([0u8; 0x3c]);
        let mut ped = Box::new([0u8; 0xb84]);
        let mut pedmat = Box::new([0u8; 0x3c]);
        let mut holder = Box::new([0u8; 0x56c]);
        let (task_a, inner_a, objmat_a) = (addr(task.as_ref()), addr(inner.as_ref()), addr(objmat.as_ref()));
        let (ped_a, pedmat_a, holder_a) = (addr(ped.as_ref()), addr(pedmat.as_ref()), addr(holder.as_ref()));
        put_u32(task.as_mut(), TASK_INNER, inner_a);
        if c.use_matrix {
            put_u32(inner.as_mut(), INNER_MATRIX, objmat_a);
            for (i, v) in c.obj.iter().enumerate() {
                put_u32(objmat.as_mut(), MATRIX_POS + i * 4, *v);
            }
        } else {
            put_u32(inner.as_mut(), INNER_MATRIX, 0);
            for (i, v) in c.obj.iter().enumerate() {
                put_u32(inner.as_mut(), INLINE_POS + i * 4, *v);
            }
        }
        put_u32(ped.as_mut(), PED_MATRIX, pedmat_a);
        for (i, v) in c.dir.iter().enumerate() {
            put_u32(pedmat.as_mut(), i * 4, *v);
        }
        for (i, v) in c.ped.iter().enumerate() {
            put_u32(pedmat.as_mut(), MATRIX_POS + i * 4, *v);
        }
        put_u32(ped.as_mut(), PED_MODE, c.mode);
        ped.as_mut()[PED_FLAG] = c.flag;
        put_u32(ped.as_mut(), PED_COUNTER_PTR, holder_a);
        put_u32(holder.as_mut(), COUNTER_VALUE, c.counter);
        // Classifier constants behind their cells.
        set_cell_u32(ST_SRC_DWORD, c.threshold);
        set_cell_u32(C_LOWER, c.lower);
        set_cell_u32(C_FAR, c.far);
        set_cell_u32(C_NEAR, c.near);
        set_cell_u64(C_MID, c.mid);

        reset_scripts(c.state_ans);
        let ret_rw = unsafe { rw_00be4f60(task_a, ped_a) };
        let log_rw = STATE_LOG.lock().unwrap().clone();
        assert_eq!(log_rw.len(), 1, "state probed exactly once");
        assert_eq!(log_rw[0], (ped_a.wrapping_add(PED_STATE), 1));
        // The cells are read-only to this routine.
        assert_eq!(cell_u32(ST_SRC_DWORD), c.threshold);

        let f3 = |w: &[u32; 3]| w.map(f32::from_bits);
        let query = FacingQuery {
            obj: f3(&c.obj),
            dir: f3(&c.dir),
            ped: f3(&c.ped),
            mode_hi: c.mode == 3 || c.mode == 4,
            flag: c.flag != 0,
            counter: c.counter,
        };
        let tuning = ReactTuning {
            threshold: c.threshold,
            lower: f32::from_bits(c.lower),
            far: f32::from_bits(c.far),
            mid: f64::from_bits(c.mid),
            near: f32::from_bits(c.near),
        };
        let mut probes = 0u32;
        let ret_lift = query.code(
            &mut || {
                probes += 1;
                c.state_ans
            },
            &tuning,
        );
        assert_eq!(probes, 1, "lift probes once");
        assert_eq!(u32::from(ret_lift), ret_rw, "code agrees");

        if wrong_code(&query, c.state_ans, &tuning) as u32 != ret_rw {
            *wrong_caught += 1;
        }
    }

    /// Image classifier bounds: -0.2, -1.2, +1.2 as floats, 0.2 as double.
    const IMG_LOWER: u32 = 0xBE4C_CCCD;
    const IMG_FAR: u32 = 0xBF99_999A;
    const IMG_NEAR: u32 = 0x3F99_999A;
    const IMG_MID: u64 = 0x3FC9_9999_9999_999A;

    fn base_case() -> Case {
        Case {
            obj: [0, 0, 0],
            use_matrix: true,
            dir: [0x3F80_0000, 0, 0],
            ped: [0x3F00_0000, 0, 0],
            mode: 0,
            state_ans: 0,
            flag: 0,
            counter: 0,
            threshold: 0xFFFF_FFFF,
            lower: IMG_LOWER,
            far: IMG_FAR,
            mid: IMG_MID,
            near: IMG_NEAR,
        }
    }

    fn random_case(rng: &mut Rng) -> Case {
        const MODES: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 0x7FFF_FFFF, 0xFFFF_FFFF, 0x8000_0000];
        let mut obj = [0u32; 3];
        let mut dir = [0u32; 3];
        let mut ped = [0u32; 3];
        for v in obj.iter_mut().chain(dir.iter_mut()).chain(ped.iter_mut()) {
            *v = rng.float_bits();
        }
        let threshold = rng.edge_word();
        let counter = match rng.below(6) {
            0 => threshold,
            1 => threshold.wrapping_add(1),
            2 => threshold.wrapping_sub(1),
            3 => 0,
            4 => 0xFFFF_FFFF,
            _ => rng.u32(),
        };
        // Tuning varies per case, so the lift cannot hardcode the image.
        let tuning_img = rng.below(2) == 0;
        Case {
            obj,
            use_matrix: rng.below(2) == 0,
            dir,
            ped,
            mode: if rng.below(2) == 0 {
                MODES[rng.below(10) as usize]
            } else {
                rng.u32()
            },
            state_ans: match rng.below(4) {
                0 => 0,
                1 => rng.below(256),
                2 => rng.u32() & 0xFFFF_FF00,
                _ => rng.u32(),
            },
            flag: if rng.below(2) == 0 {
                [0, 1, 0xFF][rng.below(3) as usize] as u8
            } else {
                rng.u32() as u8
            },
            counter,
            threshold,
            lower: if tuning_img { IMG_LOWER } else { rng.float_bits() },
            far: if tuning_img { IMG_FAR } else { rng.float_bits() },
            mid: if tuning_img {
                IMG_MID
            } else {
                (u64::from(rng.u32()) << 32) | u64::from(rng.u32())
            },
            near: if tuning_img { IMG_NEAR } else { rng.float_bits() },
        }
    }

    #[test]
    fn react_code_matches_rewrite() {
        let _held = lock();
        plant_stubs();
        let mut wrong_caught = 0u32;
        let mut compared = 0u32;
        let mut one = |c: &Case| {
            run_case(c, &mut wrong_caught);
            compared += 1;
        };
        // Dot 0.5 through the matrix layout: the mid window, which the
        // mutant answers with the near pair: the mutant catcher.
        one(&base_case());
        // Same through the inline layout.
        let mut c = base_case();
        c.use_matrix = false;
        one(&c);
        // Near window: dot -0.5.
        let mut c = base_case();
        c.ped = [0xBF00_0000, 0, 0];
        one(&c);
        // Far by distance: dot 2.0.
        let mut c = base_case();
        c.ped = [0x4000_0000, 0, 0];
        one(&c);
        // NaN dot answers far.
        let mut c = base_case();
        c.ped = [0x7FC0_0000, 0, 0];
        one(&c);
        // State probe nonzero answers near even for a far dot.
        let mut c = base_case();
        c.ped = [0x4000_0000, 0, 0];
        c.state_ans = 1;
        one(&c);
        // Only the low byte steers: 0x100 proceeds.
        let mut c = base_case();
        c.state_ans = 0x100;
        one(&c);
        // Counter gate: set flag, counter above threshold answers far
        // even for a near dot.
        let mut c = base_case();
        c.ped = [0xBF00_0000, 0, 0];
        c.flag = 1;
        c.threshold = 10;
        c.counter = 11;
        one(&c);
        // Counter equal to the threshold proceeds (unsigned above).
        let mut c = base_case();
        c.flag = 0xFF;
        c.threshold = 10;
        c.counter = 10;
        one(&c);
        // Window edges are exclusive: dot exactly -0.2 is not near.
        let mut c = base_case();
        c.dir = [0xBF80_0000, 0, 0];
        c.ped = [0x3E4C_CCCD, 0, 0];
        one(&c);
        // Dot exactly 0.2f converts above the 0.2 double: mid.
        let mut c = base_case();
        c.ped = [0x3E4C_CCCD, 0, 0];
        one(&c);
        // Modes 3 and 4 take the high member, 5 the low one.
        for mode in [3, 4, 5] {
            let mut c = base_case();
            c.mode = mode;
            one(&c);
        }
        let mut rng = Rng(0xBE4F_6071);
        for _ in 0..240 {
            let c = random_case(&mut rng);
            one(&c);
        }
        assert!(wrong_caught > 0, "wrong lift was never caught");
        assert!(compared >= 240, "ran {compared} comparisons");
    }
}
