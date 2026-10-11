//! Host tests of the task predicates and the task state block: the edge
//! cases a reader would ask about, on the 64-bit host. The 32-bit
//! differential cases live in the `lf-taskshapediff` test crate.

use lf_peds_tasks::ped_task::registry::State;
use lf_peds_tasks::ped_task::shape_registry;
use lf_peds_tasks::ped_task::{
    Anchor, BlockCallees, ENTRY_CONSTANTS, FetchCallee, RecordTable, ScaleFactors, ScratchSlot,
    TableRecord, TaskBlock, TurnParams, TurnTask, TurnTrig, Vec3,
};
use lf_peds_tasks::ped_task::{
    BLOCK_WORDS, INVALID_HANDLE, PredicateTask, ResourceTag, STATE_RECORD_COUNT,
    STATE_RECORD_WORDS, StateRecord, TaskStateBlock,
};

/// A task that passes the enable and count test and nothing else.
fn enabled_task(count: i32) -> PredicateTask {
    PredicateTask {
        enabled: 1,
        count,
        ..PredicateTask::default()
    }
}

/// A task bound to a resource with the given tag word, of the given kind,
/// with the flag bit clear.
fn bound_task(kind: u8, tag: u32) -> PredicateTask {
    PredicateTask {
        kind,
        resource: Some(ResourceTag { tag }),
        ..PredicateTask::default()
    }
}

#[test]
fn enabled_needs_a_nonzero_enable_word() {
    let mut task = enabled_task(5);
    assert!(task.enabled_positive_count());
    task.enabled = 0;
    assert!(!task.enabled_positive_count());
    // Any nonzero enable word counts, not only one.
    task.enabled = 0x8000_0000;
    assert!(task.enabled_positive_count());
}

#[test]
fn count_must_be_strictly_positive_and_signed() {
    assert!(!enabled_task(0).enabled_positive_count());
    assert!(enabled_task(1).enabled_positive_count());
    assert!(!enabled_task(-1).enabled_positive_count());
    // The most negative count is a large unsigned value but not positive.
    assert!(!enabled_task(i32::MIN).enabled_positive_count());
    assert!(enabled_task(i32::MAX).enabled_positive_count());
}

#[test]
fn flag_bit_passes_without_a_resource() {
    let flagged = PredicateTask {
        flags: 0x04,
        kind: 0,
        resource: None,
        ..PredicateTask::default()
    };
    assert!(flagged.resource_tag());
    // Other flag bits alone do not pass.
    let other = PredicateTask {
        flags: 0xFB,
        ..flagged
    };
    assert!(!other.resource_tag());
}

#[test]
fn kind_low_nibble_decides_the_threshold() {
    // Kind one is below the minimum; two is the first that can pass.
    assert!(!bound_task(1, 0x80).resource_tag());
    assert!(bound_task(2, 0x80).resource_tag());
    // Only the low nibble counts: a high nibble alone does not lift the kind.
    assert!(!bound_task(0xF1, 0x80).resource_tag());
    assert!(bound_task(0x12, 0x80).resource_tag());
}

#[test]
fn unbound_task_fails_unless_flagged() {
    assert!(bound_task(2, 0x80).resource_tag(), "the bound task passes");
    let unbound = PredicateTask {
        kind: 2,
        resource: None,
        ..PredicateTask::default()
    };
    assert!(!unbound.resource_tag());
}

#[test]
fn tag_bits_under_the_mask_must_equal_the_expected_value() {
    assert!(bound_task(2, 0x80).resource_tag());
    // Bits outside the mask are ignored.
    assert!(bound_task(2, 0xFFFF_FC3F | 0x80).resource_tag());
    // A set bit inside the mask fails.
    assert!(!bound_task(2, 0x80 | 0x40).resource_tag());
    assert!(!bound_task(2, 0x80 | 0x100).resource_tag());
    // Bit 4 of the word is outside the mask: the mask is 0x3c0, not 0x3f0.
    assert!(bound_task(2, 0x80 | 0x10).resource_tag());
}

