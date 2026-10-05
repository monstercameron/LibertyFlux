//! The probe slot (vf2): confirm the object's key, publish the board tag.

use crate::Tag;

/// The lifted leaderboard object as the probe sees it: its key answer
/// (what the object's own key slot reports) and its per-board tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardObject {
    /// What the object's key slot reports.
    pub key: u32,
    /// The board's tag.
    pub tag: Tag,
}

/// Slot vf2: the board's tag when `expected` matches the object's key.
///
/// The 32-bit form returns the `out` address on a match and stores the
/// relocated tag through it; on a key mismatch, or when the receiver is
/// null, it stores nothing and returns null. The lift narrows the
/// address-valued result to its meaning: `Some(tag)` on a key match,
/// `None` otherwise; storing is the caller's job. (The differential test
/// pins the 32-bit return-address shape and the no-store cases.)
#[must_use]
pub fn query_tag(obj: &BoardObject, expected: u32) -> Option<Tag> {
    if obj.key != expected {
        return None;
    }
    Some(obj.tag)
}
