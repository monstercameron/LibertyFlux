//! Differential cases: the lifted shaft channel steps against their verified
//! 32-bit rewrites.
//!
//! Each case builds one shaft object (calibration, table, flag and pending
//! bytes, noise in the bytes nothing reads) and runs a sequence of channel
//! steps two ways: through the verified rewrite on the object's bytes, and
//! through the lifted methods on a lifted object. After every step the
//! answer, the words the step writes for its channel, the pending byte and
//! every radicand the square-root helper is asked for are compared, floats
//! bit for bit. A second writing of the steps with one deliberate fault
//! each must be caught on every channel. The cases run on the 32-bit target
//! only.

#[cfg(target_arch = "x86")]
#[path = "support/reference.rs"]
mod reference;

#[cfg(target_arch = "x86")]
mod x86 {
    use std::cell::RefCell;

    use lf_math::shaft::{Calibration, Channel, Shaft, ShaftHost};
    use lf_shaftdiff::rewrites::{fn_00656340, fn_00656680, fn_006569D0, fn_00656D40};
    use lf_shaftdiff::rt::set_callee1;

    use super::reference::{self, Mutant, WRONG};

    /// Size of the test object: the rewrites touch words up to offset 0x20F.
    const OBJ_BYTES: usize = 0x300;
    /// Gain word.
    const GAIN: usize = 0x130;
    /// Scale word of each calibration pair.
    const SCALE: [usize; 2] = [0x138, 0x13C];
    /// Divisor word of each calibration pair.
    const DIVISOR: [usize; 2] = [0x148, 0x14C];
    /// Seeds-fixed byte.
    const FLAG: usize = 0x2C2;
    /// Pending byte, cleared by every step.
    const PENDING: usize = 0x2C0;
    /// Number of generated objects in the main case set.
    const CASES: u32 = 20000;
    /// Number of generated objects per channel in the fault set.
    const FAULT_CASES: u32 = 1500;

    /// Offset of a channel's stored step length.
    fn input_at(ch: usize) -> usize {
        0x10 + 4 * ch
    }

    /// Offset of a channel's scratch block (seeds, residual, mixed, total).
    fn base(ch: usize) -> usize {
        0x170 + 0x10 * ch
    }

    /// Offset of a table word: four rows of three columns.
    fn table_at(row: usize, col: usize) -> usize {
        0xE0 + 0x10 * row + 4 * col
    }

    /// The verified rewrite of channel `ch`, called as the original's
    /// step method is called: the object address and the step length bits.
    fn rewrite(ch: usize, this: u32, dt_bits: u32) -> u32 {
        match ch {
            0 => fn_00656340::rw_00656340(this, dt_bits),
            1 => fn_00656680::rw_00656680(this, dt_bits),
            2 => fn_006569D0::rw_006569D0(this, dt_bits),
            _ => fn_00656D40::rw_00656D40(this, dt_bits),
        }
    }

    thread_local! {
        /// Radicands the rewrite's square-root callee was asked for, in order.
        static ROOT_CALLS: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
    }

    /// Callee 1 for the rewrites: records the radicand and answers the host
    /// square root, the same operation the lifted host uses.
    extern "cdecl" fn root_stub(bits: u32) -> u32 {
        ROOT_CALLS.with(|calls| calls.borrow_mut().push(bits));
        f32::from_bits(bits).sqrt().to_bits()
    }

    /// The lifted side's square-root host: records each radicand.
    #[derive(Default)]
    struct Recorder {
        calls: Vec<u32>,
    }

    impl ShaftHost for Recorder {
        fn root(&mut self, radicand: f32) -> f32 {
            self.calls.push(radicand.to_bits());
            radicand.sqrt()
        }
    }

    /// The test's shaft object, as the rewrites see it: a byte array with the
    /// words and bytes at the original's offsets.
    #[derive(Clone)]
    #[repr(C, align(8))]
    struct Obj([u8; OBJ_BYTES]);

    impl Obj {
        fn word(&self, off: usize) -> u32 {
            u32::from_le_bytes([
                self.0[off],
                self.0[off + 1],
                self.0[off + 2],
                self.0[off + 3],
            ])
        }

