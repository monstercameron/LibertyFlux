//! The player pedestrian's own data and the nine lifted methods.
//!
//! Each method's doc comment says what the original does, in the order it
//! does it. Floating-point operands are pinned with
//! `core::hint::black_box` where both operands can be not-a-number, so the
//! result carries the first operand's payload as the original's does.

use core::hint::black_box;

use lf_core::Handle32;

use super::world::{ObjTag, PedTag, PlayerPedWorld};

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
}

/// The bias added to the job word before the first job runs.
const JOB_BIAS: u32 = 0x80;

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
}

impl CPlayerPed {
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
    pub fn notify_sub_objects<W: PlayerPedWorld + ?Sized>(&mut self, arg: u32, world: &mut W) -> u32 {
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
