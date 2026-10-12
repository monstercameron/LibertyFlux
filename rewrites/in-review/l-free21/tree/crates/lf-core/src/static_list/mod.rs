//! Static intrusive lists: a list head word and the link words of nodes.
//!
//! The original keeps several singly linked lists whose nodes are static
//! data (objects in the executable's image, not allocated at run time). A
//! list is one head word that holds the link of the first node, or the empty
//! link. Each node carries one or more link words; a link word holds the
//! same kind of value as a head word. Lifted code keeps every such word in a
//! [`StaticLinks`] table and refers to words by [`WordId`] and to nodes by
//! [`NodeId`], so no address appears here and nothing depends on pointer
//! width.
//!
//! Specification of the operation in this module:
//!
//! - [`StaticLinks::replace_head`] takes a node and pushes it onto a list.
//!   The value the head held is copied into a save word (a link word of the
//!   node, or a separate word the caller names), the head is set to name the
//!   node, and the value the head held before is returned. The save word is
//!   written before the head, and the two words are distinct, so the order
//!   of the two writes is not observable.
//! - A link word that is empty is `None`. A word names at most one node.
//! - Nothing here loops or allocates per call. An id outside the table is a
//!   caller error and panics: the original would have touched memory outside
//!   the list, and the lift fails loudly instead of guessing.

/// Index of one link word in a [`StaticLinks`] table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WordId(pub usize);

/// Index of one static node. A node is a value that link words can name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// The value of a link word: the empty link, or the node it names.
pub type Link = Option<NodeId>;

/// The link words of a set of static lists, owned as one table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticLinks {
    words: Vec<Link>,
}

impl StaticLinks {
    /// A table of `count` link words, all empty.
    #[must_use]
    pub fn new(count: usize) -> Self {
        Self {
            words: vec![None; count],
        }
    }

    /// The number of link words in the table.
    #[must_use]
    pub fn len(&self) -> usize {
        self.words.len()
    }

    /// True when the table holds no words.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// The value of one link word.
    ///
    /// # Panics
    ///
    /// When `id` is outside the table.
    #[must_use]
    pub fn word(&self, id: WordId) -> Link {
        self.words[id.0]
    }

    /// Sets one link word.
    ///
    /// # Panics
    ///
    /// When `id` is outside the table.
    pub fn set_word(&mut self, id: WordId, link: Link) {
        self.words[id.0] = link;
    }

    /// Pushes `node` onto the list whose head is `head`.
    ///
    /// The head's value is copied into `save` (the node's link word, or a
    /// separate save word), the head is set to name `node`, and the value the
    /// head held is returned. `save` and `head` must be different words.
    ///
    /// # Panics
    ///
    /// When `head` or `save` is outside the table, or when they are the same
    /// word.
    pub fn replace_head(&mut self, head: WordId, save: WordId, node: NodeId) -> Link {
        assert!(head != save, "a list head cannot be its own save word");
        let old = self.words[head.0];
        self.words[save.0] = old;
        self.words[head.0] = Some(node);
        old
    }
}

#[cfg(test)]
mod tests {
    use super::{Link, NodeId, StaticLinks, WordId};

    const HEAD: WordId = WordId(0);
    const SAVE: WordId = WordId(1);
    const FIRST: NodeId = NodeId(0);
    const SECOND: NodeId = NodeId(1);

    #[test]
    fn empty_head_returns_empty_and_names_the_node() {
        let mut links = StaticLinks::new(2);
        let old = links.replace_head(HEAD, SAVE, FIRST);
        assert_eq!(old, None);
        assert_eq!(links.word(HEAD), Some(FIRST));
        assert_eq!(links.word(SAVE), None);
    }

    #[test]
    fn second_push_saves_the_previous_head_in_the_save_word() {
        let mut links = StaticLinks::new(2);
        links.replace_head(HEAD, SAVE, FIRST);
        let old = links.replace_head(HEAD, SAVE, SECOND);
        assert_eq!(old, Some(FIRST));
        assert_eq!(links.word(HEAD), Some(SECOND));
        assert_eq!(links.word(SAVE), Some(FIRST));
    }

    #[test]
    fn pushing_the_node_already_at_the_head_saves_it_into_its_link() {
        let mut links = StaticLinks::new(2);
        links.set_word(HEAD, Some(FIRST));
        let old = links.replace_head(HEAD, SAVE, FIRST);
        assert_eq!(old, Some(FIRST));
        assert_eq!(links.word(HEAD), Some(FIRST));
        assert_eq!(links.word(SAVE), Some(FIRST));
    }

    #[test]
    fn words_other_than_head_and_save_are_untouched() {
        let mut links = StaticLinks::new(3);
        let other: Link = Some(SECOND);
        links.set_word(WordId(2), other);
        links.replace_head(HEAD, SAVE, FIRST);
        assert_eq!(links.word(WordId(2)), other);
        assert_eq!(links.len(), 3);
        assert!(!links.is_empty());
    }

    #[test]
    #[should_panic(expected = "its own save word")]
    fn head_and_save_must_differ() {
        let mut links = StaticLinks::new(1);
        links.replace_head(HEAD, HEAD, FIRST);
    }
}
