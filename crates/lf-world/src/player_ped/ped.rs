//! The player pedestrian's own data and the nine lifted methods.
//!
//! Each method's doc comment says what the original does, in the order it
//! does it. Floating-point operands are pinned with
//! `core::hint::black_box` where both operands can be not-a-number, so the
//! result carries the first operand's payload as the original's does.

use core::hint::black_box;

use lf_core::Handle32;

use super::world::{ByteGlobal, GateWorld, ObjTag, PedTag, PlayerPedWorld, TaskWorld, WordGlobal};

/// An object identity the pedestrian stores: a pointer in the original.
/// `None` stands for the original's zero.
pub type ObjRef = Option<Handle32<ObjTag>>;

/// Field offsets within the class's 32-bit layout. Structure facts only:
/// the lifted code reads and writes the named fields, never these offsets.
pub mod offsets {
    /// The current model slot word, compared with the sentinel of "none".
    pub const CURRENT_SLOT: u32 = 0x68;
    /// The stored result of the setup-and-store step.
    pub const SETUP_RESULT: u32 = 0x6C;
    /// The link pointer compared with the link target.
    pub const LINK: u32 = 0x38;
    /// The first sub-object pointer.
    pub const SUB_A: u32 = 0x7B0;
    /// The second sub-object pointer, also the link target.
    pub const SUB_B: u32 = 0x7B4;
    /// The mode half-word whose low nibble selects the pre-pass.
    pub const MODE: u32 = 0x1E2;
    /// The motion value, a single-precision float.
    pub const MOTION_VALUE: u32 = 0xE9C;
    /// The first two words the reset step zeroes.
    pub const PRE_WORDS: u32 = 0xD48;
    /// The two words the apply step sets to the all-ones sentinel.
    pub const APPLY_WORDS: u32 = 0xD50;
    /// A four-word descriptor copied from a caller's block.
    pub const DESCRIPTOR: u32 = 0xEB0;
    /// A four-word transform copied from a caller's block.
    pub const TRANSFORM: u32 = 0xEC0;
    /// The job word, also read as the bundle context.
    pub const JOB_WORD: u32 = 0x21C;
    /// The pointer to the sub-object whose entry chain runs; the chain
    /// works on the address this word names plus a fixed offset.
    pub const CHAIN_BASE: u32 = 0x224;
    /// The aim context the pedestrian holds: a pointer, zero when absent.
    pub const AIM_CONTEXT: u32 = 0x228;
    /// The source point pointer the relink measures from.
    pub const SOURCE: u32 = 0x20;
    /// The byte that, when set, marks the relink word at [`RELINK_FLAGS`].
    pub const MARKER_219: u32 = 0x219;
    /// The word the relink marks with its two low bits.
    pub const RELINK_FLAGS: u32 = 0xBE0;
    /// The nine words the relink clears when its flag byte is set.
    pub const RELINK_BLOCK: [u32; 9] = [
        0xAD0, 0xAD4, 0xAD8, 0xAE0, 0xAE4, 0xAE8, 0xAF0, 0xAF4, 0xAF8,
    ];
    /// The chain index, a signed half-word that selects a pose table entry.
    pub const CHAIN_INDEX: u32 = 0x2E;
    /// The status word the gate block keeps its lock bit in.
    pub const STATUS_28: u32 = 0x28;
    /// The flag byte the gate block tests for its sub-object step.
    pub const FLAG_F4: u32 = 0xF4;
    /// The outer object pointer the attachment step reads its inner object from.
    pub const OUTER: u32 = 0xE98;
    /// The float word the gate block compares with a global constant.
    pub const FVAL: u32 = 0xBF8;
    /// The status word the gate block arms.
    pub const STATUS_29C: u32 = 0x29C;
    /// The budget the task step settles.
    pub const BUDGET: u32 = 0xEA0;
    /// The budget the task step copies to on its last step.
    pub const BUDGET_PREV: u32 = 0xEA4;
}

/// The offset of the targeting component inside the pedestrian.
const TARGETING_OFFSET: u32 = 0x2B0;
/// The offset of the target-creation component inside the pedestrian.
const TARGET_OFFSET: u32 = 0xE6C;
/// The aim state word value that enables the aim scaling.
const AIM_STATE_SCALED: u32 = 2;
/// The helper code used when the helper flag's low byte is zero.
const HELPER_CODE_PLAIN: u32 = 0x22E8;
/// The helper code used when the helper flag's low byte is set.
const HELPER_CODE_ALT: u32 = 0x22E4;
/// The quiet-not-a-number bit, set on a not-a-number threshold.
const QUIET_BIT: u32 = 0x0040_0000;

