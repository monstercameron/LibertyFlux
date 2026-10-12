//! The complex melee task object and its lifted methods.
//!
//! The fields are the ones the lifted methods read or write, named for
//! what they hold where a lifted method interprets them and by offset
//! where the lift only carries them. Offsets are given in the doc comments
//! as offsets from the start of the task's object; they describe the layout
//! the proof checks against and are not used by the lifted code itself.

use lf_core::boundary::Handle32;

use super::host::{CtorArgs, MeleeHost, StartArgs};

/// The kind tag the initialise and follow-up methods pass to their callees.
pub const KIND_MELEE: u32 = 0x1B1;

/// The second sub-task answer the follow-up method accepts. The first is
/// [`KIND_MELEE`].
pub const FOLLOW_UP_ALT: u32 = 0x11D;

/// The disable bit of the flag byte at offset `0xF0`: when it is set, the
/// follow-up method answers 0 without asking the sub-task.
pub const DISABLE_BIT: u8 = 0x20;

/// The base every constructed flag byte carries: bit 7 set, the rest from
/// the constructor's flag words.
pub const FLAG_BASE: u8 = 0x80;

/// The bit of the destroy flags word that asks for the task to be freed.
pub const FREE_BIT: u32 = 1;

/// The offsets, from the start of the task, of the ten sub-object slots
/// the constructor has reset, in the order it resets them: one slot at
/// `0x54`, seven slots twelve bytes apart from `0x60`, and one slot each
/// at `0xB4` and `0xC0`.
pub const SLOT_OFFSETS: [u32; 10] = [
    0x54, 0x60, 0x6C, 0x78, 0x84, 0x90, 0x9C, 0xA8, 0xB4, 0xC0,
];

/// One sub-object slot: twelve bytes the constructor's reset call owns.
/// The lift carries the bytes; only the host writes them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResetSlot(pub [u8; 12]);

/// The task the melee task holds as its sub-task. The type is never built:
/// it only names what the sub-task handle identifies, and the lift never
/// reads through the handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubTask;

/// The complex melee task: the words its lifted methods touch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MeleeTask {
    /// The sub-task handle (offset `0x8`).
    pub sub_task: Option<Handle32<SubTask>>,
    /// Three words the constructor zeroes (offsets `0x20`, `0x24`, `0x28`).
    pub word_20: u32,
    /// See [`MeleeTask::word_20`].
    pub word_24: u32,
    /// See [`MeleeTask::word_20`].
    pub word_28: u32,
    /// The word the start method forwards to the melee starter, stored by
    /// the constructor (offset `0x30`).
    pub mode_word: u32,
    /// A word the constructor zeroes (offset `0x34`).
    pub word_34: u32,
    /// Two words the constructor copies from global state, carried
    /// opaquely: no lifted method interprets them (offsets `0x38`, `0x3C`).
    pub word_38: u32,
    /// See [`MeleeTask::word_38`].
    pub word_3c: u32,
    /// The ready flag: the constructor sets it to 1 (offset `0x40`).
    pub flag_40: u8,
    /// The byte past the ready flag, zeroed by the constructor and never
    /// interpreted (offset `0x41`).
    pub filler_41: u8,
    /// Two words the constructor zeroes (offsets `0x44`, `0x48`).
    pub word_44: u32,
    /// See [`MeleeTask::word_44`].
    pub word_48: u32,
    /// A half word the constructor zeroes (offset `0x4C`).
    pub half_4c: u16,
    /// A word the constructor zeroes (offset `0x50`).
    pub word_50: u32,
    /// The ten sub-object slots (offsets `0x54` to `0xCC`, twelve bytes
    /// each, in the order of [`SLOT_OFFSETS`]).
    pub slots: [ResetSlot; 10],
    /// Six words the constructor zeroes (offsets `0xCC` to `0xE0`); the
    /// initialise method later stores its two global words into the last
    /// two of them.
    pub word_cc: u32,
    /// See [`MeleeTask::word_cc`].
    pub word_d0: u32,
    /// See [`MeleeTask::word_cc`].
    pub word_d4: u32,
    /// See [`MeleeTask::word_cc`].
    pub word_d8: u32,
    /// See [`MeleeTask::word_cc`].
    pub word_dc: u32,
    /// See [`MeleeTask::word_cc`].
    pub word_e0: u32,
    /// The initialise flag: the constructor zeroes it and the initialise
    /// method sets it to 1 (offset `0xE4`).
    pub flag_e4: u8,
    /// The byte past the initialise flag, zeroed by the constructor and
    /// never interpreted (offset `0xE5`).
    pub filler_e5: u8,
    /// Two words the constructor zeroes (offsets `0xE8`, `0xEC`).
    pub word_e8: u32,
    /// See [`MeleeTask::word_e8`].
    pub word_ec: u32,
    /// The flag byte: the constructor composes it from its flag words, the
    /// start method splits four of its bits to the melee starter, and the
    /// follow-up method tests its disable bit (offset `0xF0`).
    pub flags_f0: u8,
    /// The high flag byte: the constructor keeps the high nibble of the
    /// storage's byte and clears the low nibble (offset `0xF1`).
    pub flags_f1: u8,
}

