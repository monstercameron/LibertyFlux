//! The replay notifier control: a refresh the game state may suppress.
//!
//! The control's `this` touches only the word at `+0x04` (an object
//! pointer), while the bar reads the same offset as its seconds float:
//! two shapes under one name prefix, so the control is its own type. The
//! word at `+0x00` is never touched by the routine and is not modelled.

use lf_core::boundary::Handle32;

/// Identity of the control's inner object (the refresh target).
#[derive(Debug)]
pub struct InnerTag;

/// An opaque inner object: the control's refresh target.
pub type InnerHandle = Handle32<InnerTag>;

/// Identity of the shared notifier behind the hub.
#[derive(Debug)]
pub struct NotifierTag;

/// An opaque shared notifier: the hub's answer.
pub type NotifierHandle = Handle32<NotifierTag>;

/// Refreshes the control's inner object: the second callee.
///
/// The 32-bit callee takes the inner object and answers a word the
/// routine discards; the lift runs the refresh for its effect only (the
/// proof pins the discarded answer and the passed object).
pub trait RefreshInner {
    /// Runs the refresh.
    fn refresh(&mut self);
}

impl<F: FnMut()> RefreshInner for F {
    fn refresh(&mut self) {
        self();
    }
}

/// Looks up the shared notifier: the third callee plus its flag word.
///
/// The 32-bit form fetches twice (only the second answer is used) and
/// clears bit 0 of the flag byte beside the notifier directly; the lift
/// fetches twice through [`lookup`](Self::lookup) and clears through
/// [`clear_suppress`](Self::clear_suppress), so the proof compares both
/// fetches and the cleared flag in order.
pub trait NotifyHub {
    /// Fetches the notifier, or `None` for a null answer.
    fn lookup(&mut self) -> Option<NotifierHandle>;
    /// Clears the notifier's suppress flag (bit 0 of its flag byte).
    fn clear_suppress(&mut self, id: NotifierHandle);
}

/// What the refresh decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshOutcome {
    /// The probe fell below the bound: the probe value.
    BelowBound(u32),
    /// The game state suppresses refreshes: the state word.
    Suppressed(u32),
    /// The first notifier fetch answered null.
    NoNotifier,
    /// Refreshed through the notifier.
    Refreshed(NotifierHandle),
}

/// The notifier control: an inner object behind the `+0x04` word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotifyCtl {
    /// The inner object the refresh runs on.
    pub inner: InnerHandle,
}

impl NotifyCtl {
    /// Refreshes the shared notifier unless the game state suppresses it.
    ///
    /// Answers the probe when it falls below `bound`, or the state word
    /// when that state is in the suppressed set
    /// {2, 7, 8, 11, 12, 13, 14, 15, 16, 17}. Otherwise runs the refresh,
    /// fetches the notifier twice (a null first fetch ends the work),
    /// clears its suppress flag, and answers it.
    ///
    /// # Panics
    ///
    /// When the second fetch answers null (the original writes through
    /// the null pointer and faults).
    pub fn maybe_notify(
        &self,
        bound: u32,
        state: u32,
        probe: &mut impl super::bar::Sample,
        refresh: &mut impl RefreshInner,
        hub: &mut impl NotifyHub,
    ) -> RefreshOutcome {
        let t = probe.sample();
        if t < bound {
            return RefreshOutcome::BelowBound(t);
        }
        match state {
            2 | 7 | 8 | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 => {
                return RefreshOutcome::Suppressed(state);
            }
            _ => {}
        }
        refresh.refresh();
        if hub.lookup().is_none() {
            return RefreshOutcome::NoNotifier;
        }
        let id = hub
            .lookup()
            .expect("second notifier fetch answered null: the original faults there");
        hub.clear_suppress(id);
        RefreshOutcome::Refreshed(id)
    }
}
