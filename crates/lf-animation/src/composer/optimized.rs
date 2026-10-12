//! The composer's sequence, forwarding, table, gate and drain methods.
//!
//! The composer keeps the words these methods read: an immediate-mode flag,
//! a linked flag, a 16-bit sequence counter, a word that one method returns,
//! a free-node cache, the pending list and its counters, and the inner object
//! it forwards to. Before the sequence step and before most forwards, the
//! composer runs its own slot at offset 0x78, which is its verified method
//! vf30 (the pending-list drain, lifted as `drain_pending`). The lifted
//! methods reach it through [`ComposerCallees::sync`], so these methods do
//! not depend on the drain's code. The sync is skipped while immediate mode
//! is on, except where the original always runs it.
//!
//! The composer's slots at offsets 0x7C and 0x84 are its verified methods vf31
//! (the gate pass, lifted as `gate_pending`) and vf33 (the node handler, lifted
//! as `handle_pending_node`). A composition may route those callees to the
//! lifted code; the proofs script them.
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

/// Identity of one node of the composer's pool or pending list.
pub type NodeId = Handle32<PoolNode>;

/// The link word of each pool node: the node that follows it in whichever
/// list it is on. The original keeps this word at offset 4 of the node; the
/// lift holds only that word and the caller maps each node to its memory.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NodeLinks {
    next: std::collections::HashMap<NodeId, Option<NodeId>>,
}

impl NodeLinks {
    /// An empty set of nodes.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a node and its link word.
    pub fn insert(&mut self, node: NodeId, next: Option<NodeId>) {
        self.next.insert(node, next);
    }