#[test]
fn predicates_do_not_change_the_task() {
    let task = bound_task(3, 0x80);
    let before = task;
    let _ = task.enabled_positive_count();
    let _ = task.resource_tag();
    assert_eq!(task, before);
}

#[test]
fn reset_marks_only_the_handle_words_invalid_on_a_zero_block() {
    let mut block = TaskStateBlock::default();
    block.reset();
    let words = block.to_words();
    assert_eq!(words.len(), BLOCK_WORDS);
    assert_eq!(words[0], 0, "the header is cleared");
    for k in 0..STATE_RECORD_COUNT {
        let base = 1 + k * STATE_RECORD_WORDS;
        assert_eq!(words[base], INVALID_HANDLE, "record {k} first handle");
        assert_eq!(words[base + 1], INVALID_HANDLE, "record {k} second handle");
        for offset in 2..STATE_RECORD_WORDS {
            assert_eq!(words[base + offset], 0, "record {k} payload word {offset}");
        }
    }
    assert_eq!(words[BLOCK_WORDS - 1], 0, "the last word is a payload word");
}

#[test]
fn reset_clears_everything_it_was_given() {
    let mut block = TaskStateBlock {
        header: 0x1234,
        records: [StateRecord {
            handle_a: 7,
            handle_b: 8,
            words: [9, 10, 11, 12, 13, 14],
        }; STATE_RECORD_COUNT],
    };
    block.reset();
    let once = block.clone();
    block.reset();
    assert_eq!(block, once, "a second reset changes nothing");
    assert_eq!(block.header, 0);
    for record in &block.records {
        assert_eq!(record.handle_a, INVALID_HANDLE);
        assert_eq!(record.handle_b, INVALID_HANDLE);
        assert_eq!(record.words, [0; STATE_RECORD_WORDS - 2]);
    }
}

#[test]
fn registry_section_counts_match_the_lifted_routines() {
    let (proven, lifted, missing) = shape_registry::counts();
    assert_eq!(proven, 6);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 0);
    for row in shape_registry::ROWS {
        assert_eq!(row.state, State::Proven);
        assert!(
            !row.narrows.is_empty(),
            "{} {} states its narrowings",
            row.class,
            row.method
        );
    }
}

/// A collaborator that records the block initialiser's calls and answers a
/// fixed draw and tick.
#[derive(Default)]
struct Calls {
    draw: u32,
    tick: u32,
    log: Vec<String>,
}

impl BlockCallees for Calls {
    fn construct_first(&mut self, block: u32) {
        self.log.push(format!("construct_first {block:#x}"));
    }
    fn construct_middle(&mut self, sub: u32) {
        self.log.push(format!("construct_middle {sub:#x}"));
    }
    fn construct_tail(&mut self, sub: u32) {
        self.log.push(format!("construct_tail {sub:#x}"));
    }
    fn setup_first(&mut self, block: u32) {
        self.log.push(format!("setup_first {block:#x}"));
    }
    fn draw(&mut self) -> u32 {
        self.log.push("draw".to_owned());
        self.draw
    }
    fn setup_second(&mut self, block: u32, arg: u32) {
        self.log.push(format!("setup_second {block:#x} {arg}"));
    }
    fn setup_third(&mut self, block: u32, arg: u32) {
        self.log.push(format!("setup_third {block:#x} {arg}"));
    }
    fn tick(&self) -> u32 {
        self.tick
    }
}

/// The image's factors: the reciprocal of 2^15 and the negative of 500.
const IMAGE_SCALE: ScaleFactors = ScaleFactors {
    inverse_range: 1.0 / 32768.0,
    factor: -500.0,
};