/// The bias added to the job word before the first job runs.
const JOB_BIAS: u32 = 0x80;

/// The fixed offset the entry chain adds to its sub-object pointer.
const CHAIN_OFFSET: u32 = 0x84;

/// The player pedestrian's data the lifted methods own.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CPlayerPed {
    /// The current model slot word (see [`offsets::CURRENT_SLOT`]).
    pub current_slot: u32,
    /// The stored result of the setup-and-store step.
    pub setup_result: u32,
    /// The link pointer.
    pub link: ObjRef,
    /// The first sub-object pointer.
    pub sub_a: ObjRef,
    /// The second sub-object pointer, also the link target.
    pub sub_b: ObjRef,
    /// The mode half-word.
    pub mode: u16,
    /// The motion value.
    pub motion_value: f32,
    /// The two words the reset step zeroes.
    pub pre_words: [u32; 2],
    /// The two words the apply step sets to the all-ones sentinel.
    pub apply_words: [u32; 2],
    /// A descriptor copied from a caller's block.
    pub descriptor: [u32; 4],
    /// A transform copied from a caller's block.
    pub transform: [u32; 4],
    /// The job word.
    pub job_word: u32,
    /// The pointer word the entry chain offsets; zero is a value the chain
    /// runs on, not an absent object.
    pub chain_base: u32,
    /// The aim context pointer; zero when there is none.
    pub aim_context: u32,
    /// The source point pointer the relink measures from.
    pub source: u32,
    /// The marker byte the relink reads.
    pub marker_219: u8,
    /// The relink word the marker sets bits in.
    pub relink_flags: u32,
    /// The nine words the relink clears, in the order of
    /// [`offsets::RELINK_BLOCK`].
    pub relink_block: [u32; 9],
    /// The chain index that selects a pose table entry.
    pub chain_index: i16,
    /// The gate block's status word, whose bit 0x80_0000 is its lock.
    pub status_28: u32,
    /// The flag byte the gate block tests for its sub-object step.
    pub flag_f4: u8,
    /// The outer object pointer.
    pub outer: u32,
    /// The float word the gate block compares with a global constant.
    pub fval_bits: u32,
    /// The status word the gate block arms.
    pub status_29c: u32,
    /// The budget the task step settles.
    pub budget: u32,
    /// The budget the task step copied on its last run.
    pub budget_prev: u32,
}

/// The scaled aim threshold: the aim factor times the property threshold.
///
/// When either operand is not-a-number the result is that operand's payload
/// with the quiet bit set, the aim factor's when both are, as the original
/// keeps it; otherwise the product. The product's operands are pinned with
/// `black_box` so a finite product keeps the operand order.
fn scaled_threshold(initial: f32, aim: f32) -> f32 {
    let product = black_box(aim) * black_box(initial);
    if initial.is_nan() && aim.is_nan() {
        f32::from_bits(aim.to_bits() | QUIET_BIT)
    } else if initial.is_nan() {
        f32::from_bits(initial.to_bits() | QUIET_BIT)
    } else if aim.is_nan() {
        f32::from_bits(aim.to_bits() | QUIET_BIT)
    } else {
        product
    }
}

/// The state word value that refuses the gate.
const STATE_REFUSE: u32 = 1;
/// The mode word value that refuses the gate.
const MODE_REFUSE: u32 = 0x12;
/// The busy bit a probe clears in the relink word.
const BIT_BUSY: u32 = 0x2;
/// The bit the gate block arms in its status word.
const BIT_ARMED: u32 = 0x0100_0000;
/// The lock bit of the status word that blocks the deep refresh.
const BIT_LOCK: u32 = 0x0080_0000;
/// The offset of the sub-object the gate block refreshes.
const SUB_OFFSET: u32 = 0xBB0;
/// The offset of the inner object pointer inside the outer object.
const OUTER_INNER: u32 = 0x58;

/// Which of the two probes a gate step runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProbeId {
    First,
    Second,
}

