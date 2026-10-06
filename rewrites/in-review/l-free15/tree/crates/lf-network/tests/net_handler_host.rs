//! Host tests for the network handler slots: edge cases a reader of the
//! code would ask about, run on the 64-bit host.

use lf_core::boundary::Handle32;
use lf_network::net_handler::registry::{self, State};
use lf_network::net_handler::{Handler, HandlerSlots, HandlerTag};

fn handler(raw: u32) -> Option<Handler> {
    Handle32::<HandlerTag>::new(raw)
}

#[test]
fn new_slots_are_empty() {
    let slots = HandlerSlots::<11>::new();
    assert_eq!(slots.current(), None);
    for unit in 0..11 {
        assert_eq!(slots.saved(unit), None);
    }
    assert_eq!(HandlerSlots::<11>::default(), slots);
}

#[test]
fn swap_installs_and_answers_old() {
    let mut slots = HandlerSlots::<11>::new();
    // First install: nothing displaced, save cell stays clear.
    assert_eq!(slots.swap(3, handler(0xA11CE)), None);
    assert_eq!(slots.current(), handler(0xA11CE));
    assert_eq!(slots.saved(3), None);
    // Second install through another unit: the first handler is parked.
    assert_eq!(slots.swap(7, handler(0xB0B)), handler(0xA11CE));
    assert_eq!(slots.current(), handler(0xB0B));
    assert_eq!(slots.saved(7), handler(0xA11CE));
    // The first unit's cell is untouched by the second unit's swap.
    assert_eq!(slots.saved(3), None);
}

#[test]
fn swap_covers_the_zero_word() {
    let mut slots = HandlerSlots::<1>::new();
    // Installing nothing over nothing: all words stay zero.
    assert_eq!(slots.swap(0, None), None);
    assert_eq!(slots.current(), None);
    assert_eq!(slots.saved(0), None);
    // Installing nothing over a handler parks the handler.
    slots.swap(0, handler(9));
    assert_eq!(slots.swap(0, None), handler(9));
    assert_eq!(slots.current(), None);
    assert_eq!(slots.saved(0), handler(9));
}

#[test]
fn swap_same_value_still_parks() {
    let mut slots = HandlerSlots::<2>::new();
    slots.swap(0, handler(42));
    // Old and new agree, but the save cell is still written.
    assert_eq!(slots.swap(1, handler(42)), handler(42));
    assert_eq!(slots.saved(1), handler(42));
    assert_eq!(slots.saved(0), None);
}

#[test]
fn save_cells_are_per_unit() {
    let mut slots = HandlerSlots::<11>::new();
    for unit in 0..11 {
        let tag = 1000 + unit as u32;
        slots.swap(unit, handler(tag));
    }
    // Each cell holds what its own unit displaced: unit 0 displaced
    // nothing, unit k displaced unit k-1's install.
    assert_eq!(slots.saved(0), None);
    for unit in 1..11 {
        assert_eq!(slots.saved(unit), handler(1000 + unit as u32 - 1));
    }
    assert_eq!(slots.current(), handler(1010));
}

#[test]
fn single_cell_group_works() {
    // The lone instance with a slot of its own is the same type with N = 1.
    let mut slots = HandlerSlots::<1>::new();
    assert_eq!(slots.swap(0, handler(1)), None);
    assert_eq!(slots.swap(0, handler(2)), handler(1));
    assert_eq!(slots.saved(0), handler(1));
    assert_eq!(slots.current(), handler(2));
}

#[test]
#[should_panic]
fn swap_past_last_cell_panics() {
    let mut slots = HandlerSlots::<11>::new();
    let _ = slots.swap(11, handler(1));
}

#[test]
#[should_panic]
fn saved_past_last_cell_panics() {
    let slots = HandlerSlots::<11>::new();
    let _ = slots.saved(11);
}

#[test]
fn handler_zero_is_none() {
    assert!(handler(0).is_none());
    let h = handler(0x1234).unwrap();
    assert_eq!(h.get(), 0x1234);
    assert_eq!(Handle32::raw_or_zero(Some(h)), 0x1234);
    assert_eq!(Handle32::<HandlerTag>::raw_or_zero(None), 0);
}

#[test]
fn registry_counts_cover_the_list() {
    assert_eq!(registry::ROWS.len(), 18);
    assert_eq!(registry::counts(), (12, 0, 6));
    assert!(
        registry::ROWS
            .iter()
            .filter(|r| r.state == State::Proven)
            .all(|r| r.method == "HandlerSlots::swap")
    );
    assert!(
        registry::ROWS
            .iter()
            .filter(|r| r.state == State::Missing)
            .all(|r| !r.narrows.is_empty())
    );
}