#[test]
fn block_init_runs_its_calls_in_order_with_the_sub_object_handles() {
    let mut calls = Calls {
        draw: 0,
        tick: 77,
        log: Vec::new(),
    };
    let _ = TaskBlock::initialise(0x1000, &mut calls, IMAGE_SCALE);
    assert_eq!(
        calls.log,
        vec![
            "construct_first 0x1000",
            "construct_middle 0x1080",
            "construct_tail 0x1110",
            "setup_first 0x1000",
            "draw",
            "setup_second 0x1000 0",
            "setup_third 0x1000 0",
        ]
    );
}

#[test]
fn block_init_sets_the_owned_words() {
    let mut calls = Calls {
        draw: 0,
        tick: 77,
        log: Vec::new(),
    };
    let block = TaskBlock::initialise(0x1000, &mut calls, IMAGE_SCALE);
    assert_eq!(block.params, [0; 6]);
    assert_eq!(block.unit_scale, 1.0);
    assert_eq!(block.unit_scale_tail, 0);
    assert_eq!(block.tick_copy, 77);
    assert_eq!(block.ready, 1);
    assert_eq!(block.result, 0);
    assert_eq!(block.ctrl_a, 0);
    assert_eq!(block.selector, u32::MAX, "no selection");
    assert_eq!(block.ctrl_b, 0);
    assert_eq!(block.flag_b, 0);
    assert_eq!(block.tail, 0);
}

#[test]
fn block_init_countdown_spans_five_hundred_to_fifteen_hundred() {
    let countdown = |draw: u32| {
        let mut calls = Calls {
            draw,
            tick: 0,
            log: Vec::new(),
        };
        TaskBlock::initialise(0x1000, &mut calls, IMAGE_SCALE).countdown
    };
    // No draw: the countdown is the base.
    assert_eq!(countdown(0), 500);
    // The largest sixteen-bit draw takes the countdown to its top.
    assert_eq!(countdown(0xFFFF), 1499);
    // Bits above the sixteen used are ignored.
    assert_eq!(countdown(0x0001_0000), 500);
    assert_eq!(countdown(0xFFFF_0000), 500);
}

#[test]
fn block_init_truncates_the_scaled_draw_toward_zero() {
    // Half the range: 32768 times 2^-15 times -500 is -500, exactly.
    let mut calls = Calls {
        draw: 0x8000,
        tick: 0,
        log: Vec::new(),
    };
    assert_eq!(
        TaskBlock::initialise(0x1000, &mut calls, IMAGE_SCALE).countdown,
        1000
    );
    // A product of minus 0.5 truncates to zero, not minus one.
    let half = ScaleFactors {
        inverse_range: 1.0,
        factor: -0.5,
    };
    let mut calls = Calls {
        draw: 1,
        tick: 0,
        log: Vec::new(),
    };
    assert_eq!(
        TaskBlock::initialise(0x1000, &mut calls, half).countdown,
        500
    );
}

#[test]
fn block_init_not_a_number_and_overflow_follow_the_verified_rewrite() {
    // The verified rewrite converts with the Rust conversion: a not-a-number
    // gives zero, and an overflow saturates.
    let nan = ScaleFactors {
        inverse_range: f32::NAN,
        factor: 1.0,
    };
    let mut calls = Calls {
        draw: 1,
        tick: 0,
        log: Vec::new(),
    };
    assert_eq!(
        TaskBlock::initialise(0x1000, &mut calls, nan).countdown,
        500
    );
    let overflow = ScaleFactors {
        inverse_range: 1.0,
        factor: f32::INFINITY,
    };
    let mut calls = Calls {
        draw: 1,
        tick: 0,
        log: Vec::new(),
    };
    assert_eq!(
        TaskBlock::initialise(0x1000, &mut calls, overflow).countdown,
        500u32.wrapping_sub(i32::MAX as u32)
    );
}
/// A trigonometric stand-in that counts its calls.
#[derive(Default)]
struct Trig {
    calls: Vec<&'static str>,
}

impl TurnTrig for Trig {
    fn sine(&mut self, _angle: f32) -> f32 {
        self.calls.push("sine");
        0.5
    }
    fn cosine(&mut self, _angle: f32) -> f32 {
        self.calls.push("cosine");
        2.0
    }
}