/// The gate's hook step: the hook answers with its argument, else the first
/// flag byte must be set, and then the second flag byte decides.
fn hook_gate<W: GateWorld + ?Sized>(world: &mut W) -> bool {
    let argument = world.hook_argument();
    if world.hook_call(argument) != 0 {
        return true;
    }
    if world.global_byte(ByteGlobal::FlagA) == 0 {
        return false;
    }
    world.global_byte(ByteGlobal::FlagB) != 0
}

impl CPlayerPed {
    /// The sub-object the gate block refreshes: the pedestrian's address plus a
    /// fixed offset.
    fn sub_address(&self, this: Handle32<PedTag>) -> u32 {
        this.get().wrapping_add(SUB_OFFSET)
    }

    /// Runs the attachment step when the indexed pose entry is present, its
    /// flag byte is set and the marker byte is set: the outer object's inner
    /// object is attached to the pedestrian.
    fn attach_indexed<W: GateWorld + ?Sized>(&mut self, this: Handle32<PedTag>, world: &mut W) {
        let entry = world.pose_entry(i32::from(self.chain_index));
        if entry == 0 || world.entry_flag(entry) == 0 || self.marker_219 == 0 {
            return;
        }
        let inner = world.read_word(self.outer.wrapping_add(OUTER_INNER));
        world.attach_virtual(inner, this);
    }

    /// Runs one probe on the chain base and, when it answers with a set low
    /// byte, clears the busy bit of the relink word. Returns the low byte.
    fn probe_and_clear<W: GateWorld + ?Sized>(&mut self, which: ProbeId, world: &mut W) -> u8 {
        let obj = self.chain_base;
        let answer = match which {
            ProbeId::First => world.probe_first(obj),
            ProbeId::Second => world.probe_second(obj),
        };
        if (answer & 0xFF) != 0 {
            self.relink_flags &= !BIT_BUSY;
        }
        (answer & 0xFF) as u8
    }

    /// The deep refresh: runs when the deep gate passes and the lock bit of the
    /// status word is clear; then refreshes the sub-object.
    fn deep_refresh<W: GateWorld + ?Sized>(&mut self, this: Handle32<PedTag>, world: &mut W) {
        if (world.deep_refresh_gate() & 0xFF) != 0 {
            return;
        }
        if self.status_28 & BIT_LOCK != 0 {
            return;
        }
        world.deep_refresh_entry(self.sub_address(this));
    }

