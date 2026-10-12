//! Host tests of the lifted mirror-reflection render phase.
//!
//! They run on every host. A recording host answers each collaborator from a
//! queue and keeps the object words the phase reads and writes, so the tests
//! can check the calls the phase makes, in order, and the words it leaves.

use std::collections::{HashMap, VecDeque};

use lf_renderer::mirror_phase::registry::{self, LiftState};
use lf_renderer::mirror_phase::{MirrorGlobals, MirrorHost, MirrorReflectionPhase, Ref, Word};

/// A recording host: answers from per-name queues, keeps object words.
#[derive(Default)]
struct Fake {
    calls: Vec<(&'static str, Vec<u32>)>,
    answers: HashMap<&'static str, VecDeque<u32>>,
    words: HashMap<(u8, u32, u32), u32>,
    matrix: [u32; 4],
}

fn key(object: Ref, offset: u32) -> (u8, u32, u32) {
    match object {
        Ref::Phase(base) => (0, base, offset),
        Ref::Snapshot(base) => (1, base, offset),
        Ref::Handle(base) => (2, base, offset),
    }
}

impl Fake {
    fn answer(&mut self, name: &'static str, values: &[u32]) {
        self.answers.insert(name, values.iter().copied().collect());
    }

    fn call(&mut self, name: &'static str, args: Vec<u32>) -> u32 {
        self.calls.push((name, args));
        self.answers
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }

    fn names(&self) -> Vec<&'static str> {
        self.calls.iter().map(|(n, _)| *n).collect()
    }
}

impl MirrorHost for Fake {
    fn word(&mut self, object: Ref, offset: u32) -> u32 {
        *self.words.get(&key(object, offset)).unwrap_or(&0)
    }
    fn byte(&mut self, object: Ref, offset: u32) -> u8 {
        (self.word(object, offset & !3) >> (8 * (offset & 3))) as u8
    }
    fn set_word(&mut self, object: Ref, offset: u32, value: u32) {
        self.words.insert(key(object, offset), value);
        self.calls.push(("set_word", vec![offset, value]));
    }
    fn destruct(&mut self, _this: Ref) {
        self.call("destruct", vec![]);
    }
    fn free_storage(&mut self, _this: Ref) {
        self.call("free_storage", vec![]);
    }
    fn probe_gate(&mut self, _d: Ref) -> u32 {
        self.call("probe_gate", vec![])
    }
    fn query(&mut self, kind: u32, sub: u32) -> u32 {
        self.call("query", vec![kind, sub])
    }
    fn apply_six(&mut self, _q: Ref, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
        self.call("apply_six", vec![a1, a2, a3, a4, a5, a6])
    }
    fn release(&mut self, _h: Ref) {
        self.call("release", vec![]);
    }
    fn fetch_floats(&mut self) -> u32 {
        self.call("fetch_floats", vec![])
    }
    fn feed_four(&mut self, _q: Ref, flag: u32, _block: &mut [u32; 4]) -> u32 {
        self.call("feed_four", vec![flag])
    }
    fn toggle(&mut self, _q: Ref, on: u32) -> u32 {
        self.call("toggle", vec![on])
    }
    fn pair(&mut self, _q: Ref, a: u32, b: u32) -> u32 {
        self.call("pair", vec![a, b])
    }
    fn find_player(&mut self, zero: u32) -> u32 {
        self.call("find_player", vec![zero])
    }
    fn select(&mut self, _slot: Ref, a: u32, selector: u32) -> u32 {
        self.call("select", vec![a, selector])
    }
    fn commit(&mut self, _t: Ref) -> u32 {
        self.call("commit", vec![])
    }
    fn slot30(&mut self, _o: Ref, arg: Word) -> u32 {
        let value = match arg {
            Word::Value(v) => v,
            Word::PhaseTagged(b) => 0xF000_0000 | u32::from(b),
        };
        self.call("slot30", vec![value])
    }
    fn bind(&mut self, _d: Ref, _v: Ref) -> u32 {
        self.call("bind", vec![])
    }
    fn reduce(&mut self, _p: Ref, selector: u32, _frame: &mut [u32; 8]) -> u32 {
        self.call("reduce", vec![selector])
    }
    fn describe(&mut self, _half: &mut [u32; 4]) -> u32 {
        self.call("describe", vec![])
    }
    fn describe_with(&mut self, _half: &mut [u32; 4], _d: Ref) -> u32 {
        self.call("describe_with", vec![])
    }
    fn combine(&mut self, selector: u32, _b: Ref, _half: &mut [u32; 4], _p: Ref, _z: u32) -> u32 {
        self.call("combine", vec![selector])
    }
    fn finish(&mut self, _half: &mut [u32; 4]) -> u32 {
        self.call("finish", vec![])
    }
    fn fallback(&mut self, _zero: u32) -> u32 {
        self.call("fallback", vec![])
    }
    fn current_matrix(&mut self) -> [u32; 4] {
        self.call("current_matrix", vec![]);
        self.matrix
    }
    fn project(&mut self, _p: Ref, _m: &mut [u32; 4]) -> u32 {
        self.call("project", vec![])
    }
    fn set_six(
        &mut self,
        _d: Ref,
        _a1: u32,
        _a2: u32,
        _a3: u32,
        _a4: u32,
        _a5: u32,
        _a6: u32,
    ) -> u32 {
        self.call("set_six", vec![])
    }
    fn fresh_matrix(&mut self) -> [u32; 4] {
        self.call("fresh_matrix", vec![]);
        self.matrix
    }
    fn scale_project(&mut self, _d: Ref, _m: &mut [u32; 4]) -> u32 {
        self.call("scale_project", vec![])
    }
    fn rebuild_target(&mut self, _a: Ref, _b: Ref, _c: Ref, _p: Ref, _one: u32, _zero: u32) {
        self.call("rebuild_target", vec![]);
    }
    fn fetch_target(&mut self, _s: Ref) -> u32 {
        self.call("fetch_target", vec![])
    }
    fn lookup_object(&mut self, zero: u32) -> u32 {
        self.call("lookup_object", vec![zero])
    }
    fn footer(&mut self) -> u32 {
        self.call("footer", vec![])
    }
}