const NO_INNER: TurnParams = TurnParams {
    angle: 0.0,
    inner: None,
};

#[test]
fn turn_gate_is_strict_at_the_threshold() {
    let mut task = TurnTask { rate: 0.5, flag: 0 };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut trig = Trig::default();
    task.advance(
        &mut target,
        &NO_INNER,
        lf_peds_tasks::ped_task::RATE_GATE,
        &mut trig,
    );
    assert_eq!(
        task.rate, 0.5,
        "a gate equal to the threshold does not raise the rate"
    );
    task.advance(
        &mut target,
        &NO_INNER,
        f32::from_bits(lf_peds_tasks::ped_task::RATE_GATE.to_bits() + 1),
        &mut trig,
    );
    assert!(task.rate > 0.5, "one step above the threshold raises it");
}

#[test]
fn turn_reset_flag_sets_the_rate_to_one_before_the_gate() {
    let mut task = TurnTask {
        rate: 0.25,
        flag: 0x80,
    };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut trig = Trig::default();
    task.advance(&mut target, &NO_INNER, 0.0, &mut trig);
    assert_eq!(task.rate, 1.0);
    assert_eq!(task.flag, 0x80, "the flag byte is not cleared");
}

#[test]
fn turn_rate_is_capped_at_one_and_not_a_number_becomes_one() {
    let mut task = TurnTask {
        rate: 0.995,
        flag: 0,
    };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut trig = Trig::default();
    task.advance(&mut target, &NO_INNER, 1.0, &mut trig);
    assert_eq!(task.rate, 1.0, "a sum above one is capped");
    let mut task = TurnTask {
        rate: f32::NAN,
        flag: 0,
    };
    task.advance(&mut target, &NO_INNER, 1.0, &mut trig);
    assert_eq!(task.rate, 1.0, "a not-a-number rate becomes one");
}

#[test]
fn turn_inner_direction_skips_the_trigonometry() {
    let mut task = TurnTask { rate: 1.0, flag: 0 };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut trig = Trig::default();
    let params = TurnParams {
        angle: 1.0,
        inner: Some(Vec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        }),
    };
    task.advance(&mut target, &params, 0.0, &mut trig);
    assert!(
        trig.calls.is_empty(),
        "no sine or cosine when the inner direction is present"
    );
    // Each component is scaled by a tenth and by the rate, then subtracted.
    assert!((target.x + 1.0).abs() < 1e-6);
    assert!((target.y + 2.0).abs() < 1e-6);
    assert!((target.z + 3.0).abs() < 1e-6);
}

#[test]
fn turn_without_inner_uses_negated_sine_cosine_and_zero() {
    let mut task = TurnTask { rate: 1.0, flag: 0 };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut trig = Trig::default();
    task.advance(&mut target, &NO_INNER, 0.0, &mut trig);
    assert_eq!(
        trig.calls,
        vec!["sine", "cosine"],
        "sine first, then cosine"
    );
    // The x component is minus the sine (0.5) scaled, the y is the cosine (2.0),
    // and the z component is zero.
    assert!((target.x - 0.05).abs() < 1e-6);
    assert!((target.y + 0.2).abs() < 1e-6);
    assert_eq!(target.z, 0.0);
}
/// A fetch collaborator with a fixed anchor and fixed points per record;
/// it logs its calls.
struct Scan {
    anchor: Anchor,
    points: Vec<[f32; 4]>,
    calls: Vec<(usize, ScratchSlot)>,
}

impl FetchCallee for Scan {
    fn anchor(&self) -> Anchor {
        self.anchor
    }
    fn fetch(&mut self, record: usize, slot: ScratchSlot) -> [f32; 4] {
        self.calls.push((record, slot));
        self.points[record]
    }
}

/// A table whose records carry the given tags, the rest zero.
fn table_with(tags: &[u32]) -> RecordTable {
    let mut table = RecordTable::default();
    for (k, tag) in tags.iter().enumerate() {
        table.records[k] = TableRecord { tag: *tag };
    }
    table
}

