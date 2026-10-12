//! The composer's sequence, forwarding and stack-read methods.
//!
//! The composer keeps the words these methods read: an immediate-mode flag,
//! a linked flag, a 16-bit sequence counter that advances by two, a word that
//! one method returns, a free-node cache, and the inner object it forwards
//! to. Before the sequence step, before most forwards and before the stack
//! read, the composer runs its own slot at offset 0x78 (a sync or pool
//! refill, which the verified rewrites name differently), so the lifted
//! methods call it through [`ComposerCallees::sync`]. The sync is skipped
//! while immediate mode is on, except where the original always runs it.
//!
//! The composer's identity and its inner object are opaque handles: the
//! lifted code passes them to the callees and reads through them only where
//! [`ComposerOptimized::peek_or_pop`] is handed the inner stack itself.

use lf_core::boundary::Handle32;

/// Step by which the sequence counter advances.
pub const SEQUENCE_STEP: u16 = 2;

/// Identity of one composer.
pub type ComposerId = Handle32<ComposerOptimized>;

/// Identity of a free node in the composer's pool.
pub struct PoolNode;

/// The inner object a composer forwards to, and that the stack read works
/// on: a table of words and the index of its top item.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemStack {
    /// The table's words.
    pub table: Vec<u32>,
    /// Index of the current item.
    pub top: u32,
}

/// Identity of the inner object.
pub type InnerId = Handle32<ItemStack>;

/// Identity of a word the gate writes through: the caller's out-word.
pub struct OutWord;

/// Identity of an out-word.
pub type OutId = Handle32<OutWord>;

/// Identity of a pending node.
pub struct PendingNode;

/// Identity of one pending node.
pub type NodeId = Handle32<PendingNode>;

/// Step by which the table dispatch advances the sequence counter. The
/// sequence methods above advance by [`SEQUENCE_STEP`]; this one is not the
/// same routine and advances by one.
pub const DISPATCH_STEP: u16 = 1;

/// Identity of a table handler: the object a table entry names.
pub struct TableHandler;

/// Identity of one table handler.
pub type HandlerId = Handle32<TableHandler>;

/// Flag bit read from the mode word: when its low byte is zero the stack
/// read pops the current item.
pub const POP_BYTE_ZERO: u8 = 0;

/// The composer's callees and its own slots, as the lifted methods call them.
pub trait ComposerCallees {
    /// The composer's slot at offset 0x78. The sequence methods and the
    /// forwards call it while immediate mode is off; the drain-and-forward
    /// method calls it always. The verified rewrites name it a sync in some
    /// places and a pool refill in others; neither name is accepted.
    fn sync(&mut self, composer: Option<ComposerId>);

    /// Numbered callee 2 of the drain-and-forward method: called on the
    /// composer. A proposed name is apply_static_tracks, an unverified
    /// method of the same class; that name is not accepted.
    fn drain(&mut self, composer: Option<ComposerId>);

    /// The inner object's forward slot (offset 0x08 of its vtable), called
    /// on the inner object. Returns the forward's answer.
    fn forward_inner(&mut self, inner: Option<InnerId>) -> u32;

    /// Callee of the forward that pairs two words (a proposed name is
    /// apply_pose_direct; not accepted). The composer is passed first and
    /// last, as the original passes its own identity twice.
    fn apply_pose_direct(&mut self, composer: Option<ComposerId>, a0: u32, a1: u32, last: Option<ComposerId>);

    /// Callee of the second two-word forward (a proposed name is
    /// apply_static_tracks_to_pose; not accepted).
    fn apply_static_tracks_to_pose(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    );

    /// The composer's gate slot at offset 0x7C, called with the composer and
    /// the out-word's identity (the original passes the out-word's address
    /// as an argument). Returns the gate's answer; only its low byte is a
    /// verdict.
    fn check(&mut self, composer: Option<ComposerId>, token: Option<OutId>) -> u32;

    /// The composer's node-handler slot at offset 0x80, called with the
    /// composer and one pending node.
    fn handle_node(&mut self, composer: Option<ComposerId>, node: Option<NodeId>);

    /// Numbered callee 2 of the table dispatch: called on the table entry
    /// with five more arguments (two constants, the word at offset 0x70,
    /// the method's own argument and a constant). Its meaning is not
    /// established.
    fn handle_table_entry(&mut self, entry: Option<HandlerId>, a: u32, b: u32, c: u32, d: u32, e: u32);

    /// Callee of the first one-word forward, which passes zero as its
    /// second argument (a proposed name is copy_dof_pair; not accepted).
    fn copy_dof_pair(&mut self, composer: Option<ComposerId>, a0: u32, a1: u32, last: Option<ComposerId>);

    /// Callee of the second one-word forward, which passes zero as its
    /// second argument (a proposed name is setup_identity_frame; not
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
    /// Linked flag at offset 0x11: nonzero makes the drain-and-forward
    /// method return at once.
    linked: bool,
    /// Sequence counter at offset 0x72.
    sequence: u16,
    /// Word at offset 0x18, which the sequence method returns.
    word_18: u32,
    /// Free-node cache at offset 0x88.
    free_node: Option<Handle32<PoolNode>>,
    /// Inner object at offset 0x0C.
    inner: Option<InnerId>,
    /// Word at offset 0x70, which the table dispatch passes on as a 16-bit value.
    word_70: u16,
}