    /// Refreshes the pedestrian's state, and reports its readiness.
    ///
    /// The gate runs in order. Gate A is the hook answer, or the first flag
    /// byte set with the second clear. The two or-bytes join gate A's answer;
    /// when any is set the guard byte must also be set, or the method returns
    /// zero. The pedestrian's own refresh entry runs on its result, then the
    /// attachment step. Gate C then refuses on the state word, on the unequal
    /// pair, or on the mode word. Two probes run next and clear the busy bit
    /// when they answer with a set low byte; when the second answers zero, the
    /// deep refresh runs if the relink bit is set, or else when the marker byte
    /// is clear and the float word differs from the global constant. The
    /// sub-object's entry runs when the flag byte has its second bit set. The
    /// status word gets its armed bit, and the readiness probe's answer is
    /// returned; when that answer is zero, the tail entry runs first.
    ///
    /// Gate C and the float compare use the words as they are read: the float
    /// compare is an ordinary inequality, so a not-a-number on either side
    /// differs.
    pub fn refresh_gate<W: GateWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        world: &mut W,
    ) -> u32 {
        let mut acc = u8::from(hook_gate(world));
        acc |= world.global_byte(ByteGlobal::OrFirst);
        acc |= world.global_byte(ByteGlobal::OrSecond);
        if acc != 0 && world.global_byte(ByteGlobal::Guard) == 0 {
            return 0;
        }
        let got = world.refresh_virtual(self, this);
        world.refresh_follow(got);
        self.attach_indexed(this, world);
        if world.global_word(WordGlobal::State) == STATE_REFUSE {
            return 0;
        }
        if world.global_word(WordGlobal::PairA) != world.global_word(WordGlobal::PairB) {
            return 0;
        }
        if world.global_word(WordGlobal::Mode) == MODE_REFUSE {
            return 0;
        }
        self.probe_and_clear(ProbeId::First, world);
        if self.probe_and_clear(ProbeId::Second, world) == 0 {
            if (self.relink_flags >> 1) & 1 != 0 {
                self.deep_refresh(this, world);
            } else if self.marker_219 == 0 {
                let current = f32::from_bits(self.fval_bits);
                let constant = f32::from_bits(world.global_word(WordGlobal::FloatConst));
                if current != constant {
                    self.deep_refresh(this, world);
                }
            }
        }
        if self.flag_f4 & 2 != 0 {
            world.sub_virtual(self.sub_address(this));
        }
        self.status_29c |= BIT_ARMED;
        let ready = world.ready_probe(self, this);
        if ready == 0 {
            world.tail_virtual(self, this);
        }
        ready
    }

    /// Adds a single-precision amount to the motion value, stores the sum
    /// and hands its bit pattern to the motion helper, whose answer is the
    /// method's answer.
    ///
    /// Both operands are pinned with `black_box`: the sum keeps the operand
    /// order of the original's float add, so a not-a-number on both sides
    /// keeps the same payload.
    pub fn add_motion_value<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        amount: f32,
        world: &mut W,
    ) -> u32 {
        let updated = black_box(amount) + black_box(self.motion_value);
        self.motion_value = updated;
        world.motion_helper(self, this, updated.to_bits())
    }

    /// Notifies the trigger entry while the link is live, else runs the
    /// base update.
    ///
    /// The link is live when it is set and equal to the link target. The
    /// live case calls the trigger entry with the constant words 0, 1 and 0;
    /// the other case forwards to the base update.
    pub fn notify_while_linked<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        world: &mut W,
    ) -> u32 {
        match (self.link, self.sub_b) {
            (Some(link), Some(target)) if link == target => {
                world.trigger_entry(self, this, 0, 1, 0)
            }
            _ => world.base_update(self, this),
        }
    }

    /// Runs the two jobs when the ready check passes; reports whether they
    /// ran.
    ///
    /// The ready check's low byte decides. When it is set, the first job
    /// runs with the job word biased by a fixed amount and the job word
    /// itself. The second job then runs with the job word read again, since
    /// the first job may have changed it, and the same argument shape.
    pub fn run_jobs_if_ready<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        world: &mut W,
    ) -> bool {
        if (world.ped_ready_check(self, this) & 0xff) == 0 {
            return false;
        }
        let job = self.job_word;
        world.ped_job_a(self, this, job.wrapping_add(JOB_BIAS), job);
        let job = self.job_word;
        world.ped_job_b(self, this, job.wrapping_add(JOB_BIAS), job);
        true
    }

    /// Clears the reset words, runs the pre-pass when the mode nibble is one
    /// of the listed values and the argument's bit 0x10 is clear, sets the
    /// apply words to the all-ones sentinel, runs the apply pass, then
    /// notifies the first sub-object, whose answer is the method's answer.
    ///
    /// The mode nibble is read before the pre-pass runs; the sub-object is
    /// read after the apply pass.
    pub fn reset_and_apply<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        arg: u32,
        world: &mut W,
    ) -> u32 {
        let nibble = self.mode & 0xF;
        self.pre_words = [0, 0];
        if matches!(nibble, 6 | 8 | 9 | 0xA) && (arg & 0x10) == 0 {
            world.pre_step(self, this, arg);
        }
        self.apply_words = [u32::MAX, u32::MAX];
        world.apply_step(self, this, arg);
        let obj = self.sub_a;
        world.notify_object(self, obj, 8, 0)
    }

    /// Dispatches the argument to each sub-object that is present, first
    /// the first sub-object and then the second. Returns the last answer
    /// produced, or zero when neither sub-object is present.
    ///
    /// The second sub-object is read after the first dispatch has run.
    pub fn notify_sub_objects<W: PlayerPedWorld + ?Sized>(
        &mut self,
        arg: u32,
        world: &mut W,
    ) -> u32 {
        let mut answer = 0;
        if let Some(first) = self.sub_a {
            answer = world.dispatch_slot8(self, first, arg);
        }
        if let Some(second) = self.sub_b {
            answer = world.dispatch_slot8(self, second, arg);
        }
        answer
    }

    /// Asks the context for a lookup object; when it has one, runs the
    /// setup helper on it with the pedestrian and the arguments, stores the
    /// setup result in the pedestrian, runs the follow-up helper on that
    /// result, and returns the stored word read back.
    ///
    /// The read back comes after the follow-up helper, so a change the
    /// helper makes to the stored word is what the method returns.
    pub fn setup_and_store<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        first: u32,
        second: u32,
        third: u32,
        setup_tail: u32,
        final_arg: u32,
        world: &mut W,
    ) -> u32 {
        let context = world.player_context();
        let lookup = world.lookup_context(context);
        let result = if lookup != 0 {
            world.setup(self, lookup, this, 0, first, second, third, setup_tail)
        } else {
            0
        };
        self.setup_result = result;
        let _ = world.finalize(self, result, final_arg);
        self.setup_result
    }

    /// Changes the pedestrian's model. When the model's state byte is zero,
    /// the lookup's answer is the method's answer. Otherwise the model is
    /// preloaded, looked up again, the pedestrian is revived with a flag
    /// that is zero only when the current slot is the all-ones sentinel,
    /// and the bundle named by the job word is loaded, whose answer is the
    /// method's answer.
    ///
    /// The current slot and the job word are read after the calls that
    /// precede them, as the original reads them.
    pub fn change_model<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        model_id: u32,
        world: &mut W,
    ) -> u32 {
        if world.model_state(model_id) == 0 {
            return world.lookup_model(self, this, model_id);
        }
        world.preload_model(model_id);
        world.lookup_model(self, this, model_id);
        let revive_flag = if self.current_slot == u32::MAX { 0 } else { 1 };
        world.revive(self, this, revive_flag);
        let bundle = self.job_word;
        world.load_bundle(self, this, bundle)
    }

    /// Acquires an aim target from the pedestrian's targeting context and
    /// reports whether one was acquired.
    ///
    /// The context the targeting component returns is asked for its property
    /// argument, the property is looked up, and its threshold is the first
    /// value. When the aim state word is the scaled value and the state gate
    /// passes, and the pedestrian holds an aim context, the threshold is
    /// scaled by that context's aim factor (see [`scaled_threshold`]). A
    /// target is then created with the threshold and the property key and
    /// attached to the targeting component, and the aim context is marked
    /// selected. A null context, a null target or a missing aim context end
    /// the method early with no mark.
    ///
    /// The aim state word is read before the gate runs; the aim context is
    /// read only on the scaled path and again after the target is attached.
    pub fn acquire_aim_target<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        property_key: u32,
        helper_flag: u32,
        world: &mut W,
    ) -> bool {
        let targeting = this.get().wrapping_add(TARGETING_OFFSET);
        let context = world.get_context(targeting);
        if context == 0 {
            return false;
        }
        let property_argument = world.context_property_argument(context);
        let property = world.get_property(property_argument);
        let initial = f32::from_bits(world.property_threshold_bits(property));
        let state = world.aim_state();
        let threshold = if state == AIM_STATE_SCALED && (world.state_gate() as u8) != 0 {
            if self.aim_context == 0 {
                initial
            } else {
                let aim = f32::from_bits(world.aim_factor_bits(self.aim_context));
                scaled_threshold(initial, aim)
            }
        } else {
            initial
        };
        let helper_code = if (helper_flag as u8) == 0 {
            HELPER_CODE_PLAIN
        } else {
            HELPER_CODE_ALT
        };
        let target = world.create_target(
            this.get().wrapping_add(TARGET_OFFSET),
            helper_code,
            threshold.to_bits(),
            property_key,
            0,
            0,
        );
        if target == 0 {
            return false;
        }
        world.attach_target(targeting, target);
        if self.aim_context == 0 {
            return false;
        }
        world.mark_selected(self.aim_context);
        true
    }

    /// Relinks the pedestrian toward its target when it has moved far enough,
    /// clears the relink block when the flag byte is set, and returns the
    /// relink answer or the source pointer on a short miss.
    ///
    /// The base entry runs first. The link is compared with the target; when
    /// they differ and the target is present, the squared distance between the
    /// source point and the target point is measured in the original's order,
    /// (dx*dx + dy*dy) + dz*dz. A set flag byte, or a distance above the
    /// threshold, runs the relink entry with one; otherwise the source pointer
    /// is returned at once. A set flag byte then clears the relink block and,
    /// when the marker byte is set, sets two bits of the relink word.
    ///
    /// The answer is the link when no relink ran. A distance that is not a
    /// number fails the threshold test, as the original's compare does.
    pub fn relink_on_move<W: PlayerPedWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        flag: u32,
        world: &mut W,
    ) -> u32 {
        world.base_entry(self, this, flag);
        let mut answer = Handle32::raw_or_zero(self.link);
        let target = Handle32::raw_or_zero(self.sub_b);
        if answer != target && target != 0 {
            let src = self.source;
            let s_x = f32::from_bits(world.read_word(src.wrapping_add(0x30)));
            let s_y = f32::from_bits(world.read_word(src.wrapping_add(0x34)));
            let s_z = f32::from_bits(world.read_word(src.wrapping_add(0x38)));
            let t_x = f32::from_bits(world.read_word(target.wrapping_add(0x40)));
            let t_y = f32::from_bits(world.read_word(target.wrapping_add(0x44)));
            let t_z = f32::from_bits(world.read_word(target.wrapping_add(0x48)));
            let dx = black_box(s_x) - black_box(t_x);
            let dy = black_box(s_y) - black_box(t_y);
            let dz = black_box(s_z) - black_box(t_z);
            let dist_sq = black_box(black_box(dx * dx) + black_box(dy * dy)) + black_box(dz * dz);
            let threshold = f32::from_bits(world.distance_threshold_bits());
            if (flag & 0xFF) != 0 || dist_sq > threshold {
                answer = world.relink(self, this, 1);
            } else {
                return src;
            }
        }
        if (flag & 0xFF) == 0 {
            return answer;
        }
        self.relink_block = [0; 9];
        if self.marker_219 != 0 {
            self.relink_flags |= 0x0C;
        }
        answer
    }

    /// Runs the entry chain of the sub-object the chain base names: the
    /// reset entry, then the flag entry with the flag set, then the main
    /// entry, whose answer is the method's answer.
    ///
    /// The sub-object is the chain base plus a fixed offset, computed once
    /// before the first entry runs; the chain base is not read again.
    pub fn run_entry_chain<W: PlayerPedWorld + ?Sized>(&mut self, world: &mut W) -> u32 {
        let sub = self.chain_base.wrapping_add(CHAIN_OFFSET);
        world.chain_reset(sub);
        world.chain_flag(sub, 1);
        world.chain_main(sub)
    }

    /// Stores the four words of a caller's transform block and returns the
    /// fourth. The words are copied by bits, so signed zeros and payloads
    /// are kept.
    pub fn set_transform(&mut self, source: [u32; 4]) -> u32 {
        self.transform = source;
        source[3]
    }

    /// Stores the four words of a caller's descriptor block by bits. The
    /// original returns zero, which carries no information and is not part
    /// of this method.
    pub fn set_descriptor(&mut self, source: [u32; 4]) {
        self.descriptor = source;
    }
}
/// The task-list link offset inside the task record.
const TASK_LIST: u32 = 0x2E0;
/// The task tag the first walk looks for.
const TAG_FIRST: u32 = 0x410;
/// The task tag the second walk looks for.
const TAG_SECOND: u32 = 0x41E;
/// The task kind whose presence decides the budget's cost step.
const TASK_BUDGET: u32 = 0x413;
/// The largest budget the award step allows.
const BUDGET_MAX: u32 = 10_000;
/// The heat byte's floor: a heat at or below it is refused.
const HEAT_FLOOR: u8 = 0x64;
/// The offsets of the two heat bytes inside the item.
const HEAT_FIRST: u32 = 0x26DC;
const HEAT_SECOND: u32 = 0x26DE;
/// The output words that set the classified flag, and the one that also holds.
const OUT_FLAG: u32 = 5;
const OUT_HOLD: u32 = 8;
/// The bound of the cost conversion: the 64-bit range, as floating-point values.
const COST_LIMIT: f64 = 9_223_372_036_854_775_808.0;