const ORIGIN: Anchor = Anchor {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

#[test]
fn scan_fetches_only_the_three_scanned_tags_in_order() {
    let tags = [0x11, 0x05, 0x12, 0x1B];
    let table = table_with(&tags);
    let mut scan = Scan {
        anchor: ORIGIN,
        points: vec![[0.0; 4]; TABLE_SLOTS_FOR_TEST],
        calls: Vec::new(),
    };
    let _ = table.collect_near(10.0, 100, &mut scan);
    assert_eq!(
        scan.calls,
        vec![
            (0, ScratchSlot::First),
            (2, ScratchSlot::Second),
            (3, ScratchSlot::Third),
        ]
    );
}

const TABLE_SLOTS_FOR_TEST: usize = lf_peds_tasks::ped_task::TABLE_SLOTS;

#[test]
fn scan_keeps_points_strictly_inside_the_radius() {
    let table = table_with(&[0x11, 0x11]);
    let mut points = vec![[0.0_f32; 4]; TABLE_SLOTS_FOR_TEST];
    points[0] = [3.0, 4.0, 0.0, 9.0];
    // Exactly on the radius: squared distance 25 is not below the limit 25.
    points[1] = [5.0, 0.0, 0.0, 1.0];
    let mut scan = Scan {
        anchor: ORIGIN,
        points,
        calls: Vec::new(),
    };
    let kept = table.collect_near(5.0, 100, &mut scan);
    assert_eq!(
        kept.len(),
        0,
        "distance 5 is not strictly inside a radius of 5"
    );
    let mut points = vec![[0.0_f32; 4]; TABLE_SLOTS_FOR_TEST];
    points[0] = [3.0, 4.0, 0.0, 9.0];
    let mut scan = Scan {
        anchor: ORIGIN,
        points,
        calls: Vec::new(),
    };
    let kept = table_with(&[0x11]).collect_near(5.001, 100, &mut scan);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].points, [3.0, 4.0, 0.0, 9.0]);
    assert_eq!(kept[0].constants, ENTRY_CONSTANTS);
}

#[test]
fn scan_stops_once_the_count_reaches_the_limit() {
    let table = table_with(&[0x11, 0x11, 0x11]);
    let mut scan = Scan {
        anchor: ORIGIN,
        points: vec![[0.0; 4]; TABLE_SLOTS_FOR_TEST],
        calls: Vec::new(),
    };
    let kept = table.collect_near(1.0, 2, &mut scan);
    assert_eq!(kept.len(), 2);
    assert_eq!(scan.calls.len(), 2, "no fetch after the limit is reached");
    // A limit of zero or less keeps one entry: the check is after each keep.
    let mut scan = Scan {
        anchor: ORIGIN,
        points: vec![[0.0; 4]; TABLE_SLOTS_FOR_TEST],
        calls: Vec::new(),
    };
    assert_eq!(table.collect_near(1.0, 0, &mut scan).len(), 1);
}

#[test]
fn scan_drops_a_point_with_a_not_a_number_component() {
    let table = table_with(&[0x12]);
    let mut points = vec![[0.0_f32; 4]; TABLE_SLOTS_FOR_TEST];
    points[0] = [f32::NAN, 0.0, 0.0, 0.0];
    let mut scan = Scan {
        anchor: ORIGIN,
        points,
        calls: Vec::new(),
    };
    assert!(table.collect_near(1.0e6, 10, &mut scan).is_empty());
}

#[test]
fn scan_with_an_empty_table_fetches_nothing() {
    let table = RecordTable::default();
    let mut scan = Scan {
        anchor: ORIGIN,
        points: vec![[0.0; 4]; TABLE_SLOTS_FOR_TEST],
        calls: Vec::new(),
    };
    assert!(table.collect_near(1.0, 10, &mut scan).is_empty());
    assert!(scan.calls.is_empty());
}
