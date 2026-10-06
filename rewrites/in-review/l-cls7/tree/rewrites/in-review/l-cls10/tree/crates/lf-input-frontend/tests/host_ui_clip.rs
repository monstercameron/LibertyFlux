//! Host tests for the lifted UI clip: edge cases a reader would ask
//! about, run on the 64-bit host against the lifted code only.

use lf_core::Handle32;
use lf_input_frontend::ui_clip::{
    BasicClip, ClipWorld, ElementTag, EntryTag, EntryTableTag, MatchOut, PartTag, SinkTag,
    SubmitTag, TransformRecord, TripleKind, quiet_snan, registry, triple_bytes, truncate_raw,
    up_to_nul,
};
use std::collections::{HashMap, VecDeque};

fn cookie<T>(v: u32) -> Handle32<T> {
    Handle32::new(v).expect("nonzero test cookie")
}

/// A scripted world for host tests.
#[derive(Default)]
struct World {
    words: HashMap<&'static str, VecDeque<u32>>,
    texts: VecDeque<Option<Vec<u8>>>,
    titles: VecDeque<Vec<u8>>,
    matches: VecDeque<MatchOut>,
    xforms: VecDeque<[u8; 24]>,
    directs: VecDeque<Handle32<EntryTag>>,
    tables: HashMap<u32, Vec<TransformRecord>>,
    direct_recs: HashMap<u32, TransformRecord>,
    submit_word: u32,
    calls: Vec<String>,
    forwards: Vec<u32>,
    pushes: Vec<u32>,
}

impl World {
    fn pop(&mut self, name: &'static str) -> u32 {
        self.calls.push(name.to_string());
        self.words.get_mut(name).and_then(VecDeque::pop_front).unwrap_or(0)
    }

    fn with(mut self, name: &'static str, values: &[u32]) -> Self {
        self.words.insert(name, values.iter().copied().collect());
        self
    }
}

impl ClipWorld for World {
    fn submit_word(&mut self, _s: Handle32<SubmitTag>) -> u32 {
        self.calls.push("submit_word".to_string());
        self.submit_word
    }
    fn set_submit_word(&mut self, _s: Handle32<SubmitTag>, value: u32) {
        self.calls.push("set_submit_word".to_string());
        self.submit_word = value;
    }
    fn probe(&mut self) -> u32 {
        self.pop("probe")
    }
    fn run_action(&mut self) -> u32 {
        self.pop("run_action")
    }
    fn part_predicate(&mut self, _p: Handle32<PartTag>) -> bool {
        self.pop("part_predicate") & 0xff != 0
    }
    fn forward_to_part(&mut self, _p: Handle32<PartTag>, a: u32) -> u32 {
        self.forwards.push(a);
        self.pop("forward_to_part")
    }
    fn part_count(&mut self) -> u32 {
        self.pop("part_count")
    }
    fn set_element_flag(&mut self, _e: Handle32<ElementTag>, _f: u8) {
        self.calls.push("set_element_flag".to_string());
    }
    fn measure(&mut self) -> f32 {
        f32::from_bits(self.pop("measure"))
    }
    fn push_adjusted(&mut self, _s: Handle32<SubmitTag>, b: u32) -> u32 {
        self.pushes.push(b);
        self.pop("push_adjusted")
    }
    fn encode_triple(&mut self, _k: TripleKind, _b: [u8; 3]) {
        self.calls.push("encode_triple".to_string());
    }
    fn run_triple_mid(&mut self, _k: TripleKind) {
        self.calls.push("run_triple_mid".to_string());
    }
    fn finish_triple(&mut self) {
        self.calls.push("finish_triple".to_string());
    }
    fn frame_check(&mut self) -> u32 {
        self.pop("frame_check")
    }
    fn set_part_text(&mut self, _p: Handle32<PartTag>, _b: &[u8]) -> u32 {
        self.pop("set_part_text")
    }
    fn part_text(&mut self, _p: Handle32<PartTag>) -> Option<Vec<u8>> {
        self.calls.push("part_text".to_string());
        self.texts.pop_front().unwrap_or(None)
    }
    fn sink_title_present(&mut self, _s: Handle32<SinkTag>) -> bool {
        self.pop("sink_title_present") != 0
    }
    fn source_title(&mut self, _p: Handle32<PartTag>) -> Vec<u8> {
        self.calls.push("source_title".to_string());
        self.titles.pop_front().unwrap_or(vec![0])
    }
    fn submit_to_sink(&mut self, _s: Handle32<SinkTag>, _b: &[u8]) {
        self.calls.push("submit_to_sink".to_string());
    }
    fn fetch_handle(&mut self, _p: Handle32<PartTag>) -> u32 {
        self.pop("fetch_handle")
    }
    fn match_entries(&mut self, _h: u32) -> MatchOut {
        self.calls.push("match_entries".to_string());
        self.matches.pop_front().expect("match answer")
    }
    fn transform_bytes(&mut self, _a: u32, _b: u32) -> [u8; 24] {
        self.calls.push("transform_bytes".to_string());
        self.xforms.pop_front().expect("transform answer")
    }
    fn release_service(&mut self) {
        self.calls.push("release_service".to_string());
    }
    fn teardown_entries(&mut self, _t: Handle32<EntryTableTag>) {
        self.calls.push("teardown_entries".to_string());
    }
    fn entry_record(&mut self, t: Handle32<EntryTableTag>, i: u32) -> &mut TransformRecord {
        self.calls.push("entry_record".to_string());
        self.tables.get_mut(&t.get()).expect("table").get_mut(i as usize).expect("index")
    }
    fn direct_entry(&mut self, _h: u32) -> Handle32<EntryTag> {
        self.calls.push("direct_entry".to_string());
        self.directs.pop_front().expect("direct cookie")
    }
    fn direct_record(&mut self, e: Handle32<EntryTag>) -> &mut TransformRecord {
        self.calls.push("direct_record".to_string());
        self.direct_recs.get_mut(&e.get()).expect("direct record")
    }
}