        fn float(&self, off: usize) -> f32 {
            f32::from_bits(self.word(off))
        }

        fn set_word(&mut self, off: usize, value: u32) {
            self.0[off..off + 4].copy_from_slice(&value.to_le_bytes());
        }

        fn set_float(&mut self, off: usize, value: f32) {
            self.set_word(off, value.to_bits());
        }

        fn byte(&self, off: usize) -> u8 {
            self.0[off]
        }
    }

    /// Small deterministic generator (splitmix64).
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }

    /// Edge values: zeros, ones, the calibration's usual magnitudes, the
    /// extremes, infinities and not-a-number values with two payloads and
    /// both signs.
    const EDGE: [f32; 24] = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        -0.5,
        2.0,
        2.5,
        0.25,
        f32::MIN_POSITIVE,
        f32::MIN,
        f32::MAX,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::from_bits(0x7FC0_1234),
        f32::from_bits(0xFFC0_1234),
        f32::from_bits(0x7F80_0001),
        f32::from_bits(0x0000_0001),
        100.0,
        1.0e-3,
        8_388_608.0,
        -3.0,
        0.1,
    ];

    /// One value: edge values, random bit patterns, small exact values, or a
    /// step-sized fraction.
    fn pick(rng: &mut Rng) -> f32 {
        match rng.below(8) {
            0..=2 => EDGE[rng.below(EDGE.len() as u64) as usize],
            3 => f32::from_bits(rng.next() as u32),
            4 | 5 => (rng.below(33) as f32 - 16.0) * 0.25,
            _ => rng.below(1000) as f32 / 1000.0,
        }
    }

    /// A value for a divisor: usually not zero, so the offsets stay finite.
    fn divisor(rng: &mut Rng) -> f32 {
        if rng.below(4) == 0 {
            pick(rng)
        } else {
            let v = pick(rng);
            if v == 0.0 { 1.0 } else { v }
        }
    }

    /// A shaft object: calibration, table and flags drawn at random, with
    /// noise in every byte before them.
    fn config(rng: &mut Rng) -> Obj {
        let mut obj = Obj([0; OBJ_BYTES]);
        for byte in obj.0.iter_mut() {
            *byte = rng.next() as u8;
        }
        obj.set_float(GAIN, pick(rng));
        for pair in 0..2 {
            obj.set_float(SCALE[pair], pick(rng));
            obj.set_float(DIVISOR[pair], divisor(rng));
        }
        for row in 0..4 {
            for col in 0..3 {
                obj.set_float(table_at(row, col), pick(rng));
            }
        }
        obj.0[FLAG] = [0, 0, 1, 0x80][rng.below(4) as usize];
        obj.0[PENDING] = rng.next() as u8;
        obj
    }

    /// The lifted object that starts from the same bytes as the rewrite.
    fn lifted(obj: &Obj) -> Shaft {
        let mut table = [[0.0f32; 3]; 4];
        for (row, values) in table.iter_mut().enumerate() {
            for (col, value) in values.iter_mut().enumerate() {
                *value = obj.float(table_at(row, col));
            }
        }
        Shaft {
            gain: obj.float(GAIN),
            calibration: [
                Calibration {
                    scale: obj.float(SCALE[0]),
                    divisor: obj.float(DIVISOR[0]),
                },
                Calibration {
                    scale: obj.float(SCALE[1]),
                    divisor: obj.float(DIVISOR[1]),
                },
            ],
            table,
            seeds_fixed: obj.byte(FLAG) != 0,
            pending: obj.byte(PENDING) != 0,
            channels: [Channel::default(); 4],
        }
    }

    /// The words the rewrite leaves for one channel, in comparison order:
    /// stored length, seeds, residual, mixed values, total.
    fn channel_words(obj: &Obj, ch: usize) -> Vec<(&'static str, u32)> {
        let b = base(ch);
        vec![
            ("input", obj.word(input_at(ch))),
            ("seed0", obj.word(b)),
            ("seed1", obj.word(b + 4)),
            ("seed2", obj.word(b + 8)),
            ("residual", obj.word(b + 0xC)),
            ("mixed0", obj.word(b + 0x60)),
            ("mixed1", obj.word(b + 0x64)),
            ("mixed2", obj.word(b + 0x68)),
            ("total", obj.word(b + 0x6C)),
        ]
    }

    /// The lifted channel's words in the same order as [`channel_words`].
    fn lifted_words(ch: &Channel) -> Vec<u32> {
        vec![
            ch.input.to_bits(),
            ch.seeds[0].to_bits(),
            ch.seeds[1].to_bits(),
            ch.seeds[2].to_bits(),
            ch.residual.to_bits(),
            ch.mixed[0].to_bits(),
            ch.mixed[1].to_bits(),
            ch.mixed[2].to_bits(),
            ch.total.to_bits(),
        ]
    }

    /// Which side of the comparison runs the steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Engine {
        /// The lifted methods.
        Lift,
        /// A second writing of the steps, with the given deliberate fault.
        Writing(Mutant),
    }

    /// What the cases saw, so a run shows what each branch was exercised.
    #[derive(Default, Debug)]
    struct Tally {
        steps: u64,
        fixed: u64,
        rooted: u64,
        zero_length: u64,
        nan_seed: u64,
    }

    /// Runs one step sequence on both sides. Returns the first disagreement.
    fn run_sequence(
        start: &Obj,
        steps: &[(usize, f32)],
        engine: Engine,
        tally: &mut Tally,
    ) -> Option<String> {
        let mut rewritten = start.clone();
        let mut expected = start.clone();
        let mut lift = lifted(start);
        for (n, &(ch, dt)) in steps.iter().enumerate() {
            tally.steps += 1;
            let fixed = lift.seeds_fixed;
            ROOT_CALLS.with(|calls| calls.borrow_mut().clear());
            let rw_this = (&mut rewritten as *mut Obj) as usize as u32;
            let rw_ret = rewrite(ch, rw_this, dt.to_bits());
            let rw_roots: Vec<u32> = ROOT_CALLS.with(|calls| calls.borrow().clone());
            let rw_words = channel_words(&rewritten, ch);
            let rw_pending = rewritten.byte(PENDING);
            let rw_seeds_nan = (0..3).any(|k| f32::from_bits(rewritten.word(base(ch) + 4 * k)).is_nan());
            if fixed {
                tally.fixed += 1;
            } else if rw_roots.len() == 1 {
                tally.rooted += 1;
            } else if rw_roots.is_empty() {
                tally.zero_length += 1;
            }
            if rw_seeds_nan {
                tally.nan_seed += 1;
            }

            let mut host = Recorder::default();
            let (got_ret, got_channel, got_pending) = match engine {
                Engine::Lift => {
                    let ret = match ch {
                        0 => lift.advance_0(&mut host, dt),
                        1 => lift.advance_1(&mut host, dt),
                        2 => lift.advance_2(&mut host, dt),
                        _ => lift.advance_3(&mut host, dt),
                    };
                    (ret, lift.channels[ch], lift.pending)
                }
                Engine::Writing(m) => {
                    let out = reference::step(m, ch, &lift, &mut host, dt);
                    lift.channels[ch] = reference::to_channel(&out, dt);
                    lift.pending = false;
                    (out.ret, lift.channels[ch], lift.pending)
                }
            };

            let mut mismatch = None;
            let mut check = |what: &str, want: u32, got: u32| {
                if mismatch.is_none() && want != got {
                    mismatch = Some(format!(
                        "step {n} (channel {ch}, dt {:#010x}): {what} rewrite {want:#010x} lift {got:#010x}",
                        dt.to_bits()
                    ));
                }
            };
            check("answer", rw_ret, got_ret.to_bits());
            check("pending byte", u32::from(rw_pending), u32::from(got_pending));
            let got_words = lifted_words(&got_channel);
            for (k, (name, want)) in rw_words.iter().enumerate() {
                check(name, *want, got_words[k]);
            }
            if mismatch.is_none() && rw_roots != host.calls {
                mismatch = Some(format!(
                    "step {n} (channel {ch}): root calls rewrite {rw_roots:x?} lift {:x?}",
                    host.calls
                ));
            }
            if mismatch.is_some() {
                return mismatch;
            }
            // Every other byte of the object: the rewrite may write only the
            // words the lifted step stores and the pending byte.
            expected.set_word(input_at(ch), got_channel.input.to_bits());
            for (k, word) in lifted_words(&got_channel).iter().enumerate().skip(1) {
                let off = match k {
                    1..=3 => base(ch) + 4 * (k - 1),
                    4 => base(ch) + 0xC,
                    5..=7 => base(ch) + 0x60 + 4 * (k - 5),
                    _ => base(ch) + 0x6C,
                };
                expected.set_word(off, *word);
            }
            expected.0[PENDING] = u8::from(got_pending);
            if rewritten.0 != expected.0 {
                let at = (0..OBJ_BYTES)
                    .find(|&i| rewritten.0[i] != expected.0[i])
                    .unwrap_or(0);
                return Some(format!(
                    "step {n} (channel {ch}): object byte {at:#x} differs from the lifted step's effects"
                ));
            }
        }
        None
    }

    /// A sequence of steps: each channel once, then random channels.
    fn sequence(rng: &mut Rng) -> Vec<(usize, f32)> {
        let len = 4 + rng.below(9) as usize;
        (0..len)
            .map(|i| {
                let ch = if i < 4 { i } else { rng.below(4) as usize };
                (ch, pick(rng))
            })
            .collect()
    }

    fn setup() {
        set_callee1(root_stub as *const () as usize as u32);
    }

    #[test]
    fn lifted_steps_match_verified_rewrites() {
        setup();
        let mut rng = Rng(0x5AF7_0001);
        let mut tally = Tally::default();
        for case in 0..CASES {
            let cfg = config(&mut rng);
            let steps = sequence(&mut rng);
            if let Some(msg) = run_sequence(&cfg, &steps, Engine::Lift, &mut tally) {
                panic!("case {case}: {msg}");
            }
        }
        println!("lifted cases {CASES}: {tally:?}");
        assert!(tally.fixed > 0, "no fixed-seed step ran");
        assert!(tally.rooted > 0, "no root-taking step ran");
        assert!(tally.zero_length > 0, "no zero-length step ran");
        assert!(tally.nan_seed > 0, "no not-a-number seed ran");
    }

    #[test]
    fn writing_without_a_fault_agrees_with_the_rewrites() {
        setup();
        let mut rng = Rng(0x5AF7_0002);
        let mut tally = Tally::default();
        for case in 0..CASES {
            let cfg = config(&mut rng);
            let steps = sequence(&mut rng);
            if let Some(msg) = run_sequence(&cfg, &steps, Engine::Writing(Mutant::None), &mut tally) {
                panic!("control case {case}: {msg}");
            }
        }
    }

    /// Each deliberate fault, run on every channel: at least one generated
    /// case must show a difference, or the cases cannot see that fault.
    #[test]
    fn deliberate_faults_are_caught_on_every_channel() {
        setup();
        let mut rng = Rng(0x5AF7_0003);
        let mut lines = Vec::new();
        let mut missed = Vec::new();
        for &mutant in &WRONG {
            let mut row = Vec::new();
            for ch in 0..4 {
                let mut caught = 0u32;
                let mut tally = Tally::default();
                for _ in 0..FAULT_CASES {
                    let cfg = config(&mut rng);
                    let steps: Vec<(usize, f32)> = (0..6).map(|_| (ch, pick(&mut rng))).collect();
                    if run_sequence(&cfg, &steps, Engine::Writing(mutant), &mut tally).is_some() {
                        caught += 1;
                    }
                }
                row.push(caught);
            }
            lines.push(format!("{mutant:?}: caught per channel {row:?} of {FAULT_CASES}"));
            for (ch, caught) in row.iter().enumerate() {
                if *caught == 0 {
                    missed.push(format!("{mutant:?} on channel {ch}"));
                }
            }
        }
        for line in &lines {
            println!("{line}");
        }
        assert!(missed.is_empty(), "faults never caught: {missed:?}");
    }
}