fn seeded(target: u32, params: u32) -> Fake {
    let mut host = Fake::default();
    host.words.insert((0, 0, 0x938), target);
    host.words.insert((0, 0, 0x940), params);
    host
}

/// Globals with every gate open and nothing else set.
fn open_globals() -> MirrorGlobals {
    MirrorGlobals {
        render_enabled: 1,
        render_skipped: 0,
        select_flag_a: 0,
        select_flag_b: 0,
        float_block_enabled: 0,
        scale_enabled: 0,
        scale_bits: 0x3F80_0000,
        one_bits: 0x3F80_0000,
        swap_enabled: 0,
        swap_index: 0,
        table: vec![0; 12],
        published_selector: 0,
        published_count: 0,
        session: 0x1000,
        stamp_counter: 0,
        stamp_tag_first: 0xA1,
        stamp_tag_final: 0xA2,
        trailer_first: 0xA3,
        trailer_second: 0xA4,
    }
}

fn phase(target: u32, params: u32) -> MirrorReflectionPhase {
    MirrorReflectionPhase {
        target,
        params,
        seeds: [0, 0, 0],
        status: 0,
    }
}

#[test]
fn deleting_destructor_frees_only_when_bit_zero_is_set() {
    for (flags, frees) in [(0, false), (1, true), (2, false), (3, true)] {
        let mut host = Fake::default();
        MirrorReflectionPhase::deleting_destructor(&mut host, flags);
        assert_eq!(
            host.names().contains(&"free_storage"),
            frees,
            "flags {flags}"
        );
        assert_eq!(
            host.names()[0],
            "destruct",
            "the destructor always runs first"
        );
    }
}

