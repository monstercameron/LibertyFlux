//! Differential cases, part 2: the six single-callee bar methods.
//!
//! Each case plants the callee stub and the globals the rewrite reads,
//! runs the rewrite and the lifted method on the same inputs with the
//! same scripted answers, and compares the answer, every written byte,
//! and every collaborator call in order. Each method has a deliberately
//! wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_files_memory::replay_bar::ReplayBar;
    use lf_files_memory::replay_bar::ReplaySlot;
    use lf_files_memory::replay_bar::TimeBases;
    use lf_replaydiff::rewrites::*;
    use lf_replaydiff::rt;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        BAR_LEN, DEN_NARROW_VA, DEN_WIDE_VA, Rng, addr, diff_words, float_corpus, get_word,
        int_corpus, lock, plant_bar, snap,
    };

    /// Scripted answers and call log for the no-arg cdecl sampler.
    static SAMPLE_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static SAMPLE_LOG: Mutex<Vec<()>> = Mutex::new(Vec::new());
    extern "cdecl" fn sample_stub() -> u32 {
        SAMPLE_LOG.lock().unwrap().push(());
        SAMPLE_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    /// Scripted answers and call log for the thiscall stamp scorer.
    static SCORE_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static SCORE_LOG: Mutex<Vec<(u32, u32, u32)>> = Mutex::new(Vec::new());
    extern "thiscall" fn score_stub(this: u32, stamp: u32, zero: u32) -> u32 {
        SCORE_LOG.lock().unwrap().push((this, stamp, zero));
        SCORE_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_files_memory::replay_bar::ReplayBar;
        use lf_files_memory::replay_bar::TimeBases;

        /// Ignores the selector, always taking the narrow word.
        pub fn scaled_position_narrow(bar: &ReplayBar, a: u32, bases: &TimeBases) -> f32 {
            let q = (a as f32) / (bar.total as f32);
            let mut r = bar.width * q;
            r += bar.origin;
            r *= bases.den_narrow.cast_signed() as f32;
            r
        }

        /// Triggers on scores at the threshold, not above it.
        pub fn scan_forward_ge(
            bar: &ReplayBar,
            thresh: u32,
            score: &mut impl FnMut(u32) -> u32,
        ) -> (u32, u32) {
            for slot in &bar.slots {
                let v = score(slot.stamp);
                if v >= thresh {
                    return (score(slot.stamp), u32::from(slot.tag));
                }
            }
            (bar.total, 100)
        }

        /// Triggers on scores at the threshold, not below it.
        pub fn scan_backward_le(
            bar: &ReplayBar,
            thresh: u32,
            score: &mut impl FnMut(u32) -> u32,
        ) -> (u32, u32, u32) {
            for slot in bar.slots.iter().rev() {
                let v = score(slot.stamp);
                if v <= thresh {
                    return (score(slot.stamp), u32::from(slot.tag), slot.stamp);
                }
            }
            (0, 100, 0)
        }

        /// Scores the upper marker before the lower one.
        pub fn measure_swapped(
            bar: &ReplayBar,
            score: &mut impl FnMut(u32) -> u32,
        ) -> (f32, u32, u32) {
            let span = bar.hi - bar.lo;
            let ratio = (bar.total as f32) / span;
            let second = score(bar.bound_hi);
            let first = score(bar.bound_lo);
            (ratio, first, second)
        }

        /// Divides by the negated span.
        pub fn slot_ratio_neg(bar: &ReplayBar, v: u32) -> f32 {
            (v as f32) / (bar.lo - bar.hi)
        }

        /// Stores max on the lower bound and min on the upper one.
        pub fn clamp_bound_swapped(bar: &mut ReplayBar, value: u32, which: u32, t: u32) -> u32 {
            if which == 0 {
                let d = if value <= t { t } else { value };
                bar.bound_lo = d;
                0
            } else if which == 1 {
                let e = if value >= t { t } else { value };
                bar.bound_hi = e;
                e
            } else {
                which
            }
        }
    }

    /// A bar with random geometry and `n` random slots.
    fn random_bar(rng: &mut Rng, n: usize) -> ReplayBar {
        let mut bar = ReplayBar::empty();
        bar.seconds = rng.f32();
        bar.origin = rng.f32();
        bar.weight = rng.f32();
        bar.width = rng.f32();
        bar.hi = rng.f32();
        bar.lo = rng.f32();
        bar.total = rng.u32();
        bar.bound_lo = rng.u32();
        bar.bound_hi = rng.u32();
        for _ in 0..n {
            bar.slots.push(ReplaySlot {
                tag: rng.u32() as u8,
                stamp: rng.u32(),
            });
        }
        bar
    }

    #[test]
    fn scaled_position_matches() {
        let _guard = lock();
        rt::set_callee(1, sample_stub as usize as u32);
        let mut rng = Rng(0xCC20);
        let mut cases = 0;
        let mut caught = 0;
        let modes = [0u32, 1, 0xff, 0x100, 0x101, 0xff00, rng.u32(), rng.u32()];
        let args = int_corpus(&mut rng, 8);
        for trial in 0..24 {
            let mut bar = random_bar(&mut rng, 0);
            if trial == 0 {
                bar.total = 8;
                bar.width = 100.0;
                bar.origin = 5.0;
            }
            if trial == 1 {
                // A zero total pins the divide-by-zero path.
                bar.total = 0;
            }
            let bases = TimeBases {
                num_narrow: rng.u32(),
                num_wide: rng.u32(),
                den_narrow: rng.u32(),
                den_wide: rng.u32() | 1,
            };
            if bases.den_narrow == bases.den_wide {
                continue;
            }
            rt::set_global(DEN_NARROW_VA, bases.den_narrow);
            rt::set_global(DEN_WIDE_VA, bases.den_wide);
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            for &a in &args {
                for &mode in &modes {
                    SAMPLE_SCRIPT.lock().unwrap().clear();
                    SAMPLE_LOG.lock().unwrap().clear();
                    SAMPLE_SCRIPT.lock().unwrap().push_back(mode);
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D6CC20::rw_00d6cc20(base, a) };
                    assert!(diff_words(&before, base).is_empty(), "reader writes");
                    assert_eq!(SAMPLE_LOG.lock().unwrap().len(), 1, "one mode sample");
                    let wide = mode & 0xff != 0;
                    let lift = bar.scaled_position(a, &mut || wide, &bases);
                    assert_eq!((lift as f64).to_bits(), got.to_bits(), "a={a:#x}");
                    if wrong::scaled_position_narrow(&bar, a, &bases).to_bits() != lift.to_bits() {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
            std::hint::black_box(&planted);
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong narrow word never caught ({cases} cases)");
    }

    /// Scripted scorer answers and the matching lift-side script.
    fn scorer_scripts(scores: &[u32]) -> (Vec<u32>, std::collections::VecDeque<u32>) {
        (scores.to_vec(), scores.iter().copied().collect())
    }

    #[test]
    fn scan_forward_matches() {
        let _guard = lock();
        rt::set_callee(1, score_stub as usize as u32);
        let mut rng = Rng(0xF0C0);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..64 {
            let n = if trial < 8 { trial % 4 } else { rng.below(7) as usize };
            let bar = random_bar(&mut rng, n);
            // Thresholds: edges, around the middle, max (no hit possible).
            let mut threshs = vec![0u32, 1, u32::MAX - 1, u32::MAX, rng.u32()];
            if trial == 0 {
                threshs = vec![50];
            }
            for &thresh in &threshs {
                // Hit position: none, first, middle, last (when hittable).
                let spots: Vec<Option<usize>> = if thresh == u32::MAX || n == 0 {
                    vec![None]
                } else if trial % 2 == 0 {
                    vec![None, Some(0), Some(n / 2), Some(n - 1)]
                } else {
                    vec![if n == 0 { None } else { Some(rng.below(n as u32) as usize) }]
                };
                for spot in spots {
                    // Scores at the threshold never trigger; the hit score
                    // exceeds it and the re-score differs from the first.
                    let mut scores = Vec::new();
                    for i in 0..n {
                        if Some(i) == spot {
                            scores.push(thresh.wrapping_add(1));
                            scores.push(rng.u32());
                        } else {
                            scores.push(thresh);
                        }
                        if Some(i) == spot {
                            break;
                        }
                    }
                    let planted = plant_bar(&bar, 0, &mut rng);
                    let base = planted.base();
                    let out_box = Box::new(0xDEADu32);
                    let out = addr(&*out_box);
                    SCORE_SCRIPT.lock().unwrap().clear();
                    SCORE_LOG.lock().unwrap().clear();
                    let (rw_scores, mut lift_script) = scorer_scripts(&scores);
                    SCORE_SCRIPT.lock().unwrap().extend(rw_scores);
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D6F0C0::rw_00d6f0c0(base, thresh, out) };
                    assert!(diff_words(&before, base).is_empty(), "scanner writes");
                    let out_rw = get_word(out, 0);
                    let rw_log = SCORE_LOG.lock().unwrap().clone();
                    for &(this, _, zero) in &rw_log {
                        assert_eq!(this, base, "scorer takes the bar");
                        assert_eq!(zero, 0, "scorer's trailing zero");
                    }
                    let mut lift_log = Vec::new();
                    let mut lift_calls = 0;
                    let lift = bar.scan_forward(thresh, &mut |stamp: u32| {
                        lift_log.push(stamp);
                        lift_calls += 1;
                        lift_script.pop_front().unwrap()
                    });
                    assert_eq!(got, lift.0, "thresh={thresh:#x}");
                    assert_eq!(out_rw, lift.1, "tag word");
                    assert_eq!(
                        rw_log.iter().map(|&(_, s, _)| s).collect::<Vec<_>>(),
                        lift_log,
                        "call order"
                    );
                    // The wrong lift triggers on equality: every
                    // at-threshold score is a witness.
                    let mut wscript: VecDeque<u32> = scores.iter().copied().collect(); wscript.extend([thresh, thresh, thresh, thresh]);
                    let wl = wrong::scan_forward_ge(&bar, thresh, &mut |_| {
                        wscript.pop_front().unwrap()
                    });
                    if wl != lift {
                        caught += 1;
                    }
                    cases += 1;
                    std::hint::black_box((&planted, &out_box));
                }
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong threshold sense never caught ({cases} cases)");
    }

    #[test]
    fn scan_backward_matches() {
        let _guard = lock();
        rt::set_callee(1, score_stub as usize as u32);
        let mut rng = Rng(0xF150);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..64 {
            let n = if trial < 8 { trial % 4 } else { rng.below(7) as usize };
            let bar = random_bar(&mut rng, n);
            let mut threshs = vec![0u32, 1, 2, u32::MAX, rng.u32()];
            if trial == 0 {
                threshs = vec![50];
            }
            for &thresh in &threshs {
                let spots: Vec<Option<usize>> = if thresh == 0 || n == 0 {
                    vec![None]
                } else if trial % 2 == 0 {
                    vec![None, Some(n - 1), Some(n / 2), Some(0)]
                } else {
                    vec![if n == 0 { None } else { Some(rng.below(n as u32) as usize) }]
                };
                for spot in spots {
                    // From the end: at-threshold scores never trigger.
                    let mut scores = Vec::new();
                    for i in (0..n).rev() {
                        if Some(i) == spot {
                            scores.push(thresh.wrapping_sub(1));
                            scores.push(rng.u32());
                        } else {
                            scores.push(thresh);
                        }
                        if Some(i) == spot {
                            break;
                        }
                    }
                    let planted = plant_bar(&bar, 0, &mut rng);
                    let base = planted.base();
                    let tag_box = Box::new(0xDEADu32);
                    let stamp_box = Box::new(0xDEADu32);
                    let out_tag = addr(&*tag_box);
                    let out_stamp = addr(&*stamp_box);
                    SCORE_SCRIPT.lock().unwrap().clear();
                    SCORE_LOG.lock().unwrap().clear();
                    let (rw_scores, mut lift_script) = scorer_scripts(&scores);
                    SCORE_SCRIPT.lock().unwrap().extend(rw_scores);
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D6F150::rw_00d6f150(base, thresh, out_tag, out_stamp) };
                    assert!(diff_words(&before, base).is_empty(), "scanner writes");
                    let tag_rw = get_word(out_tag, 0);
                    let stamp_rw = get_word(out_stamp, 0);
                    let rw_log = SCORE_LOG.lock().unwrap().clone();
                    for &(this, _, zero) in &rw_log {
                        assert_eq!(this, base, "scorer takes the bar");
                        assert_eq!(zero, 0, "scorer's trailing zero");
                    }
                    let mut lift_log = Vec::new();
                    let lift = bar.scan_backward(thresh, &mut |stamp: u32| {
                        lift_log.push(stamp);
                        lift_script.pop_front().unwrap()
                    });
                    assert_eq!(got, lift.0, "thresh={thresh:#x}");
                    assert_eq!((tag_rw, stamp_rw), (lift.1, lift.2), "output words");
                    assert_eq!(
                        rw_log.iter().map(|&(_, s, _)| s).collect::<Vec<_>>(),
                        lift_log,
                        "call order"
                    );
                    let mut wscript: VecDeque<u32> = scores.iter().copied().collect(); wscript.extend([thresh, thresh, thresh, thresh]);
                    let wl = wrong::scan_backward_le(&bar, thresh, &mut |_| {
                        wscript.pop_front().unwrap()
                    });
                    if wl != lift {
                        caught += 1;
                    }
                    cases += 1;
                    std::hint::black_box((&planted, &tag_box, &stamp_box));
                }
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong threshold sense never caught ({cases} cases)");
    }

    #[test]
    fn measure_slots_matches() {
        let _guard = lock();
        rt::set_callee(1, score_stub as usize as u32);
        let mut rng = Rng(0xF1E0);
        let mut cases = 0;
        let mut caught = 0;
        let spans = float_corpus(&mut rng, 8);
        for trial in 0..24 {
            let mut bar = random_bar(&mut rng, 0);
            bar.hi = spans[trial % spans.len()];
            bar.lo = spans[(trial * 3 + 1) % spans.len()];
            if trial == 0 {
                bar.hi = 10.0;
                bar.lo = 2.0;
                bar.total = 8;
            }
            if trial == 1 {
                bar.hi = 3.0;
                bar.lo = 3.0;
            }
            let (s1, s2) = (rng.u32(), rng.u32() | 0x8000_0001);
            if s1 == s2 {
                continue;
            }
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            let ratio_box = Box::new(0xDEADu32);
            let first_box = Box::new(0xDEADu32);
            let second_box = Box::new(0xDEADu32);
            let (o_ratio, o_first, o_second) =
                (addr(&*ratio_box), addr(&*first_box), addr(&*second_box));
            SCORE_SCRIPT.lock().unwrap().clear();
            SCORE_LOG.lock().unwrap().clear();
            SCORE_SCRIPT.lock().unwrap().extend([s1, s2]);
            let before = snap(base, BAR_LEN);
            let got = unsafe { fn_00D6F1E0::rw_00d6f1e0(base, o_ratio, o_first, o_second) };
            assert!(diff_words(&before, base).is_empty(), "publisher writes");
            let rw_log = SCORE_LOG.lock().unwrap().clone();
            assert_eq!(
                rw_log,
                [(base, bar.bound_lo, 0), (base, bar.bound_hi, 0)],
                "marker order"
            );
            let mut lift_script: VecDeque<u32> = [s1, s2].into_iter().collect();
            let mut lift_log = Vec::new();
            let lift = bar.measure_slots(&mut |stamp: u32| {
                lift_log.push(stamp);
                lift_script.pop_front().unwrap()
            });
            assert_eq!(got, lift.2, "the second score wins");
            assert_eq!(get_word(o_ratio, 0), lift.0.to_bits(), "ratio word");
            assert_eq!(get_word(o_first, 0), lift.1, "first word");
            assert_eq!(get_word(o_second, 0), lift.2, "second word");
            assert_eq!(lift_log, [bar.bound_lo, bar.bound_hi]);
            let mut wscript: VecDeque<u32> = [s1, s2].into_iter().collect();
            let wl = wrong::measure_swapped(&bar, &mut |_| wscript.pop_front().unwrap());
            if wl != lift {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box((&planted, &ratio_box, &first_box, &second_box));
        }
        assert!(cases > 10, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong marker order never caught ({cases} cases)");
    }

    #[test]
    fn slot_ratio_matches() {
        let _guard = lock();
        rt::set_callee(1, sample_stub as usize as u32);
        let mut rng = Rng(0xF490);
        let mut cases = 0;
        let mut caught = 0;
        let spans = float_corpus(&mut rng, 8);
        let readings = int_corpus(&mut rng, 8);
        for trial in 0..16 {
            let mut bar = random_bar(&mut rng, 0);
            bar.hi = spans[trial % spans.len()];
            bar.lo = spans[(trial * 5 + 2) % spans.len()];
            if trial == 0 {
                bar.hi = 10.0;
                bar.lo = 2.0;
            }
            if trial == 1 {
                bar.hi = 4.0;
                bar.lo = 4.0;
            }
            for &v in &readings {
                let planted = plant_bar(&bar, 0, &mut rng);
                let base = planted.base();
                let ratio_box = Box::new(0xDEADu32);
                let first_box = Box::new(0xDEADu32);
                let second_box = Box::new(0xDEADu32);
                let (o_ratio, o_first, o_second) =
                    (addr(&*ratio_box), addr(&*first_box), addr(&*second_box));
                SAMPLE_SCRIPT.lock().unwrap().clear();
                SAMPLE_LOG.lock().unwrap().clear();
                SAMPLE_SCRIPT.lock().unwrap().push_back(v);
                let before = snap(base, BAR_LEN);
                let got = unsafe { fn_00D6F490::rw_00d6f490(base, o_ratio, o_first, o_second) };
                assert!(diff_words(&before, base).is_empty(), "publisher writes");
                assert_eq!(SAMPLE_LOG.lock().unwrap().len(), 1, "one reading");
                assert_eq!(got, o_second, "the echoed pointer is pinned");
                let mut script: VecDeque<u32> = [v].into_iter().collect();
                let lift = bar.slot_ratio(&mut || script.pop_front().unwrap());
                assert_eq!(get_word(o_ratio, 0), lift.0.to_bits(), "ratio word");
                assert_eq!((get_word(o_first, 0), get_word(o_second, 0)), (lift.1, lift.2));
                assert_eq!((lift.1, lift.2), (bar.bound_lo, bar.bound_hi));
                if wrong::slot_ratio_neg(&bar, v).to_bits() != lift.0.to_bits() {
                    caught += 1;
                }
                cases += 1;
                std::hint::black_box((&planted, &ratio_box, &first_box, &second_box));
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong negated span never caught ({cases} cases)");
    }

    #[test]
    fn clamp_bound_matches() {
        let _guard = lock();
        rt::set_callee(1, sample_stub as usize as u32);
        let mut rng = Rng(0x0910);
        let mut cases = 0;
        let mut caught = 0;
        let values = int_corpus(&mut rng, 8);
        let whichs = [0u32, 1, 2, 3, 100, u32::MAX];
        for &value in &values {
            for &t in &values {
                for &which in &whichs {
                    let mut bar = random_bar(&mut rng, 0);
                    // Bounds start inverted so every store changes a word.
                    bar.bound_lo = !value.min(t);
                    bar.bound_hi = !value.max(t);
                    let planted = plant_bar(&bar, 0, &mut rng);
                    let base = planted.base();
                    SAMPLE_SCRIPT.lock().unwrap().clear();
                    SAMPLE_LOG.lock().unwrap().clear();
                    SAMPLE_SCRIPT.lock().unwrap().push_back(t);
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D70910::rw_00d70910(base, value, which) };
                    // The sample runs even for an unknown selector.
                    assert_eq!(SAMPLE_LOG.lock().unwrap().len(), 1, "always sampled");
                    let mut lift = bar.clone();
                    let mut script: VecDeque<u32> = [t].into_iter().collect();
                    let want = lift.clamp_bound(value, which, &mut || {
                        script.pop_front().unwrap()
                    });
                    assert_eq!(got, want, "value={value:#x} t={t:#x} which={which}");
                    assert_eq!(get_word(base, 0xb8), lift.bound_lo, "lower word");
                    assert_eq!(get_word(base, 0xcc), lift.bound_hi, "upper word");
                    let diff = diff_words(&before, base);
                    if which == 0 {
                        assert_eq!(diff, vec![0xb8usize]);
                    } else if which == 1 {
                        assert_eq!(diff, vec![0xccusize]);
                    } else {
                        assert!(diff.is_empty(), "unknown selector writes");
                    }
                    let mut wbar = bar.clone();
                    let wl = wrong::clamp_bound_swapped(&mut wbar, value, which, t);
                    if wl != want || wbar.bound_lo != lift.bound_lo || wbar.bound_hi != lift.bound_hi
                    {
                        caught += 1;
                    }
                    cases += 1;
                    std::hint::black_box(&planted);
                }
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong swapped clamp never caught ({cases} cases)");
    }
}