fn clip() -> BasicClip {
    BasicClip::new(
        0xAA,
        7,
        f32::from_bits(0x4040_0000),
        Some(cookie(1)),
        Some(cookie(2)),
        Some(cookie(3)),
        Some(cookie(4)),
        vec![cookie(10), cookie(11), cookie(12)],
    )
}

#[test]
fn registry_counts() {
    assert_eq!(registry::ROWS.len(), 31);
    assert_eq!(registry::counts(), (14, 0, 17));
}

#[test]
fn truncate_edges() {
    assert_eq!(truncate_raw(1.9), 1);
    assert_eq!(truncate_raw(-1.9), -1);
    assert_eq!(truncate_raw(f32::NAN), i32::MIN);
    assert_eq!(truncate_raw(f32::INFINITY), i32::MIN);
    assert_eq!(truncate_raw(f32::NEG_INFINITY), i32::MIN);
    assert_eq!(truncate_raw(2147483648.0), i32::MIN);
    assert_eq!(truncate_raw(-2147483648.0), i32::MIN);
    assert_eq!(truncate_raw(2147483520.0), 2147483520);
}

#[test]
fn quiet_edges() {
    // Signalling NaNs gain the quiet bit, nothing else changes.
    assert_eq!(quiet_snan(f32::from_bits(0x7F80_0001)).to_bits(), 0x7FC0_0001);
    assert_eq!(quiet_snan(f32::from_bits(0xFF80_00FF)).to_bits(), 0xFFC0_00FF);
    for bits in [0u32, 0x8000_0000, 0x3F80_0000, 0x7F80_0000, 0x7FC0_0000, 0x007F_FFFF] {
        assert_eq!(quiet_snan(f32::from_bits(bits)).to_bits(), bits);
    }
}

#[test]
fn triple_known_values() {
    assert_eq!(triple_bytes(0.0), [0, 0, 0]);
    // Sixty times the first factor is one: the chain nets out exactly.
    assert_eq!(triple_bytes(60.0), [1, 0, 0]);
    // NaN input: every conversion sees NaN or a NaN-derived value.
    let [b0, b1, b2] = triple_bytes(f32::NAN);
    assert_eq!((b0, b1, b2), (0, 0, 0));
}

#[test]
fn nul_ok() {
    assert_eq!(up_to_nul(&[65, 66, 0, 67]), &[65, 66]);
    assert_eq!(up_to_nul(&[0]), &[] as &[u8]);
}

#[test]
#[should_panic(expected = "NUL-terminated")]
fn nul_missing_panics() {
    let _ = up_to_nul(&[65, 66, 67]);
}

#[test]
fn accessors() {
    let c = clip();
    assert_eq!((c.flag(), c.mode()), (0xAA, 7));
    assert_eq!(c.stored().to_bits(), 0x4040_0000);
    assert_eq!(c.sink().map(|h| h.get()), Some(4));
    assert_eq!(c.parts().len(), 3);
}

#[test]
fn submit_word_round_trip() {
    let c = clip();
    let mut w = World { submit_word: 0x1234, ..World::default() };
    let mut out = 0;
    c.submit_word_into(&mut w, &mut out);
    assert_eq!(out, 0x1234);
    assert_eq!(c.store_submit_word(&mut w, 0x77), 0x77);
    assert_eq!(w.submit_word, 0x77);
    assert_eq!(w.calls, ["submit_word", "set_submit_word"]);
}

