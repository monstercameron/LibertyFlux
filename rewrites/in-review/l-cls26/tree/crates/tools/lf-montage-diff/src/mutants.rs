//! Deliberately wrong dispatchers, one plausible mistake per variant.
//!
//! Each variant is the lifted dispatcher with one change: an unsigned
//! compare where the original is signed, an off-by-one index rule, a
//! swapped gate order, a missed write, a stale loop count. The differential
//! cases must catch every one; a variant that passes would mean the cases
//! cannot see that change.

use lf_core::Handle32;
use lf_input_frontend::montage_container::{MontageContainer, MontageGlobals, MontageWorld, Tag};

/// The wrong dispatcher variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchMutant {
    /// The freshness threshold compares unsigned.
    FreshnessUnsigned,
    /// The cursor is compared against the count, not one less.
    CursorSeenOffByOne,
    /// The drop needs both mismatches, not either.
    DropNeedsBoth,
    /// The second gate runs before the first.
    GatesSwapped,
    /// The re-arm path leaves the phase byte set.
    PhaseStaysSet,
    /// The toggled refresh path leaves the mode byte set.
    ModeStaysSet,
    /// The list walk keeps its first count instead of re-reading it.
    CountNotReread,
    /// The adjacency test needs both equalities, not either.
    AdjacentNeedsBoth,
}

/// All wrong dispatcher variants.
pub const ALL: &[DispatchMutant] = &[
    DispatchMutant::FreshnessUnsigned,
    DispatchMutant::CursorSeenOffByOne,
    DispatchMutant::DropNeedsBoth,
    DispatchMutant::GatesSwapped,
    DispatchMutant::PhaseStaysSet,
    DispatchMutant::ModeStaysSet,
    DispatchMutant::CountNotReread,
    DispatchMutant::AdjacentNeedsBoth,
];

/// The dispatcher with one wrong step.
pub fn dispatch(
    container: &mut MontageContainer,
    world: &mut impl MontageWorld,
    globals: &MontageGlobals,
    m: DispatchMutant,
) -> u32 {
    if globals.state_flag == 0 {
        container.cursor = 0;
        container.pending = 0;
        return 0;
    }
    let session = world.open_session(globals.state_flag);
    world.own_open(container.own);
    if m == DispatchMutant::GatesSwapped {
        let gate2 = world.session_tag(session);
        let want2 = world.tag_id(Tag::D);
        if gate2 == want2 {
            return refresh(container, world, session, m);
        }
        let gate1 = world.session_tag(session);
        let want1 = world.tag_id(Tag::A);
        if gate1 == want1 {
            return list(container, world, m);
        }
    } else {
        let gate1 = world.session_tag(session);
        let want1 = world.tag_id(Tag::A);
        if gate1 == want1 {
            return list(container, world, m);
        }
        let gate2 = world.session_tag(session);
        let want2 = world.tag_id(Tag::D);
        if gate2 == want2 {
            return refresh(container, world, session, m);
        }
    }
    let gate3 = world.session_tag(session);
    let want3 = world.tag_id(Tag::J);
    if gate3 != want3 {
        return want3;
    }
    reconcile(container, world, m)
}

fn list(
    container: &mut MontageContainer,
    world: &mut impl MontageWorld,
    m: DispatchMutant,
) -> u32 {
    world.activate(Tag::B);
    world.lookup(Tag::C);
    let mut count = world.list_count(container.list);
    if count != 0 {
        let mut index = 0u32;
        loop {
            let elem = world.list_at(container.list, index);
            let word_a = world.elem_word_a(elem);
            world.emit_a(container.src, index, word_a);
            let word_b = world.elem_word_b(elem);
            world.emit_b(container.src, index, word_b);
            index = index.wrapping_add(1);
            if m != DispatchMutant::CountNotReread {
                count = world.list_count(container.list);
            }
            if index >= count {
                break;
            }
        }
    }
    count
}

#[allow(clippy::too_many_lines)]
fn refresh(
    container: &mut MontageContainer,
    world: &mut impl MontageWorld,
    session: Option<Handle32<lf_input_frontend::montage_container::SessionTag>>,
    m: DispatchMutant,
) -> u32 {
    let freshness = world.freshness(container.src);
    let trips = if m == DispatchMutant::FreshnessUnsigned {
        freshness >= 100
    } else {
        (freshness as i32) >= 100
    };
    if trips {
        world.note_teardown_mark();
        return world.state_teardown();
    }
    world.activate(Tag::E);
    world.state_refresh();
    if container.mode != 0 {
        if m != DispatchMutant::ModeStaysSet {
            container.mode = 0;
        }
        world.aux_set(container.aux, 0);
        let tail = world.list_tail(container.list);
        world.tail_set(tail, 1);
        world.signal_out(container.out, 0);
    } else {
        let inner = world.list_inner(container.list);
        let probe = world.probe_open(inner);
        world.probe_prime(probe);
        let inner_again = world.list_inner(container.list);
        let probe_again = world.probe_open(inner_again);
        world.probe_set(probe_again, 0);
    }
    world.lookup(Tag::F);
    world.prepare(Tag::G);
    let handle = world.tag_id(Tag::H);
    let worker = world.open_worker(container.own, session, handle);
    let first = world.session_first(worker);
    let key = world.list_rank(container.list, first);
    world.note_rank(container.own, key);
    world.note_session(container.own, worker);
    let first_again = world.session_first(worker);
    let key_again = world.list_rank(container.list, first_again);
    world.list_touch(container.list, key_again, 1);
    world.session_mode(worker, 1);
    world.list_mode(container.list, 1);
    world.note_sync(container.own);
    let inner_a = world.list_inner(container.list);
    let prop = world.prop_read(inner_a);
    world.cfg_set(container.cfg, prop);
    let inner_b = world.list_inner(container.list);
    let prop_again = world.prop_read(inner_b);
    let blend = world.blend_level(container.src, prop_again);
    let target = world.resolve_output(Tag::I, blend);
    world.release_output(target)
}

fn reconcile(
    container: &mut MontageContainer,
    world: &mut impl MontageWorld,
    m: DispatchMutant,
) -> u32 {
    container.phase = 3;
    let count_a = world.list_count(container.list);
    let seen_against = if m == DispatchMutant::CursorSeenOffByOne {
        count_a
    } else {
        count_a.wrapping_sub(1)
    };
    let cursor_seen = container.cursor == seen_against;
    let count_b = world.list_count(container.list);
    let cursor = container.cursor;
    let want_seen = container.want == count_b;
    let adjacent = if m == DispatchMutant::AdjacentNeedsBoth {
        cursor == container.want && cursor.wrapping_add(1) == container.want
    } else {
        cursor == container.want || cursor.wrapping_add(1) == container.want
    };
    let mismatch = if m == DispatchMutant::DropNeedsBoth {
        !cursor_seen && !want_seen
    } else {
        !cursor_seen || !want_seen
    };
    if container.pending != 0 && mismatch && !adjacent {
        world.drop_pending(container.own, container.pending);
        let dropped = container.pending;
        container.dropped = dropped;
        container.cursor = 0;
        container.pending = 0;
        return dropped;
    }
    world.list_touch(container.list, cursor, 1);
    world.view_set(container.view, 0);
    let sub = world.cfg_sub(container.cfg);
    world.cfg_sub_set(sub, 0);
    let answer = world.own_rearm(container.own, 1);
    if m != DispatchMutant::PhaseStaysSet {
        container.phase = 0;
    }
    container.cursor = 0;
    container.pending = 0;
    answer
}