#[test]
fn render_does_nothing_behind_either_gate_or_without_a_target() {
    for (enabled, skipped, target) in [(0, 0, 5), (1, 1, 5), (1, 0, 0xFFFF_FFFF)] {
        let mut host = Fake::default();
        let mut g = open_globals();
        g.render_enabled = enabled;
        g.render_skipped = skipped;
        phase(target, 0x2000).render(&mut host, &mut g);
        assert!(
            host.calls.is_empty(),
            "gates {enabled}/{skipped}, target {target:#x}"
        );
    }
}

#[test]
fn render_selector_is_one_when_either_flag_is_set() {
    for (a, b, selector) in [(0, 0, 2), (1, 0, 1), (0, 1, 1), (1, 1, 1)] {
        let mut host = seeded(7, 0);
        let mut g = open_globals();
        g.select_flag_a = a;
        g.select_flag_b = b;
        phase(7, 0).render(&mut host, &mut g);
        let (_, args) = host
            .calls
            .iter()
            .find(|(n, _)| *n == "select")
            .expect("select is always called");
        assert_eq!(args[1], selector, "flags {a}/{b}");
    }
}

#[test]
fn render_stamps_two_blocks_and_advances_the_counter_twice() {
    let mut host = seeded(7, 0);
    host.answer("query", &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]);
    let mut g = open_globals();
    g.stamp_counter = 41;
    phase(7, 0).render(&mut host, &mut g);
    assert_eq!(g.stamp_counter, 43);
}

#[test]
fn render_with_a_null_query_block_stamps_nothing() {
    let mut host = seeded(7, 0);
    let mut g = open_globals();
    g.stamp_counter = 9;
    phase(7, 0).render(&mut host, &mut g);
    assert_eq!(g.stamp_counter, 9);
    assert!(!host.names().contains(&"slot30"), "no player and no stamp");
}

#[test]
fn refresh_with_a_zero_selector_or_count_returns_zero_without_calls() {
    for (selector, count) in [(0, 5), (3, 0), (3, 0x8000_0000)] {
        let mut host = Fake::default();
        host.words.insert((2, 0x3000, 0x30), selector);
        host.words.insert((2, 0x3000, 0x38), count);
        let mut g = open_globals();
        let ret = phase(0, 0x3000).refresh(&mut host, &g);
        g.swap_enabled = 0;
        assert_eq!(ret, 0);
        assert!(
            host.calls.is_empty(),
            "selector {selector}, count {count:#x}"
        );
    }
}

#[test]
#[should_panic(expected = "null parameter block")]
fn refresh_panics_on_a_null_parameter_block_as_the_original_faults() {
    let mut host = Fake::default();
    let g = open_globals();
    let _ = phase(0, 0).refresh(&mut host, &g);
}

#[test]
fn refresh_swap_saves_and_restores_the_block_around_the_commit() {
    let mut host = seeded(0, 0x3000);
    host.words.insert((2, 0x3000, 0x30), 0x11);
    host.words.insert((2, 0x3000, 0x38), 0x22);
    let mut g = open_globals();
    g.swap_enabled = 1;
    g.swap_index = 1;
    g.table = vec![0, 0, 0, 0x51, 0x52, 0x53];
    let mut p = phase(0, 0x3000);
    let _ = p.refresh(&mut host, &g);
    // The count and selector of entry one are written, the commit runs, then
    // the saved count and selector come back.
    let writes: Vec<(u32, u32)> = host
        .calls
        .iter()
        .filter(|(n, _)| *n == "set_word")
        .map(|(_, a)| (a[0], a[1]))
        .collect();
    assert_eq!(
        writes,
        vec![(0x38, 0x51), (0x30, 0x52), (0x38, 0x22), (0x30, 0x11)]
    );
}

#[test]
#[should_panic(expected = "outside the table")]
fn refresh_swap_entry_outside_the_table_panics() {
    let mut host = seeded(0, 0x3000);
    host.words.insert((2, 0x3000, 0x30), 1);
    host.words.insert((2, 0x3000, 0x38), 1);
    let mut g = open_globals();
    g.swap_enabled = 1;
    g.swap_index = 4;
    g.table = vec![0; 12];
    let _ = phase(0, 0x3000).refresh(&mut host, &g);
}