#[test]
fn probe_paths() {
    let c = clip();
    // Low byte zero: no action, probe answer returned.
    let mut w = World::default().with("probe", &[0x100]).with("run_action", &[9]);
    assert_eq!(c.probe_and_act(&mut w), 0x100);
    assert_eq!(w.calls, ["probe"]);
    // Low byte set: action runs and answers.
    let mut w = World::default().with("probe", &[0x101]).with("run_action", &[9]);
    assert_eq!(c.probe_and_act(&mut w), 9);
    assert_eq!(w.calls, ["probe", "run_action"]);
}

#[test]
fn forward_matrix() {
    // (arg low byte, predicate, mode) -> forwarded argument.
    for (arg, pred, mode, want) in [
        (0u32, false, 0u8, 0u32),
        (2, true, 1, 0),
        (1, false, 0, 1),
        (1, true, 0, 0),
        (1, true, 1, 1),
        (1, true, 2, 1),
        (1, true, 3, 0),
        (1, false, 2, 1),
    ] {
        let c = BasicClip::new(0, mode, 0.0, None, Some(cookie(2)), Some(cookie(3)), None, vec![]);
        let mut w = World::default()
            .with("part_predicate", &[u32::from(pred)])
            .with("forward_to_part", &[0xE]);
        assert_eq!(c.forward_conditional(&mut w, 0xFF00 | arg), 0xE, "arg={arg} pred={pred} mode={mode}");
        assert_eq!(w.forwards, [want], "forwarded arg for arg={arg} pred={pred} mode={mode}");
    }
}

#[test]
fn flag_empty_and_growing() {
    let mut c = clip();
    // Empty loop answers zero and flags nothing.
    let mut w = World::default().with("part_count", &[0]);
    assert_eq!(c.set_flag(&mut w, 5), 0);
    assert_eq!((c.flag(), w.calls.len()), (5, 1));
    // Growing count runs longer: rounds at 0, 1 and 2, then stops.
    let mut w = World::default().with("part_count", &[1, 3, 3, 0]);
    assert_eq!(c.set_flag(&mut w, 6), 0);
    assert_eq!(w.calls.iter().filter(|n| n.as_str() == "set_element_flag").count(), 3);
}