impl ComposerOptimized {
    /// A composer with the given words.
    #[must_use]
    pub fn new(
        immediate: bool,
        linked: bool,
        sequence: u16,
        word_18: u32,
        free_node: Option<Handle32<PoolNode>>,
        inner: Option<InnerId>,
    ) -> Self {
        Self {
            immediate,
            linked,
            sequence,
            word_18,
            free_node,
            inner,
            word_70: 0,
        }
    }

    /// The immediate-mode flag.
    #[must_use]
    pub fn immediate(&self) -> bool {
        self.immediate
    }

    /// The linked flag.
    #[must_use]
    pub fn linked(&self) -> bool {
        self.linked
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

    /// The free-node cache.
    #[must_use]
    pub fn free_node(&self) -> Option<Handle32<PoolNode>> {
        self.free_node
    }

    /// Sets the word at offset 0x70, which only the table dispatch reads.
    #[must_use]
    pub fn with_word_70(mut self, word_70: u16) -> Self {
        self.word_70 = word_70;
        self
    }

    /// The word at offset 0x70.
    #[must_use]
    pub fn word_70(&self) -> u16 {
        self.word_70
    }

    /// The inner object.
    #[must_use]
    pub fn inner(&self) -> Option<InnerId> {
        self.inner
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

    /// Drain and forward: returns at once when linked; otherwise the sync
    /// (always, not only when immediate mode is off), the drain, the
    /// free-node cache cleared, and the inner object's forward, whose answer
    /// is returned.
    ///
    /// # Panics
    ///
    /// When the composer has no inner object. The original would call
    /// through a null object, so this case is outside the domain.
    pub fn drain_and_forward<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        callees: &mut C,
    ) -> u32 {
        if self.linked {
            return 0;
        }
        callees.sync(composer);
        callees.drain(composer);
        self.free_node = None;
        let inner = self.inner.expect("drain and forward needs an inner object");
        callees.forward_inner(Some(inner))
    }

    /// Gate, then drain and forward: the gate is asked first, with the
    /// out-word's identity. A rejecting verdict returns `false` and changes
    /// nothing. An accepting verdict runs the drain, clears the free-node
    /// cache, forwards to the inner object, clears the linked flag and
    /// writes zero to the out-word, then returns `true`.
    ///
    /// `out` is the word the out-word identity names; the caller passes it,
    /// since the lift does not reach memory through a handle.
    ///
    /// # Panics
    ///
    /// When the composer has no inner object, or the gate accepts with no
    /// out-word: the original would call or write through null.
    pub fn gate_drain_forward<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        token: Option<OutId>,
        out: &mut u32,
        callees: &mut C,
    ) -> bool {
        let verdict = (callees.check(composer, token) as u8) != 0;
        if !verdict {
            return false;
        }
        assert!(token.is_some(), "an accepting gate needs an out-word");
        callees.drain(composer);
        self.free_node = None;
        let inner = self.inner.expect("gate and forward needs an inner object");
        callees.forward_inner(Some(inner));
        self.linked = false;
        *out = 0;
        true
    }

    /// Node handler: hands `node` to the handler slot, then writes zero to
    /// the done word the caller passes (`done`), and reports success. The
    /// original returns one.
    pub fn handle_pending_node<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        node: Option<NodeId>,
        done: &mut u32,
        callees: &mut C,
    ) -> bool {
        callees.handle_node(composer, node);
        *done = 0;
        true
    }

    /// Table dispatch: the sync unless immediate mode is on; the sequence
    /// counter advances by [`DISPATCH_STEP`]; the table entry at the stack's
    /// top is handed to the handler callee with the fixed arguments, the
    /// word at offset 0x70 and the method's own argument. Returns nothing:
    /// the original's return is always zero.
    ///
    /// `stack` is the inner object the composer's inner word names; the
    /// caller passes it, as for [`ComposerOptimized::peek_or_pop`].
    ///
    /// # Panics
    ///
    /// When the top index is past the end of the table, which the original
    /// would read beyond.
    pub fn dispatch_table_entry<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        stack: &ItemStack,
        callees: &mut C,
    ) {
        self.sync_unless_immediate(composer, callees);
        self.sequence = self.sequence.wrapping_add(DISPATCH_STEP);
        let entry = stack.table[stack.top as usize];
        callees.handle_table_entry(
            Handle32::new(entry),
            5,
            6,
            u32::from(self.word_70),
            a0,
            1,
        );
    }

    /// Stack read: the sync unless immediate mode is on, then the item at
    /// the stack's top is returned. When the low byte of `mode` is zero the
    /// top index is first moved down by one; the item read is the one that
    /// was on top before the move.
    ///
    /// `stack` is the inner object the composer's inner word names. The
    /// lift does not reach it through the handle: the caller passes it.
    ///
    /// # Panics
    ///
    /// When the top index is past the end of the table. The original would
    /// read beyond the table, so this case is outside the domain.
    pub fn peek_or_pop<C: ComposerCallees + ?Sized>(
        &self,
        composer: Option<ComposerId>,
        mode: u32,
        stack: &mut ItemStack,
        callees: &mut C,
    ) -> u32 {
        self.sync_unless_immediate(composer, callees);
        let top = stack.top;
        if (mode as u8) == POP_BYTE_ZERO {
            stack.top = top.wrapping_sub(1);
        }
        stack.table[top as usize]
    }
}
