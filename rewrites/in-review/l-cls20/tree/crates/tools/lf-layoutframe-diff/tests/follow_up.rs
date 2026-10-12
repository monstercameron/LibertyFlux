//! Differential cases for the token follow-up methods of the layout frame.
//!
//! Each case runs a lifted method and its verified rewrite on the same
//! generated frames and shared state. The rewrite calls its callees and
//! virtual steps through the checker runtime's callee table and the vtables
//! it reads from the frame and the target, and it reads and writes the shared
//! bytes through the runtime's image-base mapping. The harness points the
//! callee table at recording stubs that answer from a script, and maps the
//! shared bytes into a block it owns. The lifted method calls the same steps
//! through a recording host that holds the same shared bytes. Both sides must
//! produce the same answer, the same sequence of calls with the same
//! arguments, the same frame bytes, and the same shared bytes after the call.
//! A deliberately wrong lift of each method must differ from the rewrite.
//!
//! The stubs write nothing into the frame or the shared block. The shared
//! reads and writes are compared by their final values, not by their order;
//! the rewrites read a flag at most once before each decision, so the order
//! cannot change a decision here.
//!
//! The checker runtime's callee table and image base are statics the checker
//! worker sets before a trial; the harness sets them once, in the one test
//! here, and the cases run one after another.

