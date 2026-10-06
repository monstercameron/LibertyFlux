//! Differential cases, part 1: the weighted candidate picker.
//!
//! Each case builds a real 32-bit picker object, runs the rewrite and the
//! lifted method on the same inputs, and compares the answer, every slot
//! word and the callee call logs (the lifted index is rebuilt to the two
//! stub addresses and compared too). A deliberately wrong lift (first
//! prefix at or above the threshold wins, instead of strictly above) must
//! be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_peds_tasks::peds_task::{EMPTY, MAX_ENTRIES, WeightedPicker};
    use lf_pedstaskdiff::rewrites::fn_00D44AB0::rw_00D44AB0;
    use lf_pedstaskdiff::set_callee;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, get_u32, lock, put_u32};

    const RAND_CALLEE: u32 = 1;
    const COOKIE_CALLEE: u32 = 2;
    const FILL_CALLEE: u32 = 3;

    const SLOTS_OFF: usize = 0x200;
    const WEIGHTS_OFF: usize = 0x240;
    const COUNT_OFF: usize = 0x280;
    const IMG_LEN: usize = 0x284;

    static RAND_ANS: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static RAND_CALLS: Mutex<u32> = Mutex::new(0);
    static FILL_ANS: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static FILL_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static COOKIE_CALLS: Mutex<u32> = Mutex::new(0);

    extern "cdecl" fn rand_stub() -> u32 {
        *RAND_CALLS.lock().unwrap() += 1;
        RAND_ANS.lock().unwrap().pop_front().unwrap()
    }

    extern "cdecl" fn fill_stub(obj: u32, slot: u32) -> u32 {
        FILL_LOG.lock().unwrap().push((obj, slot));
        let v = FILL_ANS.lock().unwrap().pop_front().unwrap();
        unsafe {
            (slot as *mut u32).write_unaligned(v);
        }
        v
    }

    extern "cdecl" fn cookie_stub() -> u32 {
        *COOKIE_CALLS.lock().unwrap() += 1;
        0
    }

    fn plant_stubs() {
        set_callee(RAND_CALLEE, support::fn_addr!(rand_stub));
        set_callee(COOKIE_CALLEE, support::fn_addr!(cookie_stub));
        set_callee(FILL_CALLEE, support::fn_addr!(fill_stub));
    }

    fn reset_scripts(rand: u32, fill: u32) {
        *RAND_ANS.lock().unwrap() = VecDeque::from([rand]);
        *FILL_ANS.lock().unwrap() = VecDeque::from([fill]);
        *RAND_CALLS.lock().unwrap() = 0;
        FILL_LOG.lock().unwrap().clear();
        *COOKIE_CALLS.lock().unwrap() = 0;
    }

    struct Case {
        weights: [u32; MAX_ENTRIES],
        slots: [u32; MAX_ENTRIES],
        count: i32,
        rand: u32,
        fill: u32,
    }

    /// The pick with the threshold test widened to `>=`: the first prefix
    /// at or above the threshold wins.
    fn wrong_pick(picker: &mut WeightedPicker, drawn: u32, fill_ans: u32) -> (u32, [u32; 16], Vec<usize>) {
        use lf_peds_tasks::peds_task::SCALE;
        let mut log = Vec::new();
        let mut prefix = [0.0f32; MAX_ENTRIES];
        let mut total = 0.0f32;
        if picker.count > 0 {
            let n = (picker.count as u32).min(MAX_ENTRIES as u32);
            let nvec = if picker.count >= 4 { n & !3 } else { 0 };
            let mut i = 0u32;
            while i < nvec {
                let p = picker.weights[i as usize] + total;
                prefix[i as usize] = p;
                total = p;
                i += 1;
            }
            while i < n {
                total += picker.weights[i as usize];
                prefix[i as usize] = total;
                i += 1;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let threshold = (drawn as i32 as f32) * SCALE * total;
        if picker.count <= 0 {
            return (EMPTY, picker.slots, log);
        }
        let n = (picker.count as u32).min(MAX_ENTRIES as u32);
        let mut idx = 0u32;
        while idx < n {
            // Wrong: `>=` instead of `>`.
            if prefix[idx as usize] >= threshold {
                break;
            }
            if idx + 1 >= n {
                break;
            }
            idx += 1;
        }
        if picker.slots[idx as usize] == EMPTY {
            log.push(idx as usize);
            picker.slots[idx as usize] = fill_ans;
        }
        (picker.slots[idx as usize], picker.slots, log)
    }

    fn run_case(c: &Case, wrong_caught: &mut u32) {
        // Rewrite side: a real picker image, entries filled arbitrarily
        // (never read, only addressed for the fill call).
        let mut img = Box::new([0u8; IMG_LEN]);
        {
            let mut r = Rng(c.rand ^ (c.count as u32).wrapping_mul(0x9E37_79B9));
            r.bytes(&mut img[..SLOTS_OFF]);
        }
        for (i, s) in c.slots.iter().enumerate() {
            put_u32(img.as_mut(), SLOTS_OFF + i * 4, *s);
        }
        for (i, w) in c.weights.iter().enumerate() {
            put_u32(img.as_mut(), WEIGHTS_OFF + i * 4, *w);
        }
        put_u32(img.as_mut(), COUNT_OFF, c.count as u32);
        let base = addr(img.as_ref());
        reset_scripts(c.rand, c.fill);
        let ret_rw = unsafe { rw_00D44AB0(base) };
        let mut slots_rw = [0u32; MAX_ENTRIES];
        for (i, s) in slots_rw.iter_mut().enumerate() {
            *s = get_u32(img.as_ref(), SLOTS_OFF + i * 4);
        }
        let rand_calls_rw = *RAND_CALLS.lock().unwrap();
        let fill_log_rw = FILL_LOG.lock().unwrap().clone();
        let cookie_rw = *COOKIE_CALLS.lock().unwrap();
        assert_eq!(rand_calls_rw, 1, "rand drawn exactly once");
        assert_eq!(cookie_rw, 1, "cookie check runs on every exit");
        assert!(fill_log_rw.len() <= 1, "fill runs at most once");

        // Lift side: the same inputs as owned data.
        let mut weights = [0.0f32; MAX_ENTRIES];
        for (i, w) in c.weights.iter().enumerate() {
            weights[i] = f32::from_bits(*w);
        }
        let mut picker = WeightedPicker {
            weights,
            slots: c.slots,
            count: c.count,
        };
        let mut rand_calls = 0u32;
        let mut fill_log: Vec<usize> = Vec::new();
        let ret_lift = picker.pick(
            &mut || {
                rand_calls += 1;
                c.rand
            },
            &mut |idx: usize| {
                fill_log.push(idx);
                c.fill
            },
        );
        assert_eq!(rand_calls, 1, "lift draws once");
        assert_eq!(ret_lift, ret_rw, "answer agrees");
        assert_eq!(picker.slots, slots_rw, "slots agree");
        assert_eq!(fill_log.len(), fill_log_rw.len(), "fill count agrees");
        // The translation is proven, not assumed: rebuild the two stub
        // addresses from the lifted index and compare them.
        for (li, rw) in fill_log.iter().zip(fill_log_rw.iter()) {
            let idx = *li as u32;
            assert_eq!(*rw, (base.wrapping_add(idx * 32), base.wrapping_add(0x200 + idx * 4)));
        }

        // Wrong lift: must diverge from the rewrite on some case.
        let mut picker_w = WeightedPicker {
            weights,
            slots: c.slots,
            count: c.count,
        };
        let (ret_w, slots_w, fill_w) = wrong_pick(&mut picker_w, c.rand, c.fill);
        let fill_w_addrs: Vec<(u32, u32)> = fill_w
            .iter()
            .map(|li| {
                let idx = *li as u32;
                (base.wrapping_add(idx * 32), base.wrapping_add(0x200 + idx * 4))
            })
            .collect();
        if ret_w != ret_rw || slots_w != slots_rw || fill_w_addrs != fill_log_rw {
            *wrong_caught += 1;
        }
    }

    fn random_case(rng: &mut Rng) -> Case {
        const COUNTS: [i32; 16] = [
            i32::MIN,
            -100,
            -17,
            -1,
            0,
            1,
            2,
            3,
            4,
            5,
            7,
            8,
            15,
            16,
            17,
            i32::MAX,
        ];
        let mut weights = [0u32; MAX_ENTRIES];
        let mut slots = [0u32; MAX_ENTRIES];
        for w in weights.iter_mut() {
            *w = rng.float_bits();
        }
        for s in slots.iter_mut() {
            *s = match rng.below(4) {
                0 => EMPTY,
                1 => 0,
                _ => rng.u32(),
            };
        }
        Case {
            weights,
            slots,
            count: COUNTS[rng.below(16) as usize],
            rand: match rng.below(8) {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                3 => 0xFFFF_FFFF,
                _ => rng.u32(),
            },
            fill: rng.edge_word(),
        }
    }

    #[test]
    fn weighted_pick_matches_rewrite() {
        let _held = lock();
        plant_stubs();
        let mut wrong_caught = 0u32;
        let mut compared = 0u32;
        // All-zero weights with a zero draw: every prefix ties the zero
        // threshold, so the rewrite takes the last index and the widened
        // mutant takes the first: the mutant catcher.
        run_case(
            &Case {
                weights: [0; MAX_ENTRIES],
                slots: [EMPTY; MAX_ENTRIES],
                count: 16,
                rand: 0,
                fill: 0x1234_5678,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // Non-positive counts answer EMPTY after drawing.
        for count in [0, -1, -16, i32::MIN] {
            run_case(
                &Case {
                    weights: [0x3F80_0000; MAX_ENTRIES],
                    slots: [1; MAX_ENTRIES],
                    count,
                    rand: 0xDEAD_BEEF,
                    fill: 0,
                },
                &mut wrong_caught,
            );
            compared += 1;
        }
        // NaN weights: no prefix exceeds anything, last index wins.
        run_case(
            &Case {
                weights: [0x7FC0_0000; MAX_ENTRIES],
                slots: [7; MAX_ENTRIES],
                count: 16,
                rand: 42,
                fill: 0,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // No empty slot: the fill call never runs.
        run_case(
            &Case {
                weights: [0x4000_0000; MAX_ENTRIES],
                slots: [9; MAX_ENTRIES],
                count: 16,
                rand: 0,
                fill: 0,
            },
            &mut wrong_caught,
        );
        compared += 1;
        // A negative threshold picks the first positive prefix.
        run_case(
            &Case {
                weights: [0x3F80_0000; MAX_ENTRIES],
                slots: [EMPTY; MAX_ENTRIES],
                count: 16,
                rand: 0x8000_0000,
                fill: 5,
            },
            &mut wrong_caught,
        );
        compared += 1;
        let mut rng = Rng(0x70E4_11C5);
        for _ in 0..240 {
            let c = random_case(&mut rng);
            run_case(&c, &mut wrong_caught);
            compared += 1;
        }
        assert!(wrong_caught > 0, "wrong lift was never caught");
        assert!(compared >= 240, "ran {compared} comparisons");
    }
}