#[test]
fn refresh_remembers_the_flag_bit_of_the_object_for_the_second_hook() {
    let mut host = seeded(0, 0x3000);
    host.words.insert((2, 0x3000, 0x30), 1);
    host.words.insert((2, 0x3000, 0x38), 1);
    host.answer("lookup_object", &[0x4000]);
    host.words.insert((2, 0x4000, 0x24), 0x20);
    let g = open_globals();
    let _ = phase(0, 0x3000).refresh(&mut host, &g);
    let hooks: Vec<u32> = host
        .calls
        .iter()
        .filter(|(n, _)| *n == "slot30")
        .map(|(_, a)| a[0])
        .collect();
    assert_eq!(
        hooks,
        vec![1, 0xF000_0001],
        "first hook gets one, second the phase word with bit five"
    );
}

#[test]
fn reflect_with_a_null_block_takes_the_fallback_and_copies_status_bit_two() {
    for (status, bit) in [(0b100, 1), (0b011, 0), (0xFFFF_FFFB, 0), (0xFFFF_FFFF, 1)] {
        let mut host = Fake::default();
        host.words.insert((1, 0, 0x558), status);
        let mut g = open_globals();
        let mut p = phase(0, 0);
        let got = p.reflect(&mut host, &mut g);
        assert_eq!(got, bit, "status {status:#x}");
        assert_eq!(p.status, bit);
        assert!(host.names().contains(&"fallback"));
        assert!(!host.names().contains(&"reduce"));
    }
}

#[test]
fn reflect_publishes_the_block_then_latches_it_when_usable() {
    let mut host = seeded(0, 0x3000);
    host.words.insert((2, 0x3000, 0x30), 0x77);
    host.words.insert((2, 0x3000, 0x38), 4);
    host.words.insert((1, 0, 0x538), 0x99);
    let mut g = open_globals();
    let mut p = phase(0, 0x3000);
    let _ = p.reflect(&mut host, &mut g);
    assert_eq!(g.published_selector, 0x77);
    assert_eq!(g.published_count, 4);
    assert_eq!(host.words[&(2, 0x3000, 0x7C)], 4, "latched count");
    assert_eq!(host.words[&(2, 0x3000, 0x78)], 0x77, "latched selector");
    assert!(host.names().contains(&"combine"));
    assert!(!host.names().contains(&"fallback"));
}

#[test]
fn reflect_scales_the_fourth_word_only_when_the_flag_is_set() {
    let mut host = Fake::default();
    let mut g = open_globals();
    g.scale_enabled = 0;
    let _ = phase(0, 0).reflect(&mut host, &mut g);
    assert!(!host.names().contains(&"scale_project"));
    let mut host = Fake::default();
    g.scale_enabled = 1;
    let _ = phase(0, 0).reflect(&mut host, &mut g);
    assert!(host.names().contains(&"scale_project"));
}

#[test]
fn load_then_store_leaves_every_other_word_of_the_object_alone() {
    let mut object: Vec<u32> = (0..0x400u32)
        .map(|k| k.wrapping_mul(0x0101_0101) ^ 0xA5A5_5A5A)
        .collect();
    let before = object.clone();
    let p = MirrorReflectionPhase::load(&object);
    p.store(&mut object);
    assert_eq!(object, before);
}

#[test]
fn load_reads_the_status_byte_at_its_offset() {
    let mut object = vec![0u32; 0x400];
    object[0x1C / 4] = 0x0000_0500;
    let p = MirrorReflectionPhase::load(&object);
    assert_eq!(p.status, 0x05);
}

#[test]
fn registry_pins_four_proven_methods_in_slot_order() {
    assert_eq!(registry::proven_count(), 4);
    let slots: Vec<u32> = registry::METHODS.iter().map(|m| m.slot).collect();
    assert_eq!(slots, vec![0, 3, 4, 8]);
    assert!(
        registry::METHODS
            .iter()
            .all(|m| m.state == LiftState::Proven)
    );
}