impl MeleeTask {
    /// Builds a melee task: the base constructor first, then the zeroed
    /// words, the ten slot resets in [`SLOT_OFFSETS`] order, the composed
    /// flag byte, the target-node setter with the last two arguments in
    /// order, and finally the two stamp words and the ready flag.
    ///
    /// Only the low bit of each flag word reaches the flag byte: the second
    /// word's bit becomes bit 1, the third's bit 2, the fourth's bit 0 and
    /// the fifth's bit 3, over [`FLAG_BASE`].
    #[must_use]
    pub fn new<H: MeleeHost>(host: &mut H, args: &CtorArgs) -> Self {
        let mut task = Self {
            sub_task: args.sub_task,
            ..Self::default()
        };
        host.construct_base(&mut task);
        task.mode_word = args.initial_word;
        for offset in SLOT_OFFSETS {
            host.reset_slot(&mut task, offset);
        }
        task.flags_f0 = FLAG_BASE
            | (((args.flag_five as u8) & 1) << 3)
            | (((args.flag_three as u8) & 1) << 2)
            | (((args.flag_two as u8) & 1) << 1)
            | ((args.flag_four as u8) & 1);
        task.flags_f1 = args.incoming_flags_high & 0xF0;
        host.set_target_node(&mut task, args.target_first, args.target_second);
        let stamps = host.stamp_words();
        task.word_38 = stamps.word_38;
        task.word_3c = stamps.word_3c;
        task.flag_40 = 1;
        task
    }

    /// Destroys the task: the destructor always runs, and when the low bit
    /// of `flags` is set the task is freed through the allocator's manager
    /// word, which is read only then.
    pub fn destroy<H: MeleeHost>(&mut self, host: &mut H, flags: u32) {
        host.destroy_task(self);
        if flags & FREE_BIT != 0 {
            let manager = host.manager();
            host.free_task(manager);
        }
    }

    /// Starts the task through the task manager. A null handle ends the
    /// start quietly. Otherwise the flag byte is split (it was read before
    /// the mode calls) and the mode word (read after them) goes with the
    /// four bits and the flag helper's answer to the melee starter.
    pub fn start<H: MeleeHost>(&mut self, host: &mut H) {
        let manager = host.manager();
        let handle = host.task_handle(manager);
        if handle == 0 {
            return;
        }
        let flags = self.flags_f0;
        let mode = host.mode_value(self);
        let computed = host.flag_value(self, mode & 0xFF);
        host.start_melee(&StartArgs {
            handle,
            mode_word: self.mode_word,
            flag_bit1: u32::from((flags >> 1) & 1),
            flag_bit2: u32::from((flags >> 2) & 1),
            flag_bit0: u32::from(flags & 1),
            flag_bit3: u32::from((flags >> 3) & 1),
            computed,
        });
    }

    /// Initialises the task: the two timing words are stored, the
    /// initialise flag is set, and the target setup and the registration
    /// (with the kind tag and 1) run in order.
    pub fn initialize<H: MeleeHost>(&mut self, host: &mut H, arg: u32) {
        let words = host.timing_words();
        self.word_dc = words.word_dc;
        self.word_e0 = words.word_e0;
        self.flag_e4 = 1;
        host.setup_target(self, arg);
        host.register(self, arg, KIND_MELEE, 1);
    }

    /// Answers the follow-up request: after the reference cleanup, a set
    /// disable bit answers 0, and otherwise the sub-task is queried and
    /// only its [`KIND_MELEE`] and [`FOLLOW_UP_ALT`] answers reach the
    /// follow-up sink, whose answer is returned.
    pub fn follow_up<H: MeleeHost>(&mut self, host: &mut H, arg: u32) -> u32 {
        host.release_refs(self);
        if self.flags_f0 & DISABLE_BIT != 0 {
            return 0;
        }
        let answer = host.subtask_query(self.sub_task);
        if answer != FOLLOW_UP_ALT && answer != KIND_MELEE {
            return 0;
        }
        host.follow_up_sink(self, arg, KIND_MELEE, 0)
    }
}
