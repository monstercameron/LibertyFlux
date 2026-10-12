//! The composer's sequence and forwarding methods.
//!
//! The composer keeps three words these methods read: an immediate-mode
//! flag, a 16-bit sequence counter that advances by two, and a word that
//! the sequence method returns. Before the sequence step and before each
//! forward, the composer runs its own slot at offset 0x78 (a sync when
//! immediate mode is off), so the lifted methods call it through
//! [`ComposerCallees::sync`]. A forward then hands the composer's identity
//! to one callee, twice, with the method's own arguments in between.
//!
//! The composer's identity is an opaque handle: the lifted code passes it to
//! the callees and never reads through it. The rest of the composer's
//! fields are not held here; the lifted methods read and write only the
//! three words above.

use lf_core::boundary::Handle32;

/// Step by which the sequence counter advances.
pub const SEQUENCE_STEP: u16 = 2;

/// Identity of one composer.
pub type ComposerId = Handle32<ComposerOptimized>;

/// The composer's callees and its own slot, as the lifted methods call them.
pub trait ComposerCallees {
    /// The composer's slot at offset 0x78, called before the sequence step
    /// and before a forward while immediate mode is off. The verified
    /// rewrites name it a sync in the forwarders and a pool refill in the
    /// sequence methods; neither name is accepted.
    fn sync(&mut self, composer: Option<ComposerId>);

    /// Callee of the forward that pairs two words (the evidence names it
    /// apply_pose_direct; not accepted). The composer is passed first and
    /// last, as the original passes its own identity twice.
    fn apply_pose_direct(&mut self, composer: Option<ComposerId>, a0: u32, a1: u32, last: Option<ComposerId>);

    /// Callee of the second two-word forward (the evidence names it
    /// apply_static_tracks_to_pose; not accepted).
    fn apply_static_tracks_to_pose(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    );

    /// Callee of the first one-word forward, which passes zero as its
    /// second argument (the evidence names it copy_dof_pair; not accepted).
    fn copy_dof_pair(&mut self, composer: Option<ComposerId>, a0: u32, a1: u32, last: Option<ComposerId>);

    /// Callee of the second one-word forward, which passes zero as its
    /// second argument (the evidence names it setup_identity_frame; not
    /// accepted).
    fn setup_identity_frame(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    );
}

/// The words of a composer that these methods read and write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerOptimized {
    /// Immediate-mode flag at offset 0x98: nonzero turns the sync off.
    immediate: bool,
    /// Sequence counter at offset 0x72.
    sequence: u16,
    /// Word at offset 0x18, which the sequence method returns.
    word_18: u32,
}

impl ComposerOptimized {
    /// A composer with the given immediate flag, sequence counter and word.
    #[must_use]
    pub fn new(immediate: bool, sequence: u16, word_18: u32) -> Self {
        Self {
            immediate,
            sequence,
            word_18,
        }
    }

    /// The immediate-mode flag.
    #[must_use]
    pub fn immediate(&self) -> bool {
        self.immediate
    }

    /// The sequence counter.
    #[must_use]
    pub fn sequence(&self) -> u16 {
        self.sequence
    }

    /// The word at offset 0x18.
    #[must_use]
    pub fn word_18(&self) -> u32 {
        self.word_18
    }

    /// The sync, unless immediate mode is on.
    fn sync_unless_immediate<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        callees: &mut C,
    ) {
        if !self.immediate {
            callees.sync(composer);
        }
    }

    /// Advances the sequence counter, after the sync unless immediate.
    /// Returns nothing: the original's return is always zero.
    pub fn advance_sequence<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        callees: &mut C,
    ) {
        self.sync_unless_immediate(composer, callees);
        self.sequence = self.sequence.wrapping_add(SEQUENCE_STEP);
    }

    /// Advances the sequence counter as [`ComposerOptimized::advance_sequence`]
    /// does, then returns the word at offset 0x18.
    pub fn advance_and_read_word<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        callees: &mut C,
    ) -> u32 {
        self.sync_unless_immediate(composer, callees);
        self.sequence = self.sequence.wrapping_add(SEQUENCE_STEP);
        self.word_18
    }

    /// Forward with two words: the sync, then the first callee with the
    /// composer, the two words and the composer again.
    pub fn forward_two_words_pose<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        callees: &mut C,
    ) {
        self.sync_unless_immediate(composer, callees);
        callees.apply_pose_direct(composer, a0, a1, composer);
    }

    /// Forward with two words to the second callee, after the sync.
    pub fn forward_two_words_tracks<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        callees: &mut C,
    ) {
        self.sync_unless_immediate(composer, callees);
        callees.apply_static_tracks_to_pose(composer, a0, a1, composer);
    }

    /// Forward with one word and a zero to the first copy callee, after the
    /// sync.
    pub fn forward_one_word_copy<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        a0: u32,
        callees: &mut C,
    ) {
        self.sync_unless_immediate(composer, callees);
        callees.copy_dof_pair(composer, a0, 0, composer);
    }

    /// Forward with one word and a zero to the second setup callee, after
    /// the sync.
    pub fn forward_one_word_setup<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        a0: u32,
        callees: &mut C,
    ) {
        self.sync_unless_immediate(composer, callees);
        callees.setup_identity_frame(composer, a0, 0, composer);
    }
}