// Writes the checker runtime's callee table and image-base statics, which the checker worker sets before each trial in the real checker, and reads and writes the block those statics map the shared bytes into. Test-only.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_checker_rt::{CHECKER_CTABLE, CHECKER_XBASE};
    use lf_input_frontend::layout_frame::follow_up::{FollowUpHost, SharedFlag, Target, TargetStep};
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
    const FRAME_TOKEN_AT: usize = 0x54;
    const TARGET_FOLLOW_UP_AT: usize = 0x5C;
    const TARGET_STEP_107_108_AT: usize = 0x16C;
    const TARGET_STEP_111_AT: usize = 0x178;
    const TARGET_STEP_112_AT: usize = 0x184;
    const TARGET_STEP_113_AT: usize = 0x190;
    const TARGET_STEP_114_AT: usize = 0x19C;

    /// Callee ids the rewrites call.
    const CALLEE_RESOLVE: usize = 1;
    /// The token-follow-up sibling resolves through its own callee id.
    const CALLEE_RESOLVE_ALT: usize = 2;
    const CALLEE_NOTIFY: usize = 4;
    const CALLEE_FIVE: usize = 5;

    /// Image addresses of the shared bytes the family reads, and the size of
    /// the block the harness maps them into. The lowest shared byte is the
    /// base; every other shared address lies above it inside the block.
    const SHARED_BASE_VA: u32 = 0x017A_65A4;
    const HANDOFF_FLAG_VA: u32 = 0x017A_65A4;
    const PENDING_FOR_112_VA: u32 = 0x017A_65A5;
    const PENDING_FOR_113_VA: u32 = 0x017A_65A6;
    const SHARED_OBJECT_VA: u32 = 0x018B_6C8C;
    const NOTIFIER_VA: u32 = 0x019D_2E08;
    const SHARED_SIZE: usize = 0x12_0000;

    /// Tag numbers for the target's steps in the recorded log. The lift's
    /// [`TargetStep`] values map to the same tags, so a call through the
    /// wrong step shows in the log.
    const TAG_107_108: u8 = 1;
    const TAG_111: u8 = 2;
    const TAG_112: u8 = 3;
    const TAG_113: u8 = 4;
    const TAG_114: u8 = 5;

    fn step_tag(step: TargetStep) -> u8 {
        match step {
            TargetStep::Sibling107And108 => TAG_107_108,
            TargetStep::Sibling111 => TAG_111,
            TargetStep::Sibling112 => TAG_112,
            TargetStep::Sibling113 => TAG_113,
            TargetStep::Sibling114 => TAG_114,
        }
    }

    fn flag_index(flag: SharedFlag) -> usize {
        match flag {
            SharedFlag::PendingFor112 => 0,
            SharedFlag::PendingFor113 => 1,
            SharedFlag::Handoff => 2,
        }
    }

    /// Offset of a shared address inside the mapped block.
    fn shared_offset(va: u32) -> usize {
        (va - SHARED_BASE_VA) as usize
    }

    /// The buffer the rewrites write into, 4-byte aligned like the object.
    #[repr(C, align(4))]
    struct Buffer([u8; BUF]);

    /// One call, as the log records it. The receiver of a frame step is not
    /// recorded: the lift passes the frame it owns, the rewrite its buffer,
    /// and those addresses differ by construction. The notifier's receiver is
    /// not recorded either: the host knows which object it is.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Ev {
        Resolve(u32),
        FrameToken,
        TargetFollow(u32),
        FrameStep,
        TargetStep { target: u32, tag: u8, arg: u32 },
        FrameFirst(u32),
        FrameSecond(u32),
        Notify(u32),
        Sink { object: u32, stamp: u32 },
        Handoff(u32),
    }

    /// Records calls and answers them from a script, and holds the shared
    /// flags and object the lift reads. The same type serves the rewrite side
    /// (as the global behind the stubs) and the lift side (as the host passed
    /// to the lifted method).
    struct Recorder {
        script: [u32; SCRIPT_LEN],
        cursor: usize,
        log: Vec<Ev>,
        target: u32,
        flags: [u8; 3],
        object: u32,
    }

    impl Recorder {
        const fn empty() -> Self {
            Self {
                script: [0; SCRIPT_LEN],
                cursor: 0,
                log: Vec::new(),
                target: 0,
                flags: [0; 3],
                object: 0,
            }
        }

        fn reset(&mut self, trial: &Trial) {
            self.script = trial.script;
            self.cursor = 0;
            self.log.clear();
            self.target = trial.target;
            self.flags = trial.flags;
            self.object = trial.object;
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

        fn on_frame_token(&mut self) -> u32 {
            self.log.push(Ev::FrameToken);
            self.answer()
        }

        fn on_target_follow(&mut self, target: u32) -> u32 {
            self.log.push(Ev::TargetFollow(target));
            self.answer()
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

        fn on_notify(&mut self, arg: u32) -> u32 {
            self.log.push(Ev::Notify(arg));
            self.answer()
        }

        fn on_sink(&mut self, object: u32, stamp: u32) -> u32 {
            self.log.push(Ev::Sink { object, stamp });
            self.answer()
        }

        fn on_handoff(&mut self, object: u32) -> u32 {
            self.log.push(Ev::Handoff(object));
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

        fn frame_token(&mut self, _frame: &mut UILayoutFrame) -> u32 {
            self.on_frame_token()
        }

        fn target_follow_up(&mut self, target: Target) -> u32 {
            self.on_target_follow(target.0)
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

        fn read_flag(&mut self, flag: SharedFlag) -> u8 {
            self.flags[flag_index(flag)]
        }

        fn write_flag(&mut self, flag: SharedFlag, value: u8) {
            self.flags[flag_index(flag)] = value;
        }

        fn notify(&mut self, arg: u32) -> u32 {
            self.on_notify(arg)
        }

        fn shared_object(&mut self) -> u32 {
            self.object
        }

        fn call_sink(&mut self, sink: u32, stamp: u32) -> u32 {
            self.on_sink(sink, stamp)
        }

        fn handoff(&mut self, object: u32) -> u32 {
            self.on_handoff(object)
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

    extern "thiscall" fn frame_token_stub(_this: u32) -> u32 {
        REC.lock().unwrap().on_frame_token()
    }

    extern "thiscall" fn target_follow_stub(this: u32) -> u32 {
        REC.lock().unwrap().on_target_follow(this)
    }

    extern "thiscall" fn target_107_108_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_107_108, arg)
    }

    extern "thiscall" fn target_111_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_111, arg)
    }

    extern "thiscall" fn target_112_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_112, arg)
    }

    extern "thiscall" fn target_113_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_113, arg)
    }

    extern "thiscall" fn target_114_stub(this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_target_step(this, TAG_114, arg)
    }

    extern "thiscall" fn notify_stub(_this: u32, arg: u32) -> u32 {
        REC.lock().unwrap().on_notify(arg)
    }

    extern "thiscall" fn sink_stub(this: u32, stamp: u32) -> u32 {
        REC.lock().unwrap().on_sink(this, stamp)
    }

    extern "thiscall" fn handoff_stub(this: u32) -> u32 {
        REC.lock().unwrap().on_handoff(this)
    }

    fn word_of(bytes: &[u8], at: usize) -> u32 {
        let mut word = [0u8; 4];
        word.copy_from_slice(&bytes[at..at + 4]);
        u32::from_le_bytes(word)
    }

    /// The fake objects and mapped block the stubs and rewrites reach: the
    /// frame's vtable, the target, the callee table, and the shared block.
    struct Objects {
        frame_vtable: u32,
        target: u32,
        callee_table: *mut u32,
        shared: *mut u8,
        sink_addr: u32,
        handoff_addr: u32,
    }

    /// Builds the fake objects, points the callee table at the stubs, and maps
    /// the shared bytes into a block the harness owns.
    fn install() -> Objects {
        let frame_vtable: &'static mut [u32; VTABLE_WORDS] =
            Box::leak(Box::new([0u32; VTABLE_WORDS]));
        frame_vtable[FRAME_STEP_AT / 4] =
            (frame_step_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        frame_vtable[FRAME_FIRST_STEP_AT / 4] =
            (frame_first_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;
        frame_vtable[FRAME_SECOND_STEP_AT / 4] =
            (frame_second_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;
        frame_vtable[FRAME_TOKEN_AT / 4] =
            (frame_token_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;

        let target_vtable: &'static mut [u32; VTABLE_WORDS] =
            Box::leak(Box::new([0u32; VTABLE_WORDS]));
        target_vtable[TARGET_FOLLOW_UP_AT / 4] =
            (target_follow_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;
        let finish = |stub: extern "thiscall" fn(u32, u32) -> u32| stub as usize as u32;
        target_vtable[TARGET_STEP_107_108_AT / 4] = finish(target_107_108_stub);
        target_vtable[TARGET_STEP_111_AT / 4] = finish(target_111_stub);
        target_vtable[TARGET_STEP_112_AT / 4] = finish(target_112_stub);
        target_vtable[TARGET_STEP_113_AT / 4] = finish(target_113_stub);
        target_vtable[TARGET_STEP_114_AT / 4] = finish(target_114_stub);

        let target_object: &'static mut [u32; 4] =
            Box::leak(Box::new([target_vtable.as_ptr() as usize as u32, 0, 0, 0]));
        let target = target_object.as_ptr() as usize as u32;

        let callee_table: *mut u32 = Box::leak(Box::new([0u32; CALLEE_TABLE_WORDS])).as_mut_ptr();
        // SAFETY: the table holds CALLEE_TABLE_WORDS words and both ids are below that.
        unsafe {
            callee_table
                .add(CALLEE_RESOLVE)
                .write((resolve_stub as extern "cdecl" fn(u32) -> u32) as usize as u32);
            callee_table
                .add(CALLEE_RESOLVE_ALT)
                .write((resolve_stub as extern "cdecl" fn(u32) -> u32) as usize as u32);
            callee_table
                .add(CALLEE_NOTIFY)
                .write((notify_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32);
        }
        let sink_addr = (sink_stub as extern "thiscall" fn(u32, u32) -> u32) as usize as u32;
        let handoff_addr = (handoff_stub as extern "thiscall" fn(u32) -> u32) as usize as u32;

        // The shared block: a word-aligned allocation the mapped addresses
        // land in. Its base answers the lowest shared address.
        let shared_words: &'static mut [u32] =
            Box::leak(vec![0u32; SHARED_SIZE / 4].into_boxed_slice());
        let shared = shared_words.as_mut_ptr().cast::<u8>();
        let shared_addr = shared as usize as u32;
        // The runtime maps a file address to base + (address - 0x400000).
        // Choosing the base so that the lowest shared address lands on the
        // block's start makes every shared address land inside the block.
        let xbase = shared_addr
            .wrapping_sub(SHARED_BASE_VA.wrapping_sub(0x0040_0000));

        // SAFETY: these statics are read only by the runtime's accessors, and
        // this is the only place they are written; the single test calls it
        // once before any rewrite runs. The callee table and the block live
        // for the rest of the run.
        unsafe {
            core::ptr::addr_of_mut!(CHECKER_CTABLE).write(callee_table);
            core::ptr::addr_of_mut!(CHECKER_XBASE).write(xbase);
        }

        Objects {
            frame_vtable: frame_vtable.as_ptr() as usize as u32,
            target,
            callee_table,
            shared,
            sink_addr,
            handoff_addr,
        }
    }

    /// One trial's inputs: the frame bytes, the answers the stubs give, and
    /// the shared bytes before the call.
    struct Trial {
        frame: [u8; SPAN],
        script: [u32; SCRIPT_LEN],
        target: u32,
        flags: [u8; 3],
        object: u32,
    }

    /// One method under test: its rewrite, its lift, the callee-five stub it
    /// uses, the token fields it reads, and a deliberately wrong lift. Each
    /// lift and wrong lift takes the frame and the host, and answers the
    /// method's answer.
    struct Case {
        name: &'static str,
        tokens: &'static [usize],
        marker: Option<usize>,
        callee_five: fn(&Objects) -> u32,
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

    /// Wrong lift of the clear-pending sibling: the notifier runs whether or
    /// not the flag is set.
    fn wrong_113(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let token = word_of(frame.bytes(), field::WORD_E0);
        let result = if token == 0 {
            0
        } else {
            let target = host.resolve_token(token);
            let arg = host.frame_step(frame);
            let stepped = host.target_step(target, TargetStep::Sibling113, arg);
            let _gated = host.read_flag(SharedFlag::PendingFor113);
            let _ = stepped;
            host.notify(1)
        };
        host.write_flag(SharedFlag::PendingFor113, 0);
        result
    }

    /// Wrong lift of the three-section sibling: the first section ignores its
    /// flag and the tail is never taken.
    fn wrong_112(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let mut result = 0;
        let token = word_of(frame.bytes(), field::WORD_DC);
        if token != 0 {
            let target = host.resolve_token(token);
            let arg = host.frame_step(frame);
            let _stepped = host.target_step(target, TargetStep::Sibling112, arg);
            result = host.notify(1);
        }
        let marker = frame.bytes()[field::FLAG_EB];
        if marker != 0 && host.read_flag(SharedFlag::PendingFor112) == 0 {
            let stamp = host.frame_step(frame);
            let sink = host.shared_object();
            result = host.call_sink(sink, stamp);
            if host.read_flag(SharedFlag::PendingFor112) == 0 {
                result = host.notify(1);
            }
        }
        host.write_flag(SharedFlag::PendingFor112, 0);
        result
    }

    /// Wrong lift of the handoff sibling: the branch on the handoff flag is
    /// inverted.
    fn wrong_114(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let token = word_of(frame.bytes(), field::WORD_E4);
        if token == 0 {
            return 0;
        }
        let target = host.resolve_token(token);
        let arg = host.frame_step(frame);
        let stepped = host.target_step(target, TargetStep::Sibling114, arg);
        if host.read_flag(SharedFlag::Handoff) != 0 {
            let object = host.shared_object();
            host.handoff(object)
        } else {
            host.write_flag(SharedFlag::Handoff, 0);
            stepped
        }
    }

    /// Wrong lift of the plus-one sibling: the target's answer is returned
    /// without the increment.
    fn wrong_023(frame: &mut UILayoutFrame, host: &mut Recorder) -> u32 {
        let token = host.frame_token(frame);
        let target = host.resolve_token(token);
        host.target_follow_up(target)
    }

    fn sink_addr(objects: &Objects) -> u32 {
        objects.sink_addr
    }

    fn handoff_addr(objects: &Objects) -> u32 {
        objects.handoff_addr
    }

    fn cases() -> Vec<Case> {
        vec![
            Case {
                name: "token_follow_up_then_two_enables",
                tokens: &[field::WORD_D4],
                marker: None,
                callee_five: handoff_addr,
                rw: |t| r::fn_00DB16B0::rw_00db16b0(t),
                lift: |f, h| f.token_follow_up_then_two_enables(h),
                wrong: wrong_107,
            },
            Case {
                name: "token_follow_up_then_reset",
                tokens: &[field::WORD_D4],
                marker: None,
                callee_five: handoff_addr,
                rw: |t| r::fn_00DB1700::rw_00db1700(t),
                lift: |f, h| f.token_follow_up_then_reset(h),
                wrong: wrong_108,
            },
            Case {
                name: "token_follow_up_when_nonzero",
                tokens: &[field::WORD_D8],
                marker: None,
                callee_five: handoff_addr,
                rw: |t| r::fn_00DB1890::rw_00db1890(t),
                lift: |f, h| f.token_follow_up_when_nonzero(h),
                wrong: wrong_111,
            },
            Case {
                name: "token_follow_up_then_clear_pending",
                tokens: &[field::WORD_E0],
                marker: None,
                callee_five: handoff_addr,
                rw: |t| r::fn_00DB17F0::rw_00db17f0(t),
                lift: |f, h| f.token_follow_up_then_clear_pending(h),
                wrong: wrong_113,
            },
            Case {
                name: "three_gated_follow_ups_then_clear_pending",
                tokens: &[field::WORD_DC, field::WORD_E0],
                marker: Some(field::FLAG_EB),
                callee_five: sink_addr,
                rw: |t| r::fn_00DB1740::rw_00db1740(t),
                lift: |f, h| f.three_gated_follow_ups_then_clear_pending(h),
                wrong: wrong_112,
            },
            Case {
                name: "resolve_token_and_follow_up_plus_one",
                tokens: &[],
                marker: None,
                callee_five: handoff_addr,
                rw: |t| r::fn_00DB1660::rw_00db1660(t),
                lift: |f, h| f.resolve_token_and_follow_up_plus_one(h),
                wrong: wrong_023,
            },
            Case {
                name: "token_follow_up_then_handoff",
                tokens: &[field::WORD_E4],
                marker: None,
                callee_five: handoff_addr,
                rw: |t| r::fn_00DB1840::rw_00db1840(t),
                lift: |f, h| f.token_follow_up_then_handoff(h),
                wrong: wrong_114,
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

        /// Non-zero in two cases out of three.
        fn is_set(&mut self) -> bool {
            self.next() % 3 != 0
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

    /// Draws one trial. Each token field is zero a third of the time; the
    /// marker byte is zero half the time; the shared flags are random.
    fn draw(rng: &mut Rng, objects: &Objects, case: &Case) -> Trial {
        let mut frame = [0u8; SPAN];
        for byte in &mut frame {
            *byte = rng.next() as u8;
        }
        frame[0..4].copy_from_slice(&objects.frame_vtable.to_le_bytes());
        for &at in case.tokens {
            let token = if rng.is_set() {
                match rng.u32() {
                    0 => 1,
                    value => value,
                }
            } else {
                0
            };
            frame[at..at + 4].copy_from_slice(&token.to_le_bytes());
        }
        if let Some(at) = case.marker {
            frame[at] = if rng.is_set() { 1 + (rng.u32() % 255) as u8 } else { 0 };
        }
        let mut script = [0u32; SCRIPT_LEN];
        for slot in &mut script {
            *slot = rng.u32();
        }
        let flags = [
            if rng.is_set() { 0 } else { 1 },
            if rng.is_set() { 0 } else { 1 },
            if rng.is_set() { 0 } else { 1 },
        ];
        let object = match rng.u32() {
            0 => 1,
            value => value,
        };
        Trial { frame, script, target: objects.target, flags, object }
    }

    #[derive(Default)]
    struct Outcome {
        lift_mismatches: u32,
        wrong_mismatches: u32,
        trials_with_calls: u32,
        first_mismatch: Option<String>,
    }

    /// Writes a trial's shared bytes into the block the rewrites read.
    ///
    /// # Safety
    ///
    /// `objects.shared` points at the block [`install`] leaked, which holds
    /// [`SHARED_SIZE`] bytes, and the offsets written are inside it.
    unsafe fn write_shared(objects: &Objects, trial: &Trial) {
        let flags_at = [
            shared_offset(PENDING_FOR_112_VA),
            shared_offset(PENDING_FOR_113_VA),
            shared_offset(HANDOFF_FLAG_VA),
        ];
        for index in 0..3 {
            // SAFETY: the offset lies inside the leaked block (see the function's contract).
            unsafe { objects.shared.add(flags_at[index]).write(trial.flags[index]) };
        }
        let object_at = shared_offset(SHARED_OBJECT_VA);
        for (i, byte) in trial.object.to_le_bytes().iter().enumerate() {
            // SAFETY: the offset lies inside the leaked block (see the function's contract).
            unsafe { objects.shared.add(object_at + i).write(*byte) };
        }
    }

    /// Reads the three shared flags back from the block.
    ///
    /// # Safety
    ///
    /// Same contract as [`write_shared`].
    unsafe fn read_flags(objects: &Objects) -> [u8; 3] {
        let flags_at = [
            shared_offset(PENDING_FOR_112_VA),
            shared_offset(PENDING_FOR_113_VA),
            shared_offset(HANDOFF_FLAG_VA),
        ];
        // SAFETY: each offset lies inside the leaked block (see the function's contract).
        [
            unsafe { objects.shared.add(flags_at[0]).read() },
            unsafe { objects.shared.add(flags_at[1]).read() },
            unsafe { objects.shared.add(flags_at[2]).read() },
        ]
    }

    fn run(case: &Case, objects: &Objects) -> Outcome {
        let mut rng = Rng(seed_of(case.name));
        let mut outcome = Outcome::default();
        // The callee-five stub this method calls, set once per method.
        // SAFETY: the callee table is leaked by `install` and the index is in range.
        // SAFETY: the table holds CALLEE_TABLE_WORDS words and the index is below that.
        unsafe { objects.callee_table.add(CALLEE_FIVE).write((case.callee_five)(objects)) };
        for trial_index in 0..TRIALS {
            let trial = draw(&mut rng, objects, case);

            let mut rewritten = Buffer([0u8; BUF]);
            rewritten.0[..SPAN].copy_from_slice(&trial.frame);
            rewritten.0[SPAN..].fill(0x5A);
            // SAFETY: the block holds every shared offset the family reads.
            unsafe { write_shared(objects, &trial) };
            REC.lock().unwrap().reset(&trial);
            let rw_answer = (case.rw)(rewritten.0.as_mut_ptr() as usize as u32);
            let rw_log = std::mem::take(&mut REC.lock().unwrap().log);
            if !rw_log.is_empty() {
                outcome.trials_with_calls += 1;
            }
            // SAFETY: as above.
            let rw_flags = unsafe { read_flags(objects) };

            let mut lifted = UILayoutFrame::from_bytes(trial.frame);
            let mut host = Recorder::empty();
            host.reset(&trial);
            let lift_answer = (case.lift)(&mut lifted, &mut host);
            let lift_log = std::mem::take(&mut host.log);

            let frame_matches = rewritten.0[..SPAN] == lifted.bytes()[..];
            let tail_matches = rewritten.0[SPAN..] == [0x5A; BUF - SPAN];
            let flags_match = rw_flags == host.flags;
            if rw_answer != lift_answer
                || rw_log != lift_log
                || !frame_matches
                || !tail_matches
                || !flags_match
            {
                outcome.lift_mismatches += 1;
                if outcome.first_mismatch.is_none() {
                    outcome.first_mismatch = Some(format!(
                        "trial {trial_index}: answers {rw_answer:#010x} / {lift_answer:#010x}, logs {rw_log:?} / {lift_log:?}, frame equal {frame_matches}, tail equal {tail_matches}, flags {rw_flags:?} / {:?}",
                        host.flags
                    ));
                }
            }

            let mut wrong_host = Recorder::empty();
            wrong_host.reset(&trial);
            let mut wrong_frame = UILayoutFrame::from_bytes(trial.frame);
            let wrong_answer = (case.wrong)(&mut wrong_frame, &mut wrong_host);
            if wrong_answer != rw_answer
                || wrong_host.log != rw_log
                || wrong_host.flags != rw_flags
            {
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
                outcome.trials_with_calls > TRIALS / 4,
                "{}: only {} of {} trials made a call",
                case.name,
                outcome.trials_with_calls,
                TRIALS
            );
            assert!(
                outcome.wrong_mismatches > 0,
                "{}: the deliberately wrong lift matched the rewrite on every trial",
                case.name
            );
        }
    }

    /// The notifier's address is only passed to the notifier, never read, so
    /// it is not part of the block; the bytes the family reads or writes are.
    #[test]
    fn the_shared_block_maps_every_shared_byte_the_family_touches() {
        assert!(NOTIFIER_VA > SHARED_BASE_VA);
        for va in [
            HANDOFF_FLAG_VA,
            PENDING_FOR_112_VA,
            PENDING_FOR_113_VA,
            SHARED_OBJECT_VA,
        ] {
            assert!(shared_offset(va) + 4 <= SHARED_SIZE, "address {va:#x} is outside the block");
        }
    }
}
