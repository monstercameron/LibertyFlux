//! Differential cases, part 1: the seven call-free bar methods.
//!
//! Each case builds the 32-bit bar, runs the rewrite and the lifted method
//! on the same inputs, and compares the answer with every effect (pure
//! readers must leave the whole image untouched). Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_files_memory::replay_bar::ReplayBar;
    use lf_files_memory::replay_bar::ReplaySlot;
    use lf_replaydiff::rewrites::*;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        addr, diff_words, float_corpus, get_word, int_corpus, lock, plant_bar, snap, Rng, BAR_LEN,
    };

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_files_memory::replay_bar::ReplayBar;

        /// Rounds half up on both branches (forgets the negative arm).
        pub fn scaled_index_up(bar: &ReplayBar, pos: f32) -> i32 {
            let span = bar.hi - bar.lo;
            let mut v = (bar.total as f32) / span;
            v *= pos - bar.lo;
            let r = v + 0.5;
            if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
                i32::MIN
            } else {
                r as i32
            }
        }

        /// Skips the second clamp window.
        pub fn clamped_index_once(bar: &ReplayBar, pos: f32) -> i32 {
            let span = bar.hi - bar.lo;
            let ratio = (bar.total as f32) / span;
            let mut v = pos;
            if bar.lo > v {
                v = bar.lo;
            }
            if v > bar.hi {
                v = bar.hi;
            }
            v -= bar.lo;
            v *= ratio;
            let r = if 0.0 > v { v - 0.5 } else { v + 0.5 };
            if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
                i32::MIN
            } else {
                r as i32
            }
        }

        /// Reports the first passing slot from the start, not the last.
        pub fn find_slot_first(bar: &ReplayBar) -> Option<usize> {
            for (i, slot) in bar.slots.iter().enumerate() {
                if slot.stamp <= bar.bound_hi {
                    return Some(i);
                }
            }
            None
        }

        /// Scales by centiseconds instead of milliseconds.
        pub fn millis_centis(bar: &ReplayBar) -> u32 {
            let v = bar.seconds * 100.0;
            if v.is_nan() || v >= 9223372036854775808.0 || v <= -9223372036854775808.0 {
                0
            } else {
                (v as i64) as u32
            }
        }

        /// Admits edge points (non-strict comparisons).
        pub fn hit_test_loose(bar: &ReplayBar, x: f32, y: f32) -> bool {
            x >= bar.rect0.2 && bar.rect1.0 >= x && y >= bar.rect0.1 && bar.rect0.3 >= y
        }

        /// Swaps the two mode rectangles.
        pub fn region_swapped(bar: &ReplayBar, x: f32, y: f32, mode: u32) -> bool {
            let (x0, y0, x1, y1) = if mode == 0 {
                bar.rect1
            } else if mode == 1 {
                bar.rect0
            } else {
                return false;
            };
            x > x0 && x1 > x && y > y0 && y1 > y
        }
    }

    /// A bar with random geometry and `n` random slots.
    fn random_bar(rng: &mut Rng, n: usize) -> ReplayBar {
        let mut bar = ReplayBar::empty();
        bar.hi = rng.f32();
        bar.lo = rng.f32();
        bar.total = rng.u32();
        bar.bound_lo = rng.u32();
        bar.bound_hi = rng.u32();
        bar.seconds = rng.f32();
        bar.rect0 = (rng.f32(), rng.f32(), rng.f32(), rng.f32());
        bar.rect1 = (rng.f32(), rng.f32(), rng.f32(), rng.f32());
        for _ in 0..n {
            bar.slots.push(ReplaySlot {
                tag: rng.u32() as u8,
                stamp: rng.u32(),
            });
        }
        bar
    }

    #[test]
    fn scaled_index_matches() {
        let _guard = lock();
        let mut rng = Rng(0xCE70);
        let mut cases = 0;
        let mut caught = 0;
        let positions = float_corpus(&mut rng, 24);
        // The range edges cycle the corpus too: NaN payloads observe the
        // multiply order, so random bits alone are too weak a guard.
        let edges = float_corpus(&mut Rng(0xE06E), 0);
        for trial in 0..24 {
            let n = rng.below(4) as usize;
            let mut bar = random_bar(&mut rng, n);
            bar.hi = edges[trial % edges.len()];
            bar.lo = edges[(trial * 7 + 3) % edges.len()];
            if trial == 0 {
                // A sane range pins the common path.
                bar.lo = 2.0;
                bar.hi = 10.0;
                bar.total = 8;
            }
            if trial == 1 {
                // A zero span pins the division path.
                bar.lo = 3.0;
                bar.hi = 3.0;
            }
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            for &pos in &positions {
                let before = snap(base, BAR_LEN);
                let got = unsafe { fn_00D6CE70::rw_00d6ce70(base, pos.to_bits()) };
                assert!(diff_words(&before, base).is_empty(), "pure reader writes");
                let lift = bar.scaled_index(pos);
                assert_eq!(got, lift as u32, "pos={pos:?}");
                if wrong::scaled_index_up(&bar, pos) != lift {
                    caught += 1;
                }
                cases += 1;
            }
            std::hint::black_box(&planted);
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong rounding never caught ({cases} cases)");
    }

    #[test]
    fn clamped_index_matches() {
        let _guard = lock();
        let mut rng = Rng(0xCDC0);
        let mut cases = 0;
        let mut caught = 0;
        let positions = float_corpus(&mut rng, 16);
        let flags = [0u32, 1, 0xff, 0x100, 0x101, 0x1_0000, rng.u32(), rng.u32()];
        let edges = float_corpus(&mut Rng(0xED6E), 0);
        for trial in 0..16 {
            let mut bar = random_bar(&mut rng, 0);
            bar.hi = edges[trial % edges.len()];
            bar.lo = edges[(trial * 5 + 2) % edges.len()];
            if trial == 0 {
                bar.lo = 0.0;
                bar.hi = 100.0;
                bar.total = 10;
                bar.rect0.2 = 20.0;
                bar.rect1.0 = 40.0;
            }
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            for &pos in &positions {
                for &flag in &flags {
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D6CDC0::rw_00d6cdc0(base, pos.to_bits(), flag) };
                    assert!(diff_words(&before, base).is_empty(), "pure reader writes");
                    let lift = bar.clamped_index(pos, flag & 0xff != 0);
                    assert_eq!(got, lift as u32, "pos={pos:?} flag={flag:#x}");
                    // The wrong lift only applies when the window is on.
                    if flag & 0xff != 0 && wrong::clamped_index_once(&bar, pos) != lift {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
            std::hint::black_box(&planted);
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong missing window never caught ({cases} cases)"
        );
    }

    #[test]
    fn find_slot_matches() {
        let _guard = lock();
        let mut rng = Rng(0xEFF0);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..48 {
            let n = if trial < 8 {
                trial
            } else {
                rng.below(9) as usize
            };
            let mut bar = ReplayBar::empty();
            for _ in 0..n {
                bar.slots.push(ReplaySlot {
                    tag: rng.u32() as u8,
                    stamp: rng.u32(),
                });
            }
            if trial == 1 && n >= 2 {
                // Two passing stamps pin last-match reporting.
                bar.slots[0].stamp = 5;
                bar.slots[n - 1].stamp = 7;
                bar.bound_hi = 10;
            } else {
                bar.bound_hi = rng.u32();
            }
            // Bounds around the planted stamps, plus edges.
            let mut bounds = vec![0u32, 1, u32::MAX, rng.u32()];
            for s in &bar.slots {
                bounds.push(s.stamp);
                bounds.push(s.stamp.wrapping_add(1));
                bounds.push(s.stamp.wrapping_sub(1));
            }
            for &bound in &bounds {
                bar.bound_hi = bound;
                let planted = plant_bar(&bar, 0, &mut rng);
                let base = planted.base();
                let before = snap(base, BAR_LEN);
                let got = unsafe { fn_00D6EFF0::rw_00d6eff0(base) };
                assert!(diff_words(&before, base).is_empty(), "pure reader writes");
                let lift = bar.find_slot();
                assert_eq!(
                    got,
                    lift.map_or(0xffff_ffff, |i| i as u32),
                    "bound={bound:#x}"
                );
                if wrong::find_slot_first(&bar) != lift {
                    caught += 1;
                }
                cases += 1;
                std::hint::black_box(&planted);
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong first-match never caught ({cases} cases)");
    }

    #[test]
    fn millis_rounded_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF250);
        let mut cases = 0;
        let mut caught = 0;
        let values = float_corpus(&mut rng, 64);
        for &secs in &values {
            let mut bar = ReplayBar::empty();
            bar.seconds = secs;
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            let before = snap(base, BAR_LEN);
            let got = unsafe { fn_00D6F250::rw_00d6f250(base) };
            assert!(diff_words(&before, base).is_empty(), "pure reader writes");
            let lift = bar.millis_rounded();
            assert_eq!(got, lift, "secs={secs:?}");
            if wrong::millis_centis(&bar) != lift {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&planted);
        }
        assert!(cases > 50, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong scale never caught ({cases} cases)");
    }

    #[test]
    fn hit_test_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF5C0);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..24 {
            let mut bar = random_bar(&mut rng, 0);
            if trial == 0 {
                // A sane rectangle pins edges exactly.
                bar.rect0 = (0.0, 0.0, 10.0, 10.0);
                bar.rect1.0 = 20.0;
            }
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            // Points: edges, corners, middles, NaNs, random.
            let (x0, y0, _, y1) = (bar.rect0.2, bar.rect0.1, 0.0, bar.rect0.3);
            let x1 = bar.rect1.0;
            let mut xs = vec![x0, x1, (x0 + x1) / 2.0, f32::NAN, 0.0, rng.f32(), rng.f32()];
            let mut ys = vec![y0, y1, (y0 + y1) / 2.0, f32::NAN, 0.0, rng.f32(), rng.f32()];
            for &x in &xs.clone() {
                xs.push(f32::from_bits(x.to_bits().wrapping_add(1)));
            }
            for &y in &ys.clone() {
                ys.push(f32::from_bits(y.to_bits().wrapping_add(1)));
            }
            for &x in &xs {
                for &y in &ys {
                    let pt = Box::new([x.to_bits(), y.to_bits()]);
                    let p = addr(&pt[0]);
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D6F5C0::rw_00d6f5c0(base, p) };
                    assert!(diff_words(&before, base).is_empty(), "pure reader writes");
                    let lift = bar.hit_test(x, y);
                    assert_eq!(got, u8::from(lift), "x={x:?} y={y:?}");
                    if wrong::hit_test_loose(&bar, x, y) != lift {
                        caught += 1;
                    }
                    cases += 1;
                    std::hint::black_box(&pt);
                }
            }
            std::hint::black_box(&planted);
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong loose edges never caught ({cases} cases)");
    }

    #[test]
    fn region_hit_test_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF610);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..16 {
            let mut bar = random_bar(&mut rng, 0);
            if trial == 0 {
                // Disjoint rectangles pin the mode select.
                bar.rect0 = (0.0, 0.0, 10.0, 10.0);
                bar.rect1 = (20.0, 20.0, 30.0, 30.0);
            }
            let planted = plant_bar(&bar, 0, &mut rng);
            let base = planted.base();
            let mut pts = vec![(5.0f32, 5.0f32), (25.0, 25.0), (15.0, 15.0)];
            for _ in 0..6 {
                pts.push((rng.f32(), rng.f32()));
            }
            for &(x, y) in &pts {
                for mode in [0u32, 1, 2, 3, 100, u32::MAX] {
                    let pt = Box::new([x.to_bits(), y.to_bits()]);
                    let p = addr(&pt[0]);
                    let before = snap(base, BAR_LEN);
                    let got = unsafe { fn_00D6F610::rw_00d6f610(base, p, mode) };
                    assert!(diff_words(&before, base).is_empty(), "pure reader writes");
                    let lift = bar.region_hit_test(x, y, mode);
                    assert_eq!(got, u8::from(lift), "mode={mode}");
                    if wrong::region_swapped(&bar, x, y, mode) != lift {
                        caught += 1;
                    }
                    cases += 1;
                    std::hint::black_box(&pt);
                }
            }
            std::hint::black_box(&planted);
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong swapped rects never caught ({cases} cases)"
        );
    }

    #[test]
    fn store_cursor_matches() {
        let _guard = lock();
        let mut rng = Rng(0x08F0);
        let mut cases = 0;
        let mut caught = 0;
        let positions = float_corpus(&mut rng, 8);
        let gens = int_corpus(&mut rng, 8);
        for &pos in &positions {
            for &gen in &gens {
                let n = rng.below(3) as usize;
                let mut bar = random_bar(&mut rng, n);
                // Inverted words guarantee both stores change a word.
                bar.cursor = !pos.to_bits();
                bar.total = !gen;
                let planted = plant_bar(&bar, 0, &mut rng);
                let base = planted.base();
                let before = snap(base, BAR_LEN);
                let got = unsafe { fn_00D708F0::rw_00d708f0(base, pos.to_bits(), gen) };
                assert_eq!(got, gen);
                assert_eq!(diff_words(&before, base), vec![0x20usize, 0x100usize]);
                assert_eq!(get_word(base, 0x20), pos.to_bits());
                assert_eq!(get_word(base, 0x100), gen);
                // The lift stores the same words and answers the generation.
                let mut lift = bar.clone();
                let answer = lift.store_cursor(pos, gen);
                assert_eq!(answer, gen);
                assert_eq!(lift.cursor, pos.to_bits());
                assert_eq!(lift.total, gen);
                assert_eq!(lift.selected, bar.selected);
                assert_eq!(lift.slots, bar.slots);
                // Wrong: the swapped store.
                if pos.to_bits() != gen {
                    caught += 1;
                }
                cases += 1;
                std::hint::black_box(&planted);
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong swapped store never caught ({cases} cases)"
        );
    }
}