/// Walks the task chain for a node with the given tag. Returns whether the
/// tag was found and the priority of the node the walk stopped on. The walk
/// aborts, reporting the node it stopped on, when the previous node's
/// priority is at least two and strictly below the current one.
///
/// The chain is walked as the original walks it: the first node compares with
/// itself, every later node with the one before it. The walk ends at a null
/// link, so a cyclic chain would not end; the chains the proof generates are
/// finite.
fn task_walk<W: TaskWorld + ?Sized>(world: &mut W, mut node: u32, tag: u32) -> (bool, u32) {
    let mut previous: Option<u32> = None;
    let mut stopped = 0;
    while node != 0 {
        let priority = (world.read_word(node.wrapping_add(8)) >> 1) & 7;
        let compared = previous.unwrap_or(priority);
        if compared < priority && compared >= 2 {
            return (false, priority);
        }
        if world.read_word(node.wrapping_add(4)) == tag {
            return (true, priority);
        }
        previous = Some(priority);
        stopped = priority;
        node = world.read_word(node.wrapping_add(0x0C));
    }
    (false, stopped)
}

impl CPlayerPed {
    /// Settles the pedestrian's wanted-heat budget from its active task list,
    /// after running the base step, and returns the budget the step settled.
    ///
    /// The base step runs first. When the marker byte is clear, its answer is
    /// the method's answer. Otherwise the task list is walked twice, first for
    /// the first tag and then for the second; when neither walk finds its tag,
    /// the second walk's priority is the answer. A null item answers zero.
    /// The item's classification sets a flag and possibly a hold, and the task
    /// budget query decides the budget: when the task is present the frame
    /// cost is subtracted and clamped at zero; when it is absent and the hold
    /// is clear, the award is added and clamped at the maximum.
    ///
    /// The heat bytes of the item decide whether the item is accepted: the flag
    /// must be set; a hold accepts; otherwise the heat must be above the floor
    /// and below the heat limit. The budget is then copied to the previous
    /// slot, zeroed when the item is refused, and the copied value is returned.
    ///
    /// The cost is the frame delta times the cost constant, truncated toward
    /// zero. A not-a-number and any value at or beyond either end of the 64-bit
    /// range give the integer indefinite, zero in the low word, as the original
    /// does; the cast below is only reached inside that range.
    pub fn settle_budget<W: TaskWorld + ?Sized>(
        &mut self,
        this: Handle32<PedTag>,
        world: &mut W,
    ) -> u32 {
        let base = world.base_step(self, this);
        if self.marker_219 == 0 {
            return base;
        }
        let task = self.chain_base;
        let head = world.read_word(task.wrapping_add(TASK_LIST));
        if head == 0 {
            return task;
        }
        let (found_first, _) = task_walk(world, head, TAG_FIRST);
        if !found_first {
            let (found_second, priority) = task_walk(world, head, TAG_SECOND);
            if !found_second {
                return priority;
            }
        }
        let item = world.current_item(self, this);
        if item == 0 {
            return 0;
        }
        let (answer, out) = world.classify_task(self, task);
        let mut flag = true;
        let mut hold = false;
        if (answer & 0xFF) != 0 {
            flag = false;
            if out == OUT_FLAG || out == OUT_HOLD {
                flag = true;
                hold = out == OUT_HOLD;
            }
        }
        if (world.task_present(self, task.wrapping_add(TASK_LIST), TASK_BUDGET, 0) & 0xFF) != 0 {
            let delta = f32::from_bits(world.frame_delta_bits());
            let scale = f32::from_bits(world.cost_scale_bits());
            let cost_float = f64::from((black_box(delta) * black_box(scale)).trunc());
            // The bounds keep the conversion inside the 64-bit range; a value
            // outside it, or not a number, takes the integer indefinite.
            let cost64: i64 =
                if cost_float.is_nan() || cost_float >= COST_LIMIT || cost_float < -COST_LIMIT {
                    i64::MIN
                } else {
                    cost_float as i64
                };
            let cost = cost64 as i32 as u32;
            let next = self.budget.wrapping_sub(cost);
            self.budget = if (next as i32) >= 0 { next } else { 0 };
        } else if !hold {
            let award = world.award(self);
            let mut next = self.budget.wrapping_add(award);
            if (next as i32) > (BUDGET_MAX as i32) {
                next = BUDGET_MAX;
            }
            self.budget = next;
        }
        let heat = world.read_byte(item.wrapping_add(HEAT_SECOND))
            ^ world.read_byte(item.wrapping_add(HEAT_FIRST));
        let accepted = if !flag {
            false
        } else if hold {
            true
        } else if heat <= HEAT_FLOOR {
            false
        } else {
            heat < world.heat_limit()
        };
        let old = self.budget;
        if !accepted {
            self.budget = 0;
        }
        self.budget_prev = old;
        old
    }
}
