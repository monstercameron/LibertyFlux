//! Differential cases for the token follow-up methods of the layout frame.
//!
//! Each case runs a lifted method and its verified rewrite on the same
//! generated frames. The rewrite calls its callees and virtual steps through
//! the checker runtime's callee table and the vtables it reads from the
//! frame and the target; the harness points those at recording stubs that
//! answer from a script. The lifted method calls the same steps through a
//! recording host. Both sides must produce the same answer, the same sequence
//! of calls with the same arguments, and the same bytes in the frame. A
//! deliberately wrong lift of each method must differ from the rewrite.
//!
//! The stubs write nothing into the frame: a callee that wrote into the frame
//! would need its writes scripted, and none of these three does. The image
//! base is not used by these methods.
//!
//! The checker runtime's callee table and image base are statics the checker
//! worker sets before a trial; the harness sets them once, in the one test
//! here, and the cases run one after another.

// Writes the checker runtime's callee table static, which the checker worker sets before each trial in the real checker. Test-only.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_checker_rt::CHECKER_CTABLE;
    use lf_input_frontend::layout_frame::follow_up::{FollowUpHost, Target, TargetStep};
    use lf_input_frontend::layout_frame::{SPAN, UILayoutFrame, field};
    use lf_layoutframe_diff::rewrites as r;

    /// Bytes of the buffer each rewrite runs on: the frame's span and a tail
    /// that must come back unchanged.
    const BUF: usize = 0x100;

    /// Generated cases per method.
    const TRIALS: u32 = 2000;

    /// Answers the recording stubs give, in the order the calls happen.
    const SCRIPT_LEN: usize = 8;

    /// Words in a fake vtable: enough for the highest slot the family reads.
    const VTABLE_WORDS: usize = 0x200 / 4;

    /// Words in the checker runtime's callee table.
    const CALLEE_TABLE_WORDS: usize = 256;

    /// Byte offsets of the frame's virtual steps and the target's steps.
    const FRAME_FIRST_STEP_AT: usize = 0x14;
    const FRAME_SECOND_STEP_AT: usize = 0x18;
    const FRAME_STEP_AT: usize = 0x4C;
    const TARGET_STEP_107_108_AT: usize = 0x16C;
    const TARGET_STEP_111_AT: usize = 0x178;

    /// Callee ids the rewrites call.
    const CALLEE_RESOLVE: usize = 1;

    /// Tag numbers for the target's steps in the recorded log. The lift's
    /// [`TargetStep`] values map to the same tags, so a call through the
    /// wrong step shows in the log.
    const TAG_107_108: u8 = 1;
    const TAG_111: u8 = 2;

    fn step_tag(step: TargetStep) -> u8 {
        match step {
            TargetStep::Sibling107And108 => TAG_107_108,
            TargetStep::Sibling111 => TAG_111,
        }
    }

    /// The buffer the rewrites write into, 4-byte aligned like the object.
    #[repr(C, align(4))]
    struct Buffer([u8; BUF]);

    /// One call, as the log records it. The receiver of a frame step is not
    /// recorded: the lift passes the frame it owns, the rewrite its buffer,
    /// and those addresses differ by construction.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Ev {
        Resolve(u32),
        FrameStep,
        TargetStep { target: u32, tag: u8, arg: u32 },
        FrameFirst(u32),
        FrameSecond(u32),
    }

    /// Records calls and answers them from a script. The same type serves
    /// the rewrite side (as the global behind the stubs) and the lift side
    /// (as the host passed to the lifted method).
    struct Recorder {
        script: [u32; SCRIPT_LEN],
        cursor: usize,
        log: Vec<Ev>,
        target: u32,
    }

    impl Recorder {
        const fn empty() -> Self {
            Self { script: [0; SCRIPT_LEN], cursor: 0, log: Vec::new(), target: 0 }
        }

        fn reset(&mut self, script: [u32; SCRIPT_LEN], target: u32) {
            self.script = script;
            self.cursor = 0;
            self.log.clear();
            self.target = target;
        }

        fn answer(&mut self) -> u32 {
            let value = self.script[self.cursor % SCRIPT_LEN];
            self.cursor += 1;
            value
        }

        fn on_resolve(&mut self, token: u32) -> u32 {
            self.log.push(Ev::Resolve(token));
            self.target
        }

        fn on_frame_step(&mut self) -> u32 {
            self.log.push(Ev::FrameStep);
            self.answer()
        }

        fn on_target_step(&mut self, target: u32, tag: u8, arg: u32) -> u32 {
            self.log.push(Ev::TargetStep { target, tag, arg });
            self.answer()
        }

        fn on_frame_first(&mut self, arg: u32) -> u32 {
            self.log.push(Ev::FrameFirst(arg));
            self.answer()
        }

        fn on_frame_second(&mut self, arg: u32) -> u32 {
            self.log.push(Ev::FrameSecond(arg));
            self.answer()
        }
    }

    impl FollowUpHost for Recorder {
        fn resolve_token(&mut self, token: u32) -> Target {
            Target(self.on_resolve(token))
        }

        fn frame_step(&mut self, _frame: &mut UILayoutFrame) -> u32 {
            self.on_frame_step()
        }

        fn target_step(&mut self, target: Target, step: TargetStep, arg: u32) -> u32 {
            self.on_target_step(target.0, step_tag(step), arg)
        }

        fn frame_first_step(&mut self, _frame: &mut UILayoutFrame, arg: u32) -> u32 {
            self.on_frame_first(arg)
        }

        fn frame_second_step(&mut self, _frame: &mut UILayoutFrame, arg: u32) -> u32 {
            self.on_frame_second(arg)
        }
    }

    /// The recorder the rewrite's stubs write to. Only the rewrite runs use
    /// it, one at a time.
    static REC: Mutex<Recorder> = Mutex::new(Recorder::empty());

    extern "cdecl" fn resolve_stub(token: u32) -> u32 {
        REC.lock().unwrap().on_resolve(token)
    }

    extern "thiscall" fn frame_step_stub(_this: u32) -> u32 {
        REC.lock().unwrap().on_frame_step()
    }

    extern "thiscall" fn frame_first_stub(_this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_frame_first(arg)
    }

    extern "thiscall" fn frame_second_stub(_this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_frame_second(arg)
    }

    extern "thiscall" fn target_107_108_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_107_108, arg)
    }

    extern "thiscall" fn target_111_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_111, arg)
    }

    fn word_of(bytes: &[u8], at: usize) -> u32 {
        let mut word = [0u8; 4];
        word.copy_from_slice(&bytes[at..at + 4]);
        u32::from_le_bytes(word)
    }

    /// The fake objects the stubs are reached through: the frame's vtable,
    /// the target's vtable, and the target itself, each leaked for the run.
    struct Objects {
        frame_vtable: u32,
        target: u32,
    }

    /// Builds the fake objects and points the callee table at the stubs.
    fn install() -> Objects {
        let frame_vtable: &'static mut [u32; VTABLE_WORDS] =
            Box::leak(Box::new([0u32; VTABLE_WORDS]));
        frame_vtable[FRAME_STEP_AT / 4] = (frame_step_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        frame_vtable[FRAME_FIRST_STEP_AT / 4] =
            (frame_first_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;
        frame_vtable[FRAME_SECOND_STEP_AT / 4] =
            (frame_second_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;

        let target_vtable: &'static mut [u32; VTABLE_WORDS] =
            Box::leak(Box::new([0u32; VTABLE_WORDS]));
        target_vtable[TARGET_STEP_107_108_AT / 4] =
            (target_107_108_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;
        target_vtable[TARGET_STEP_111_AT / 4] =
            (target_111_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;

        let target_object: &'static mut [u32; 4] =
            Box::leak(Box::new([target_vtable.as_ptr() as usize as u32, 0, 0, 0]));
        let target = target_object.as_ptr() as usize as u32;

        let callee_table: &'static mut [u32; CALLEE_TABLE_WORDS] =
            Box::leak(Box::new([0u32; CALLEE_TABLE_WORDS]));
        callee_table[CALLEE_RESOLVE] = (resolve_stub as extern "cdecl" fn(u32) -> u32) as usize as u32;
        // SAFETY: the table lives for the rest of the run and every entry the
        // rewrites call is set above; the checker worker keeps the same contract.
        unsafe {
            core::ptr::addr_of_mut!(CHECKER_CTABLE).write(callee_table.as_ptr());
        }

        Objects { frame_vtable: frame_vtable.as_ptr() as usize as u32, target }
    }

    /// Arguments of one trial: the frame bytes, the script of answers and the
    /// token word, as the trial draws them.
    struct Trial {
        frame: [u8; SPAN],
        script: [u32; SCRIPT_LEN],
    }

    /// One method under test: its rewrite, its lift, and a deliberately wrong
    /// lift. Each takes the frame and the host, and answers the method's answer.
    struct Case {
        name: &'static str,
        token_at: usize,
        rw: fn(u32) -> u32,
        lift: fn(&mut UILayoutFrame, &mut Recorder) -> u32,
        wrong: fn(&mut UILayoutFrame, &mut Recorder) -> u32,
    }

    /// Wrong lift of the enable sibling: the second enable runs before the
    /// first, and the first's answer is returned.
    fn wrong_107(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let token = word_of(frame.bytes(), field::WORD_D4);
        if token != 0 {
            let target = host.resolve_token(token);
            let arg = host.frame_step(frame);
            host.target_step(target, TargetStep::Sibling107And108, arg);
        }
        host.frame_second_step(frame, 1);
        host.frame_first_step(frame, 1)
    }

    /// Wrong lift of the reset sibling: the reset uses an argument of one.
    fn wrong_108(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let token = word_of(frame.bytes(), field::WORD_D4);
        if token != 0 {
            let target = host.resolve_token(token);
            let arg = host.frame_step(frame);
            host.target_step(target, TargetStep::Sibling107And108, arg);
        }
        host.frame_second_step(frame, 1)
    }

    /// Wrong lift of the answering sibling: the target's step gets zero in
    /// place of the frame's answer.
    fn wrong_111(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let token = word_of(frame.bytes(), field::WORD_D8);
        if token == 0 {
            return 0;
        }
        let target = host.resolve_token(token);
        let _ignored = host.frame_step(frame);
        host.target_step(target, TargetStep::Sibling111, 0)
    }

    fn cases() -> Vec<Case> {
        vec![
            Case {
                name: "token_follow_up_then_two_enables",
                token_at: field::WORD_D4,
                rw: |t| r::fn_00DB16B0::rw_00db16b0(t),
                lift: |f, h| f.token_follow_up_then_two_enables(h),
                wrong: wrong_107,
            },
            Case {
                name: "token_follow_up_then_reset",
                token_at: field::WORD_D4,
                rw: |t| r::fn_00DB1700::rw_00db1700(t),
                lift: |f, h| f.token_follow_up_then_reset(h),
                wrong: wrong_108,
            },
            Case {
                name: "token_follow_up_when_nonzero",
                token_at: field::WORD_D8,
                rw: |t| r::fn_00DB1890::rw_00db1890(t),
                lift: |f, h| f.token_follow_up_when_nonzero(h),
                wrong: wrong_111,
            },
        ]
    }

    /// Small deterministic generator (splitmix64), seeded per method.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn u32(&mut self) -> u32 {
            (self.next() >> 32) as u32
        }
    }

    fn seed_of(name: &str) -> u64 {
        let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
        for byte in name.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    /// Draws one trial. The token is zero on a third of the trials, so the
    /// empty branch runs; otherwise it is non-zero.
    fn draw(rng: &mut Rng, objects: &Objects, token_at: usize) -> Trial {
        let mut frame = [0u8; SPAN];
        for byte in &mut frame {
            *byte = rng.next() as u8;
        }
        frame[0..4].copy_from_slice(&objects.frame_vtable.to_le_bytes());
        let token = if rng.next() % 3 == 0 {
            0
        } else {
            match rng.u32() {
                0 => 1,
                value => value,
            }
        };
        frame[token_at..token_at + 4].copy_from_slice(&token.to_le_bytes());
        let mut script = [0u32; SCRIPT_LEN];
        for slot in &mut script {
            *slot = rng.u32();
        }
        Trial { frame, script }
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        first_mismatch: Option<String>,
    }

    fn run(case: &Case, objects: &Objects) -> Outcome {
        let mut rng = Rng(seed_of(case.name));
        let mut outcome = Outcome::default();
        for trial_index in 0..TRIALS {
            let trial = draw(&mut rng, objects, case.token_at);

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&trial.frame);
            rewritten.0[SPAN..].fill(0x5A);
            REC.lock().unwrap().reset(trial.script, objects.target);
            let rw_answer = (case.rw)(rewritten.0.as_mut_ptr() as usize as u32);
            let rw_log = std::mem::take(&mut REC.lock().unwrap().log);

            let mut lifted = UILayoutFrame::from_bytes(trial.frame);
            let mut host = Recorder::empty();
            host.reset(trial.script, objects.target);
            let lift_answer = (case.lift)(&mut lifted, &mut host);
            let lift_log = std::mem::take(&mut host.log);

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == [0x5A; BUF - SPAN];
            if rw_answer != lift_answer || rw_log != lift_log || !frame_matches || !tail_matches {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial_index}: answers {rw_answer:#010x} / {lift_answer:#010x}, logs {rw_log:?} / {lift_log:?}, frame equal {frame_matches}, tail equal {tail_matches}"
                    ));
                }
            }

            let mut wrong_host = Recorder::empty();
            wrong_host.reset(trial.script, objects.target);
            let mut wrong_frame = UILayoutFrame::from_bytes(trial.frame);
            let wrong_answer = (case.wrong)(&mut wrong_frame, &mut wrong_host);
            if wrong_answer != rw_answer || wrong_host.log != rw_log {
                outcome.wrong_mismatches += 1;
            }
        }
        outcome
    }

    #[test]
    fn token_follow_up_family_matches_its_rewrites() {
        let objects = install();
        for case in cases() {
            let outcome = run(&case, &objects);
            assert_eq!(
                outcome.lift_mismatches, 0,
                "{}: {}",
                case.name,
                outcome.first_mismatch.unwrap_or_default()
            );
            assert!(
                outcome.wrong_mismatches > 0,
                "{}: the deliberately wrong lift matched the rewrite on every trial",
                case.name
            );
        }
    }
}
