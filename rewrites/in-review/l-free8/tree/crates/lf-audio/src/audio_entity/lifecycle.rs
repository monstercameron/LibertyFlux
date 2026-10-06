//! Audio entity construction and one-shot configuration.
//!
//! The constructor runs a base constructor, installs two table words,
//! clears a state word and constructs two embedded members. The base
//! and member constructors are unlifted code, so they arrive as the
//! [`BaseInit`] and [`MemberInit`] traits; the table words are
//! relocated addresses, pinned on the 32-bit side only (see
//! [`registry`](super::registry)).
//!
//! The latch slot copies a configuration word into the object the
//! first time it runs; the configuration global arrives as a plain
//! argument.

/// Builds the entity's base part. One method per unlifted callee the
/// constructor calls.
pub trait BaseInit {
    /// Runs the base constructor.
    fn init_base(&mut self);
}

/// Builds one embedded member in place.
pub trait MemberInit<M> {
    /// Constructs and returns one member.
    fn init_member(&mut self) -> M;
}

/// The constructed audio entity: a cleared state word plus two
/// embedded members. The two table words the 32-bit form installs are
/// identity, not data, and are not modelled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioEntity<M> {
    /// State word, always cleared by construction.
    pub state: u32,
    /// First embedded member, constructed before the second.
    pub member_a: M,
    /// Second embedded member.
    pub member_b: M,
}

impl<M> AudioEntity<M> {
    /// Constructs an entity: base first, then member A, then member B.
    #[must_use]
    pub fn new(base: &mut impl BaseInit, members: &mut impl MemberInit<M>) -> Self {
        base.init_base();
        let member_a = members.init_member();
        let member_b = members.init_member();
        Self {
            state: 0,
            member_a,
            member_b,
        }
    }
}

/// A configuration slot latched once from a global word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatchSlot {
    /// Latched configuration word.
    pub slot: u32,
    /// Whether the slot has been latched. The 32-bit flag byte
    /// narrows to this: any nonzero byte reads as latched.
    pub latched: bool,
}

impl LatchSlot {
    /// A fresh, unlatched slot. The 32-bit slot word starts with
    /// whatever the object held; the lift starts it at zero, which no
    /// routine reads before the first latch.
    #[must_use]
    pub fn fresh() -> Self {
        Self {
            slot: 0,
            latched: false,
        }
    }

    /// Latches `config` into the slot on the first call; later calls
    /// keep the first value.
    pub fn latch(&mut self, config: u32) {
        if !self.latched {
            self.slot = config;
        }
        self.latched = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Order(Vec<&'static str>);

    impl BaseInit for Order {
        fn init_base(&mut self) {
            self.0.push("base");
        }
    }

    impl MemberInit<u32> for Order {
        fn init_member(&mut self) -> u32 {
            let tag = if self.0.iter().any(|s| *s == "a") {
                "b"
            } else {
                "a"
            };
            self.0.push(tag);
            u32::from(tag == "b")
        }
    }

    #[test]
    fn construct_runs_base_then_members_in_order() {
        let mut order = Order(Vec::new());
        let entity = AudioEntity::new(&mut order, &mut order);
        assert_eq!(order.0, ["base", "a", "b"]);
        assert_eq!(entity.state, 0);
        assert_eq!((entity.member_a, entity.member_b), (0, 1));
    }

    #[test]
    fn latch_keeps_first_value() {
        let mut slot = LatchSlot::fresh();
        slot.latch(11);
        assert_eq!(slot.slot, 11);
        assert!(slot.latched);
        slot.latch(22);
        assert_eq!(slot.slot, 11);
    }

    #[test]
    fn latch_on_latched_slot_keeps_value() {
        let mut slot = LatchSlot {
            slot: 5,
            latched: true,
        };
        slot.latch(6);
        assert_eq!(slot.slot, 5);
    }
}
