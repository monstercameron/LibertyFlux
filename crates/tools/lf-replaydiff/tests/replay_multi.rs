//! Differential cases, part 3: the multi-callee methods and peers.
//!
//! Each case plants every stub and global its rewrite touches, runs the
//! rewrite and the lifted method on the same inputs with the same
//! scripted answers, and compares the answer, every written byte and
//! global, and every collaborator call in order across all callees (one
//! shared order log on each side). Each method has a deliberately wrong
//! lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::sync::Mutex;

    use lf_files_memory::replay_bar::blend_factors;
    use lf_files_memory::replay_bar::ClockHandle;
    use lf_files_memory::replay_bar::InnerHandle;
    use lf_files_memory::replay_bar::NotifierHandle;
    use lf_files_memory::replay_bar::NotifyCtl;
    use lf_files_memory::replay_bar::RefreshOutcome;
    use lf_files_memory::replay_bar::ReplayBar;
    use lf_files_memory::replay_bar::ReplaySlot;
    use lf_files_memory::replay_bar::StampPublish;
    use lf_files_memory::replay_bar::TimeBases;
    use lf_replaydiff::rewrites::*;
    use lf_replaydiff::rt;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        addr, diff_words, float_corpus, get_word, int_corpus, lock, plant_bar, snap, Rng, BAR_LEN,
        DEN_NARROW_VA, DEN_WIDE_VA, HUB_VA, NUM_NARROW_VA, NUM_WIDE_VA, PUB_FLAGS_VA, PUB_STAMP_VA,
        STATE_VA,
    };

    /// Unified call-order log: every stub pushes its tag.
    static ORDER: Mutex<Vec<char>> = Mutex::new(Vec::new());
    /// Scripted answers shared by the word-returning stubs.
    static SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    /// Arguments the address-taking stubs saw.
    static ARGS: Mutex<Vec<(char, u32)>> = Mutex::new(Vec::new());

    extern "cdecl" fn sample_stub() -> u32 {
        ORDER.lock().unwrap().push('s');
        SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }
    extern "thiscall" fn watch_stub(this: u32) -> u32 {
        ORDER.lock().unwrap().push('w');
        ARGS.lock().unwrap().push(('w', this));
        0xDEAD
    }
    extern "cdecl" fn publish_stub(code: u32) -> u32 {
        ORDER.lock().unwrap().push('p');
        ARGS.lock().unwrap().push(('p', code));
        0xDEAD
    }
    extern "thiscall" fn refresh_stub(inner: u32) -> u32 {
        ORDER.lock().unwrap().push('r');
        ARGS.lock().unwrap().push(('r', inner));
        0xDEAD
    }
    extern "thiscall" fn hub_stub(hub: u32) -> u32 {
        ORDER.lock().unwrap().push('h');
        ARGS.lock().unwrap().push(('h', hub));
        SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }
    /// Vtable slot `+0x20`: planted by address into fake vtables.
    extern "thiscall" fn clock_a_stub(obj: u32) -> u32 {
        ORDER.lock().unwrap().push('a');
        ARGS.lock().unwrap().push(('a', obj));
        CLOCK_A.lock().unwrap().pop_front().unwrap_or(0)
    }
    /// Vtable slot `+0x24`: planted by address into fake vtables.
    extern "thiscall" fn clock_b_stub(obj: u32) -> u32 {
        ORDER.lock().unwrap().push('b');
        ARGS.lock().unwrap().push(('b', obj));
        CLOCK_B.lock().unwrap().pop_front().unwrap_or(0)
    }
    static CLOCK_A: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static CLOCK_B: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());

    /// Clears every stub log and script.
    fn reset_stubs() {
        ORDER.lock().unwrap().clear();
        SCRIPT.lock().unwrap().clear();
        ARGS.lock().unwrap().clear();
        CLOCK_A.lock().unwrap().clear();
        CLOCK_B.lock().unwrap().clear();
    }

    /// A fake clock object: link box, object box, vtable box.
    struct FakeClock {
        link: Box<u32>,
        obj: Box<u32>,
        vtab: Box<[u32; 12]>,
    }

    impl FakeClock {
        fn new(rng: &mut Rng) -> Self {
            let mut vtab = Box::new([0u32; 12]);
            for w in vtab.iter_mut() {
                *w = rng.u32();
            }
            vtab[8] = clock_a_stub as usize as u32;
            vtab[9] = clock_b_stub as usize as u32;
            let obj = Box::new(addr(&vtab[0]));
            let link = Box::new(addr(&*obj));
            Self { link, obj, vtab }
        }

        fn link_addr(&self) -> u32 {
            addr(&*self.link)
        }

        fn obj_addr(&self) -> u32 {
            addr(&*self.obj)
        }
    }

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_files_memory::replay_bar::ReplayBar;
        use lf_files_memory::replay_bar::StampPublish;
        use lf_files_memory::replay_bar::TimeBases;

        /// Publishes on equality too (non-strict stamp tests).
        #[allow(clippy::too_many_arguments)]
        pub fn select_nonstrict(
            bar: &mut ReplayBar,
            idx: u32,
            v1: u32,
            v2: u32,
            state: &mut StampPublish,
            codes: &mut Vec<u32>,
        ) -> Option<u32> {
            if bar.slots.is_empty() {
                return None;
            }
            bar.selected = idx;
            bar.sel_mark = 0xffff_ffff;
            let stamp = bar.slots[idx as usize].stamp;
            let mut ret = v2;
            if stamp >= v1 {
                codes.push(6);
                state.flags |= 1;
                state.stamp = stamp;
                ret = stamp;
            }
            if stamp <= v2 {
                codes.push(0x0e);
                state.flags |= 1;
                state.stamp = stamp;
                ret = stamp;
            }
            Some(ret)
        }

        /// Subtracts the ticks from 16 instead of 17.
        pub fn time_factor_16(bar: &ReplayBar, ticks: u32, num1: u32, num2: u32) -> f32 {
            let mut x = 16.0 - (ticks.cast_signed() as f32);
            x += (num1.cast_signed() as f32) * bar.weight;
            x /= num2.cast_signed() as f32;
            x
        }

        /// Divides the scale on the low-ratio path instead of the blend.
        pub fn blend_flipped(f1: f32, f2: f32, scale: &mut f32, ratio: f32) -> (f32, f32, f32) {
            let mut x = f1 / f2;
            let sc = *scale;
            x *= sc;
            let skewed = x;
            if ratio >= 1.0 {
                x *= ratio;
                (f1, f2, x)
            } else {
                *scale = sc / ratio;
                (f1, f2, skewed)
            }
        }

        /// Suppresses state 18 as well.
        pub fn suppressed_wide(state: u32) -> bool {
            matches!(
                state,
                2 | 7 | 8 | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 18
            )
        }
    }

    /// A bar with random geometry and `n` random slots.
    fn random_bar(rng: &mut Rng, n: usize) -> ReplayBar {
        let mut bar = ReplayBar::empty();
        bar.origin = rng.f32();
        bar.weight = rng.f32();
        bar.width = rng.f32();
        bar.hi = rng.f32();
        bar.lo = rng.f32();
        bar.total = rng.u32();
        bar.bound_lo = rng.u32();
        bar.bound_hi = rng.u32();
        bar.selected = rng.u32();
        bar.sel_mark = rng.u32();
        for _ in 0..n {
            bar.slots.push(ReplaySlot {
                tag: rng.u32() as u8,
                stamp: rng.u32(),
            });
        }
        bar
    }

    #[test]
    fn select_entry_matches() {
        let _guard = lock();
        rt::set_callee(1, watch_stub as usize as u32);
        rt::set_callee(2, sample_stub as usize as u32);
        rt::set_callee(3, publish_stub as usize as u32);
        let mut rng = Rng(0xD0C0);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..48 {
            let n = if trial < 6 {
                trial % 3
            } else {
                1 + (rng.below(5) as usize)
            };
            let mut bar = random_bar(&mut rng, n);
            // Indexes: first, last, and the current selection.
            let mut idxs = vec![0u32];
            if n > 0 {
                idxs.push(n as u32 - 1);
                idxs.push(bar.selected.wrapping_rem(n as u32));
                if trial % 3 == 0 {
                    bar.selected = 0;
                }
            }
            for &idx in &idxs {
                let stamp = if n == 0 {
                    0
                } else {
                    bar.slots[idx as usize].stamp
                };
                // Samples around the stamp hit every publish combination.
                let samples = [
                    stamp.wrapping_sub(1),
                    stamp,
                    stamp.wrapping_add(1),
                    0,
                    u32::MAX,
                ];
                for &v1 in &samples {
                    for &v2 in &samples {
                        let flags0 = rng.u32();
                        let stamp0 = rng.u32();
                        rt::set_global(PUB_FLAGS_VA, flags0);
                        rt::set_global(PUB_STAMP_VA, stamp0);
                        let planted = plant_bar(&bar, 0, &mut rng);
                        let base = planted.base();
                        reset_stubs();
                        SCRIPT.lock().unwrap().extend([v1, v2]);
                        let before = snap(base, BAR_LEN);
                        let got = unsafe { fn_00D6D0C0::rw_00d6d0c0(base, idx) };
                        let rw_order = ORDER.lock().unwrap().clone();
                        let rw_args = ARGS.lock().unwrap().clone();
                        let flags_rw = unsafe { rt::global::<u32>(PUB_FLAGS_VA).read() };
                        let stamp_rw = unsafe { rt::global::<u32>(PUB_STAMP_VA).read() };
                        // The lift side replays the same script.
                        let mut lift = bar.clone();
                        let mut state = StampPublish {
                            flags: flags0,
                            stamp: stamp0,
                        };
                        let lift_order = Rc::new(RefCell::new(Vec::new()));
                        let lift_codes = Rc::new(RefCell::new(Vec::new()));
                        let script = Rc::new(RefCell::new(VecDeque::from([v1, v2])));
                        let want = lift.select_entry(
                            idx,
                            &mut || lift_order.borrow_mut().push('w'),
                            &mut || {
                                lift_order.borrow_mut().push('s');
                                script.borrow_mut().pop_front().unwrap()
                            },
                            &mut |code: u32| {
                                lift_order.borrow_mut().push('p');
                                lift_codes.borrow_mut().push(code);
                            },
                            &mut state,
                        );
                        let lift_order = lift_order.borrow().clone();
                        let lift_codes = lift_codes.borrow().clone();
                        if n == 0 {
                            assert_eq!(got, planted.table_addr(), "empty answers the table");
                            assert_eq!(want, None);
                            assert!(diff_words(&before, base).is_empty(), "empty writes");
                            assert!(rw_order.is_empty(), "empty calls");
                            assert_eq!(flags_rw, flags0);
                            assert_eq!(stamp_rw, stamp0);
                        } else {
                            assert_eq!(got, want.unwrap(), "idx={idx}");
                            // Only words whose value changes show up in the
                            // diff (re-selecting the index is a silent store).
                            let mut expected = Vec::new();
                            if bar.selected != idx {
                                expected.extend([0xa0usize, 0xf8]);
                            }
                            if bar.sel_mark != 0xffff_ffff {
                                expected.push(0xfc);
                            }
                            assert_eq!(diff_words(&before, base), expected);
                            assert_eq!(get_word(base, 0xa0), idx);
                            assert_eq!(get_word(base, 0xf8), idx);
                            assert_eq!(get_word(base, 0xfc), 0xffff_ffff);
                            assert_eq!(lift.selected, idx);
                            assert_eq!(lift.sel_mark, 0xffff_ffff);
                            assert_eq!(rw_order, lift_order, "call order");
                            assert_eq!(flags_rw, state.flags, "flag word");
                            assert_eq!(stamp_rw, state.stamp, "stamp word");
                            for &(tag, arg) in &rw_args {
                                if tag == 'w' {
                                    assert_eq!(arg, base, "watcher takes the bar");
                                }
                            }
                            assert_eq!(
                                rw_args
                                    .iter()
                                    .filter(|&&(t, _)| t == 'p')
                                    .map(|&(_, c)| c)
                                    .collect::<Vec<_>>(),
                                lift_codes,
                                "publish codes"
                            );
                        }
                        // The wrong lift publishes on equality.
                        if n > 0 {
                            let mut wbar = bar.clone();
                            let mut wstate = StampPublish {
                                flags: flags0,
                                stamp: stamp0,
                            };
                            let mut wcodes = Vec::new();
                            let wl = wrong::select_nonstrict(
                                &mut wbar,
                                idx,
                                v1,
                                v2,
                                &mut wstate,
                                &mut wcodes,
                            );
                            if wl != want || wcodes != lift_codes || wstate != state {
                                caught += 1;
                            }
                        }
                        cases += 1;
                        std::hint::black_box(&planted);
                    }
                }
            }
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong non-strict publish never caught ({cases} cases)"
        );
    }

    #[test]
    fn time_factor_matches() {
        let _guard = lock();
        rt::set_callee(1, sample_stub as usize as u32);
        let mut rng = Rng(0xF030);
        let mut cases = 0;
        let mut caught = 0;
        let modes = [0u32, 1, 0x100, 0x1ff, rng.u32(), rng.u32()];
        let ticks = int_corpus(&mut rng, 6);
        // The weight cycles the full edge corpus (NaN payloads observe
        // the multiply/add order), not just a few random floats.
        let weights = float_corpus(&mut Rng(0x0E16), 0);
        for trial in 0..32 {
            let mut bar = random_bar(&mut rng, 0);
            bar.weight = weights[trial % weights.len()];
            // Half the trials run with a null clock link.
            let null_link = trial % 2 == 1;
            let clock = if null_link {
                None
            } else {
                Some(FakeClock::new(&mut rng))
            };
            let link_addr = clock.as_ref().map_or(0, |c| c.link_addr());
            let mut bases = TimeBases {
                num_narrow: rng.u32(),
                num_wide: rng.u32(),
                den_narrow: rng.u32(),
                den_wide: rng.u32(),
            };
            if trial % 4 == 2 {
                // Zero numerators pin the divide-by-zero path.
                bases.num_narrow = 0;
                bases.num_wide = 0;
            }
            rt::set_global(NUM_NARROW_VA, bases.num_narrow);
            rt::set_global(NUM_WIDE_VA, bases.num_wide);
            let planted = plant_bar(&bar, link_addr, &mut rng);
            let base = planted.base();
            for &tick in &ticks {
                for &m1 in &modes {
                    for &m2 in &modes {
                        reset_stubs();
                        SCRIPT.lock().unwrap().extend([m1, m2]);
                        CLOCK_B.lock().unwrap().push_back(tick);
                        let before = snap(base, BAR_LEN);
                        let got = unsafe { fn_00D6F030::rw_00d6f030(base) };
                        assert!(diff_words(&before, base).is_empty(), "reader writes");
                        let rw_order = ORDER.lock().unwrap().clone();
                        let rw_args = ARGS.lock().unwrap().clone();
                        let w1 = m1 & 0xff != 0;
                        let w2 = m2 & 0xff != 0;
                        let wides = Rc::new(RefCell::new(VecDeque::from([w1, w2])));
                        let lift_order = Rc::new(RefCell::new(Vec::new()));
                        let mut lift = bar.clone();
                        lift.clock = ClockHandle::new(link_addr);
                        let ticked = Rc::new(RefCell::new(false));
                        let answer = lift.time_factor(
                            &mut LiftClock {
                                order: lift_order.clone(),
                                tick,
                                ticked: ticked.clone(),
                            },
                            &mut || {
                                lift_order.borrow_mut().push('s');
                                wides.borrow_mut().pop_front().unwrap()
                            },
                            &bases,
                        );
                        let lift_order = lift_order.borrow().clone();
                        let ticked = *ticked.borrow();
                        assert_eq!((answer as f64).to_bits(), got.to_bits(), "tick={tick:#x}");
                        if null_link {
                            assert!(rw_order.is_empty(), "null link calls");
                            assert!(lift_order.is_empty());
                            assert!(!ticked);
                        } else {
                            assert_eq!(rw_order, ['s', 'b', 's']);
                            assert_eq!(rw_order, lift_order, "call order");
                            assert_eq!(rw_args, [('b', clock.as_ref().unwrap().obj_addr())]);
                            assert!(ticked);
                        }
                        // The wrong lift subtracts from 16.
                        if !null_link {
                            let num1 = if w1 { bases.num_wide } else { bases.num_narrow };
                            let num2 = if w2 { bases.num_wide } else { bases.num_narrow };
                            if wrong::time_factor_16(&bar, tick, num1, num2).to_bits()
                                != answer.to_bits()
                            {
                                caught += 1;
                            }
                        }
                        cases += 1;
                    }
                }
            }
            std::hint::black_box((&planted, &clock));
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong tick base never caught ({cases} cases)");
    }

    /// Lift-side clock fake: records its slot and answers scripted ticks.
    struct LiftClock {
        order: Rc<RefCell<Vec<char>>>,
        tick: u32,
        ticked: Rc<RefCell<bool>>,
    }

    impl lf_files_memory::replay_bar::ClockRead for LiftClock {
        fn clock_a(&mut self) -> u32 {
            self.order.borrow_mut().push('a');
            0
        }
        fn clock_b(&mut self) -> u32 {
            self.order.borrow_mut().push('b');
            *self.ticked.borrow_mut() = true;
            self.tick
        }
    }

    #[test]
    fn blend_factors_matches() {
        let _guard = lock();
        rt::set_callee(3, sample_stub as usize as u32);
        let mut rng = Rng(0xF290);
        let mut cases = 0;
        let mut caught = 0;
        let modes = [0u32, 1, 0x100, rng.u32()];
        // Base pairs hitting every ratio branch: below 1, above 1, exactly
        // 1, infinite, NaN, and negative.
        let ratios: [(u32, u32); 7] = [
            (1, 2),
            (3, 2),
            (5, 5),
            (7, 0),
            (0, 0),
            (0x8000_0001, 2),
            (rng.u32(), rng.u32() | 1),
        ];
        // Signed edges for the clock readings (they convert as signed).
        let grid = [0u32, 1, 0xffff_ffff, 0x8000_0000, rng.u32(), rng.u32()];
        let scales = [
            0.0,
            1.0,
            -1.0,
            f32::INFINITY,
            f32::from_bits(0x7fc0_0001),
            f32::from_bits(1),
            rng.f32(),
            rng.f32(),
        ];
        for &(num, den) in &ratios {
            {
                let clock = FakeClock::new(&mut rng);
                let src_box = Box::new(clock.obj_addr());
                let src = addr(&*src_box);
                // Each pair doubles as both the wide and narrow words in
                // turn, so the selector side is pinned too.
                for narrow_wide in 0..2 {
                    let bases = if narrow_wide == 0 {
                        TimeBases {
                            num_narrow: num,
                            num_wide: num ^ 0x0f0f_0f0f,
                            den_narrow: den,
                            den_wide: den ^ 0xf0f0_f0f0,
                        }
                    } else {
                        TimeBases {
                            num_narrow: num ^ 0x0f0f_0f0f,
                            num_wide: num,
                            den_narrow: den ^ 0xf0f0_f0f0,
                            den_wide: den,
                        }
                    };
                    rt::set_global(NUM_NARROW_VA, bases.num_narrow);
                    rt::set_global(NUM_WIDE_VA, bases.num_wide);
                    rt::set_global(DEN_NARROW_VA, bases.den_narrow);
                    rt::set_global(DEN_WIDE_VA, bases.den_wide);
                    for &v1 in &grid {
                        for &v2 in &grid {
                            for &m1 in &modes {
                                for &m2 in &modes {
                                    for &sc in &scales {
                                        let first_box = Box::new(0xDEADu32);
                                        let second_box = Box::new(0xDEADu32);
                                        let blend_box = Box::new(0xDEADu32);
                                        let scale_box = Box::new(sc.to_bits());
                                        let (o_first, o_second, o_blend, o_scale) = (
                                            addr(&*first_box),
                                            addr(&*second_box),
                                            addr(&*blend_box),
                                            addr(&*scale_box),
                                        );
                                        reset_stubs();
                                        SCRIPT.lock().unwrap().extend([m1, m2]);
                                        CLOCK_A.lock().unwrap().push_back(v1);
                                        CLOCK_B.lock().unwrap().push_back(v2);
                                        let got = unsafe {
                                            fn_00D6F290::rw_00d6f290(
                                                src, o_first, o_second, o_blend, o_scale,
                                            )
                                        };
                                        assert_eq!(got, o_scale, "the scale pointer is pinned");
                                        let rw_order = ORDER.lock().unwrap().clone();
                                        let rw_args = ARGS.lock().unwrap().clone();
                                        assert_eq!(rw_order, ['a', 'b', 's', 's']);
                                        assert_eq!(
                                            rw_args,
                                            [('a', clock.obj_addr()), ('b', clock.obj_addr())]
                                        );
                                        let w1 = m1 & 0xff != 0;
                                        let w2 = m2 & 0xff != 0;
                                        let wides = Rc::new(RefCell::new(VecDeque::from([w1, w2])));
                                        let lift_order = Rc::new(RefCell::new(Vec::new()));
                                        let mut lift_scale = sc;
                                        let lift = blend_factors(
                                            &mut BlendClock {
                                                order: lift_order.clone(),
                                                a: v1,
                                                b: v2,
                                            },
                                            &mut lift_scale,
                                            &bases,
                                            &mut || {
                                                lift_order.borrow_mut().push('s');
                                                wides.borrow_mut().pop_front().unwrap()
                                            },
                                        );
                                        let lift_order = lift_order.borrow().clone();
                                        assert_eq!(rw_order, lift_order, "call order");
                                        let ctx = format!(
                                            "num={num:#x} den={den:#x} v1={v1:#x} v2={v2:#x} \
                                             m1={m1:#x} m2={m2:#x} sc={sc:?} narrow_wide={narrow_wide}"
                                        );
                                        assert_eq!(get_word(o_first, 0), lift.0.to_bits(), "{ctx}");
                                        assert_eq!(
                                            get_word(o_second, 0),
                                            lift.1.to_bits(),
                                            "{ctx}"
                                        );
                                        assert_eq!(get_word(o_blend, 0), lift.2.to_bits(), "{ctx}");
                                        assert_eq!(
                                            get_word(o_scale, 0),
                                            lift_scale.to_bits(),
                                            "{ctx}"
                                        );
                                        // The wrong lift flips the branch.
                                        let num =
                                            if w1 { bases.num_wide } else { bases.num_narrow };
                                        let den =
                                            if w2 { bases.den_wide } else { bases.den_narrow };
                                        let ratio =
                                            (num.cast_signed() as f32) / (den.cast_signed() as f32);
                                        let f1 = v1.cast_signed() as f32;
                                        let f2 = v2.cast_signed() as f32;
                                        let mut wscale = sc;
                                        let wl = wrong::blend_flipped(f1, f2, &mut wscale, ratio);
                                        if wl != lift || wscale.to_bits() != lift_scale.to_bits() {
                                            caught += 1;
                                        }
                                        cases += 1;
                                        std::hint::black_box((
                                            &first_box,
                                            &second_box,
                                            &blend_box,
                                            &scale_box,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                std::hint::black_box((&clock, &src_box));
            }
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong flipped branch never caught ({cases} cases)"
        );
    }

    /// Lift-side clock fake for the blend: answers both readings in order.
    struct BlendClock {
        order: Rc<RefCell<Vec<char>>>,
        a: u32,
        b: u32,
    }

    impl lf_files_memory::replay_bar::ClockRead for BlendClock {
        fn clock_a(&mut self) -> u32 {
            self.order.borrow_mut().push('a');
            self.a
        }
        fn clock_b(&mut self) -> u32 {
            self.order.borrow_mut().push('b');
            self.b
        }
    }

    #[test]
    fn maybe_notify_matches() {
        let _guard = lock();
        rt::set_callee(1, sample_stub as usize as u32);
        rt::set_callee(2, refresh_stub as usize as u32);
        rt::set_callee(3, hub_stub as usize as u32);
        let mut rng = Rng(0xF510);
        let mut cases = 0;
        let mut caught = 0;
        let states = [
            0u32,
            1,
            2,
            3,
            7,
            8,
            9,
            10,
            11,
            12,
            16,
            17,
            18,
            19,
            100,
            u32::MAX,
            rng.u32(),
        ];
        let bounds = int_corpus(&mut rng, 6);
        for &state in &states {
            for &bound in &bounds {
                // Probes below, at, and above the bound.
                for &t in &[
                    bound.wrapping_sub(1),
                    bound,
                    bound.wrapping_add(1),
                    rng.u32(),
                ] {
                    // Notifier answers: null first, or two live fetches.
                    for null_first in [false, true] {
                        let inner_box = Box::new(rng.u32());
                        let inner = addr(&*inner_box);
                        let mut ctl_img = vec![0u8; 8];
                        rng.bytes(&mut ctl_img);
                        ctl_img[4..8].copy_from_slice(&inner.to_le_bytes());
                        let ctl_box = ctl_img.into_boxed_slice();
                        let ctl = addr(&ctl_box[0]);
                        let hub_box = Box::new(rng.u32());
                        let hub = addr(&*hub_box);
                        rt::set_relocated(HUB_VA, hub);
                        rt::set_global(STATE_VA, state);
                        let mut note = vec![0u8; 0x39c];
                        rng.bytes(&mut note);
                        // Half the trials start with the flag set.
                        if bound & 1 == 0 {
                            note[0x398] |= 1;
                        } else {
                            note[0x398] &= 0xfe;
                        }
                        let flag0 = note[0x398];
                        let note_box = note.into_boxed_slice();
                        let note_addr = addr(&note_box[0]);
                        let (p, q) = if null_first {
                            (0, note_addr)
                        } else {
                            (note_addr, note_addr)
                        };
                        reset_stubs();
                        SCRIPT.lock().unwrap().extend([t, p, q]);
                        let ctl_before = Vec::from(&ctl_box[..]);
                        let got = unsafe { fn_00D6F510::rw_00d6f510(ctl, bound) };
                        let rw_order = ORDER.lock().unwrap().clone();
                        let rw_args = ARGS.lock().unwrap().clone();
                        let flag_rw =
                            unsafe { (note_addr.wrapping_add(0x398) as *const u8).read() };
                        // The lift side replays the same script.
                        let ctl_lift = NotifyCtl {
                            inner: InnerHandle::new(inner).unwrap(),
                        };
                        let lift_order = Rc::new(RefCell::new(Vec::new()));
                        let script = Rc::new(RefCell::new(VecDeque::from([t, p, q])));
                        let flag_lift = Rc::new(RefCell::new(flag0));
                        let mut hub_ids = Vec::new();
                        let want = ctl_lift.maybe_notify(
                            bound,
                            state,
                            &mut || {
                                lift_order.borrow_mut().push('s');
                                script.borrow_mut().pop_front().unwrap()
                            },
                            &mut || lift_order.borrow_mut().push('r'),
                            &mut LiftHub {
                                order: lift_order.clone(),
                                script: script.clone(),
                                ids: &mut hub_ids,
                                flag: flag_lift.clone(),
                            },
                        );
                        let lift_order = lift_order.borrow().clone();
                        let flag_lift = *flag_lift.borrow();
                        // The control image never changes.
                        let ctl_after =
                            unsafe { std::slice::from_raw_parts(ctl as *const u8, 8).to_vec() };
                        assert_eq!(ctl_after, ctl_before, "the control is untouched");
                        match want {
                            RefreshOutcome::BelowBound(v) => {
                                assert_eq!(got, v);
                                assert_eq!(rw_order, ['s']);
                            }
                            RefreshOutcome::Suppressed(s) => {
                                assert_eq!(got, s);
                                assert_eq!(rw_order, ['s']);
                            }
                            RefreshOutcome::NoNotifier => {
                                assert_eq!(got, 0);
                                assert_eq!(rw_order, ['s', 'r', 'h']);
                                assert_eq!(flag_rw, flag0, "no fetch, no clear");
                            }
                            RefreshOutcome::Refreshed(id) => {
                                assert_eq!(got, id.get());
                                assert_eq!(got, note_addr, "the address is rebuilt");
                                assert_eq!(rw_order, ['s', 'r', 'h', 'h']);
                                assert_eq!(flag_rw, flag0 & 0xfe, "the flag bit clears");
                                assert_eq!(flag_lift, flag0 & 0xfe);
                            }
                        }
                        assert_eq!(rw_order, lift_order, "call order");
                        for &(tag, arg) in &rw_args {
                            match tag {
                                'r' => assert_eq!(arg, inner, "refresh takes inner"),
                                'h' => assert_eq!(arg, hub, "fetch takes the hub"),
                                _ => {}
                            }
                        }
                        // The wrong lift suppresses state 18 as well.
                        if t >= bound
                            && wrong::suppressed_wide(state)
                                != matches!(
                                    state,
                                    2 | 7 | 8 | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11
                                )
                        {
                            caught += 1;
                        }
                        cases += 1;
                        std::hint::black_box((&ctl_box, &hub_box, &inner_box, &note_box));
                    }
                }
            }
        }
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong suppressed set never caught ({cases} cases)"
        );
    }

    /// Lift-side hub fake: scripted lookups plus the flag cell.
    struct LiftHub<'a> {
        order: Rc<RefCell<Vec<char>>>,
        script: Rc<RefCell<VecDeque<u32>>>,
        ids: &'a mut Vec<u32>,
        flag: Rc<RefCell<u8>>,
    }

    impl lf_files_memory::replay_bar::NotifyHub for LiftHub<'_> {
        fn lookup(&mut self) -> Option<NotifierHandle> {
            self.order.borrow_mut().push('h');
            let addr = self.script.borrow_mut().pop_front().unwrap();
            self.ids.push(addr);
            NotifierHandle::new(addr)
        }

        fn clear_suppress(&mut self, _id: NotifierHandle) {
            *self.flag.borrow_mut() &= 0xfe;
        }
    }
}