    /// Number of recorded nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.next.len()
    }

    /// Whether no node is recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.next.is_empty()
    }

    /// The link word of `node`.
    ///
    /// # Panics
    ///
    /// When the node was never recorded: the original would read its memory,
    /// which the caller did not describe.
    #[must_use]
    pub fn next(&self, node: NodeId) -> Option<NodeId> {
        *self
            .next
            .get(&node)
            .expect("the node's link word was not recorded")
    }

    /// Writes the link word of `node`.
    ///
    /// # Panics
    ///
    /// When the node was never recorded.
    pub fn set_next(&mut self, node: NodeId, next: Option<NodeId>) {
        let slot = self
            .next
            .get_mut(&node)
            .expect("the node's link word was not recorded");
        *slot = next;
    }
}

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
    /// The composer's own slot at offset 0x78, which is its verified method
    /// vf30 (lifted as `drain_pending`). The sequence methods and the
    /// forwards call it while immediate mode is off; the drain-and-forward
    /// method calls it always.
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
    fn apply_pose_direct(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    );

    /// Callee of the second two-word forward (a proposed name is
    /// apply_static_tracks_to_pose; not accepted).
    fn apply_static_tracks_to_pose(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    );

    /// The composer's slot at offset 0x74: the method at index 29 of the
    /// class, not in the verified list, so its meaning is not established.
    /// The gate pass calls it first, always.
    fn sync_pending(&mut self, composer: Option<ComposerId>);

    /// The composer's slot at offset 0x84, which is its verified method vf33
    /// (lifted as `handle_pending_node`). The gate pass calls it once per node
    /// with the composer, the node and a word the caller passes through.
    /// Returns the answer; only its low byte is a verdict.
    fn gate_node(&mut self, composer: Option<ComposerId>, node: Option<NodeId>, arg: u32) -> u32;

    /// The composer's slot at offset 0x7C, which is its verified method vf31
    /// (lifted as `gate_pending`). The gate-and-drain method calls it with the
    /// composer and the out-word's identity (the original passes the out-word's
    /// address as the argument). Only the low byte of the answer is a verdict.
    fn check(&mut self, composer: Option<ComposerId>, token: Option<OutId>) -> u32;

    /// The composer's slot at offset 0x80: the method at index 32, not in the
    /// verified list, so its meaning is not established. The drain calls it
    /// once per node.
    fn handle_node(&mut self, composer: Option<ComposerId>, node: Option<NodeId>);

    /// Numbered callee 2 of the table dispatch: called on the table entry
    /// with five more arguments (two constants, the word at offset 0x70,
    /// the method's own argument and a constant). Its meaning is not
    /// established.
    fn handle_table_entry(
        &mut self,
        entry: Option<HandlerId>,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
    );

    /// Callee of the first one-word forward, which passes zero as its
    /// second argument (a proposed name is copy_dof_pair; not accepted).
    fn copy_dof_pair(
        &mut self,
        composer: Option<ComposerId>,
        a0: u32,
        a1: u32,
        last: Option<ComposerId>,
    );

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
    /// Head of the pending list at offset 0x80.
    pending_head: Option<NodeId>,
    /// Tail of the pending list at offset 0x84.
    pending_tail: Option<NodeId>,
    /// Counter at offset 0x90, cleared by the drain.
    count_90: u32,
    /// Counter at offset 0x94, cleared by the drain.
    count_94: u32,
    /// Staging word at offset 0x8C: the part of the pending list the gate pass is working through.
    stage: Option<NodeId>,
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
            pending_head: None,
            pending_tail: None,
            count_90: 0,
            count_94: 0,
            stage: None,
        }
    }

    /// Sets the pending list's head and tail (offsets 0x80 and 0x84).
    #[must_use]
    pub fn with_pending(mut self, head: Option<NodeId>, tail: Option<NodeId>) -> Self {
        self.pending_head = head;
        self.pending_tail = tail;
        self
    }

    /// Sets the staging word (offset 0x8C).
    #[must_use]
    pub fn with_stage(mut self, stage: Option<NodeId>) -> Self {
        self.stage = stage;
        self
    }

    /// The staging word.
    #[must_use]
    pub fn stage(&self) -> Option<NodeId> {
        self.stage
    }

    /// Sets the two counters (offsets 0x90 and 0x94).
    #[must_use]
    pub fn with_counts(mut self, count_90: u32, count_94: u32) -> Self {
        self.count_90 = count_90;
        self.count_94 = count_94;
        self
    }

    /// The pending list's head.
    #[must_use]
    pub fn pending_head(&self) -> Option<NodeId> {
        self.pending_head
    }

    /// The pending list's tail.
    #[must_use]
    pub fn pending_tail(&self) -> Option<NodeId> {
        self.pending_tail
    }

    /// The counter at offset 0x90.
    #[must_use]
    pub fn count_90(&self) -> u32 {
        self.count_90
    }

    /// The counter at offset 0x94.
    #[must_use]
    pub fn count_94(&self) -> u32 {
        self.count_94
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

    /// Drains the pending list back into the pool. The sync runs first,
    /// always. Then the list is detached (head and tail cleared) and walked
    /// from its head: each node is handed to the handler slot, and is pushed
    /// onto the free-node cache, whose old head it takes as its link. Both
    /// counters are cleared last. The original returns zero.
    ///
    /// The nodes' link words are the caller's: `links` holds each node the
    /// walk reads, and receives the new links the walk writes.
    ///
    /// # Panics
    ///
    /// When the walk reaches a node that `links` does not record, or the list
    /// is longer than the recorded nodes (a cycle): the original would read
    /// memory the caller did not describe, or never finish.
    pub fn drain_pending<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        links: &mut NodeLinks,
        callees: &mut C,
    ) {
        callees.sync_pending(composer);
        let mut node = self.pending_head.take();
        self.pending_tail = None;
        let mut steps = 0usize;
        while let Some(current) = node {
            steps += 1;
            assert!(
                steps <= links.len(),
                "the pending list is longer than the recorded nodes: it has a cycle"
            );
            let next = links.next(current);
            callees.handle_node(composer, Some(current));
            links.set_next(current, self.free_node);
            self.free_node = Some(current);
            node = next;
        }
        self.count_94 = 0;
        self.count_90 = 0;
    }

    /// Gate pass over the pending list. When nothing is staged, the sync
    /// runs, the pending list moves to the staging word and is detached;
    /// an empty list clears the counters and finishes at once. Then, node
    /// by node from the staging word, the gate is asked with the node and
    /// `arg`. A rejection stops the pass and leaves the staging word at that
    /// node. An acceptance moves the staging word to the node's successor,
    /// pushes the node onto the free-node cache, and finishes, clearing the
    /// counters, when nothing is left staged.
    ///
    /// Returns `true` when the pass finished and `false` when a gate
    /// rejected a node.
    ///
    /// # Panics
    ///
    /// When a node the pass reaches is not recorded in `links`.
    pub fn gate_pending<C: ComposerCallees + ?Sized>(
        &mut self,
        composer: Option<ComposerId>,
        arg: u32,
        links: &mut NodeLinks,
        callees: &mut C,
    ) -> bool {
        if self.stage.is_none() {
            callees.sync_pending(composer);
            self.stage = self.pending_head.take();
            self.pending_tail = None;
            if self.stage.is_none() {
                self.count_94 = 0;
                self.count_90 = 0;
                return true;
            }
        }
        let mut steps = 0usize;
        loop {
            steps += 1;
            assert!(
                steps <= links.len(),
                "the staged list is longer than the recorded nodes: it has a cycle"
            );
            let node = self.stage;
            let verdict = (callees.gate_node(composer, node, arg) as u8) != 0;
            if !verdict {
                return false;
            }
            let node = node.expect("the gate pass always has a staged node here");
            self.stage = links.next(node);
            links.set_next(node, self.free_node);
            self.free_node = Some(node);
            if self.stage.is_none() {
                self.count_94 = 0;
                self.count_90 = 0;
                return true;
            }
        }
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
        callees.handle_table_entry(Handle32::new(entry), 5, 6, u32::from(self.word_70), a0, 1);
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