#[test]
#[should_panic(expected = "past the element array")]
fn flag_overrun_panics() {
    let mut c = clip();
    let mut w = World::default().with("part_count", &[9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    c.set_flag(&mut w, 1);
}

#[test]
fn push_scales_and_shifts() {
    let c = clip();
    // Flag clear: shift only. Flag set: scale then shift.
    for (arg, measure, want) in [
        (0u32, 10.0f32, 10.0 - 6.0),
        (1u32, 10.0f32, 10.0 * f32::from_bits(0x3F70_A3D7) - 6.0),
    ] {
        let mut w = World::default()
            .with("forward_to_part", &[0])
            .with("measure", &[measure.to_bits()])
            .with("push_adjusted", &[0xBEEF]);
        assert_eq!(c.forward_and_push(&mut w, arg), 0xBEEF);
        assert_eq!(w.pushes, [want.to_bits()], "pushed bits for arg={arg}");
    }
    // NaN measure stays NaN through the shift (payload pinned by the
    // differential proof, not here).
    let mut w = World::default()
        .with("forward_to_part", &[0])
        .with("measure", &[0x7FC0_0001])
        .with("push_adjusted", &[1]);
    assert_eq!(c.forward_and_push(&mut w, 0), 1);
}

#[test]
fn triple_stores_and_answers() {
    let mut c = clip();
    for kind in [TripleKind::First, TripleKind::Second] {
        let mut w = World::default().with("frame_check", &[0xC]);
        assert_eq!(c.set_triple(&mut w, kind, 60.0), 0xC);
        assert_eq!(c.stored().to_bits(), 60.0f32.to_bits());
        assert_eq!(w.calls, ["encode_triple", "run_triple_mid", "finish_triple", "frame_check"]);
    }
}

#[test]
fn child_text_paths() {
    let c = clip();
    // Replace.
    let mut w = World::default().with("set_part_text", &[11]);
    assert_eq!(c.set_child_text(&mut w, b"hi\0", false), 11);
    // Append missing current text falls back to replacing (one getter call).
    let mut w = World::default().with("set_part_text", &[12]);
    w.texts.push_back(None);
    assert_eq!(c.set_child_text(&mut w, b"hi\0", true), 12);
    // Append that fits.
    let mut w = World::default().with("set_part_text", &[13]);
    w.texts.push_back(Some(b"ab\0".to_vec()));
    w.texts.push_back(Some(b"ab\0".to_vec()));
    assert_eq!(c.set_child_text(&mut w, b"cd\0", true), 13);
    // Append that does not fit answers the free space (256 - 255).
    let mut cur = vec![b'x'; 255];
    cur.push(0);
    let mut w = World::default();
    w.texts.push_back(Some(cur.clone()));
    w.texts.push_back(Some(cur));
    assert_eq!(c.set_child_text(&mut w, b"yz\0", true), 1);
}

#[test]
#[should_panic(expected = "NUL-terminated")]
fn child_text_unterminated_panics() {
    let c = clip();
    let mut w = World::default();
    c.set_child_text(&mut w, b"nope", false);
}

#[test]
fn label_paths() {
    let c = clip();
    // Untitled: straight to the sink, no title query.
    let mut w = World::default();
    c.submit_label(&mut w, b"l\0", false);
    assert_eq!(w.calls, ["submit_to_sink"]);
    // Titled but sink disagrees: still straight through (one query).
    let mut w = World::default().with("sink_title_present", &[0]);
    c.submit_label(&mut w, b"l\0", true);
    assert_eq!(w.calls, ["sink_title_present", "submit_to_sink"]);
    // Titled and agreed: title fetched, buffer submitted.
    let mut w = World::default().with("sink_title_present", &[1]);
    w.titles.push_back(b"t\0".to_vec());
    c.submit_label(&mut w, b"l\0", true);
    assert_eq!(w.calls, ["sink_title_present", "source_title", "submit_to_sink"]);
    // Over-long title plus label skips the append but still submits.
    let mut w = World::default().with("sink_title_present", &[1]);
    let mut big = vec![b't'; 255];
    big.push(0);
    w.titles.push_back(big);
    c.submit_label(&mut w, b"label\0", true);
    assert_eq!(w.calls, ["sink_title_present", "source_title", "submit_to_sink"]);
}

#[test]
fn mode_paths() {
    // Same mode: no calls at all.
    let mut c = clip();
    let mut w = World::default();
    c.set_display_mode(&mut w, 7);
    assert!(w.calls.is_empty());
    // Other mode: stores only.
    c.set_display_mode(&mut w, 9);
    assert_eq!(c.mode(), 9);
    assert!(w.calls.is_empty());
    // Mode zero: four refreshes, notify, no teardown on zero gate.
    let table: Handle32<EntryTableTag> = cookie(100);
    let mut w = World::default().with("fetch_handle", &[55]).with("forward_to_part", &[0]);
    w.matches.push_back(MatchOut { table, gate: 0 });
    w.xforms.extend([[1u8; 24], [2; 24], [3; 24], [4; 24]]);
    w.tables.insert(table.get(), vec![TransformRecord::default(); 4]);
    c.set_display_mode(&mut w, 0);
    assert_eq!(c.mode(), 0);
    assert!(w.calls.contains(&"forward_to_part".to_string()));
    assert!(!w.calls.contains(&"teardown_entries".to_string()));
    assert_eq!(w.tables[&table.get()][2].transform, [3u8; 24]);
    // Mode two with a gated teardown.
    let mut w = World::default().with("fetch_handle", &[56]);
    w.matches.push_back(MatchOut { table, gate: 0x1_0000 });
    w.xforms.extend([[0u8; 24]; 4]);
    w.tables.insert(table.get(), vec![TransformRecord::default(); 4]);
    c.set_display_mode(&mut w, 2);
    assert!(w.calls.contains(&"teardown_entries".to_string()));
    assert!(!w.calls.contains(&"forward_to_part".to_string()));
    // Mode one touches the direct entries too.
    let d1: Handle32<EntryTag> = cookie(200);
    let d2: Handle32<EntryTag> = cookie(201);
    let mut w = World::default().with("fetch_handle", &[57, 58, 59]);
    w.matches.push_back(MatchOut { table, gate: 0 });
    w.xforms.extend([[0u8; 24]; 6]);
    w.directs.extend([d1, d2]);
    w.tables.insert(table.get(), vec![TransformRecord::default(); 4]);
    w.direct_recs.insert(d1.get(), TransformRecord::default());
    w.direct_recs.insert(d2.get(), TransformRecord::default());
    c.set_display_mode(&mut w, 1);
    assert_eq!(w.calls.iter().filter(|n| n.as_str() == "transform_bytes").count(), 6);
    assert_eq!(w.calls.iter().filter(|n| n.as_str() == "direct_entry").count(), 2);
}

#[test]
#[should_panic(expected = "needs the submit part")]
fn missing_submit_panics() {
    let c = BasicClip::new(0, 0, 0.0, None, None, None, None, vec![]);
    let mut w = World::default();
    let mut out = 0;
    c.submit_word_into(&mut w, &mut out);
}

#[test]
#[should_panic(expected = "needs the second part")]
fn missing_part_panics() {
    let c = BasicClip::new(0, 0, 0.0, None, None, None, None, vec![]);
    let mut w = World::default().with("forward_to_part", &[0]);
    c.forward_conditional(&mut w, 1);
}
