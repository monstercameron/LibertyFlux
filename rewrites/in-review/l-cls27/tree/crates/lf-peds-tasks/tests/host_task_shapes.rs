//! Host tests of the task predicates and the task state block: the edge
//! cases a reader would ask about, on the 64-bit host. The 32-bit
//! differential cases live in the `lf-taskshapediff` test crate.

use lf_peds_tasks::ped_task::registry::State;
use lf_peds_tasks::ped_task::shape_registry;
use lf_peds_tasks::ped_task::{
    AimCallees, AimOwner, AimTask, Anchor, BlockCallees, ENTRY_CONSTANTS, FetchCallee, FlagCallees,
    FlagOwner, RecordTable, ScaleFactors, ScratchSlot, SubjectFlag, TableRecord, TaskBlock,
    TurnParams, TurnTask, TurnTrig, Vec3,
};
use lf_peds_tasks::ped_task::{
    BLOCK_WORDS, INVALID_HANDLE, PredicateTask, ResourceTag, STATE_RECORD_COUNT,
    STATE_RECORD_WORDS, StateRecord, TaskStateBlock,
};
use lf_peds_tasks::ped_task::{EntryArgs, EntryCallees, TaskEntry, truncate_indefinite};

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
    assert_eq!(proven, 9);
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

/// A collaborator for the aim update that answers fixed values and logs.
struct Driver {
    go: u32,
    answer: [u32; 4],
    calls: Vec<&'static str>,
}

impl AimCallees for Driver {
    fn advance(&mut self, _driver: u32, _owner: u32, _record: [u32; 4], _target: Vec3) -> u32 {
        self.calls.push("advance");
        self.go
    }
    fn driver_call(&mut self, _driver: u32, _record: [u32; 4]) -> [u32; 4] {
        self.calls.push("driver");
        self.answer
    }
}

#[test]
fn aim_direction_is_normalised_to_unit_length() {
    let (x, y, z) = lf_peds_tasks::ped_task::normalised_direction(3.0, 4.0);
    assert!((x - 0.6).abs() < 1e-6);
    assert!((y - 0.8).abs() < 1e-6);
    assert_eq!(z, 0.0);
    // A zero-length aim gives a zero direction, not a division by zero.
    let (x, y, _) = lf_peds_tasks::ped_task::normalised_direction(0.0, 0.0);
    assert_eq!((x, y), (0.0, 0.0));
}

#[test]
fn aim_without_a_driver_answers_zero_and_keeps_the_direction() {
    let owner = AimOwner {
        position: [0.0, 0.0],
        driver: None,
    };
    let task = AimTask {
        accumulated: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut record = [9_u32; 4];
    let mut driver = Driver {
        go: 1,
        answer: [0; 4],
        calls: Vec::new(),
    };
    let answer = task.update(
        0x1000,
        &owner,
        &mut target,
        [0.0, 5.0],
        &mut record,
        &mut driver,
    );
    assert_eq!(answer, 0);
    assert!(driver.calls.is_empty());
    assert_eq!(
        target,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        "the target is not moved"
    );
    assert_eq!(record[3], 0, "the trailing word is zero");
    assert_eq!(f32::from_bits(record[1]), 1.0, "the unit direction along y");
}

#[test]
fn aim_stops_when_the_first_collaborator_answers_a_zero_low_byte() {
    let owner = AimOwner {
        position: [0.0, 0.0],
        driver: Some(0x2000),
    };
    let task = AimTask {
        accumulated: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    };
    let mut target = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut record = [0_u32; 4];
    let mut driver = Driver {
        go: 0x0000_0100,
        answer: [7, 7, 7, 0x1234_5600],
        calls: Vec::new(),
    };
    let answer = task.update(
        0x1000,
        &owner,
        &mut target,
        [1.0, 0.0],
        &mut record,
        &mut driver,
    );
    assert_eq!(answer, 0);
    assert_eq!(driver.calls, vec!["advance"]);
    assert_eq!(
        target,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    );
}

#[test]
fn aim_on_continue_adds_the_offset_and_returns_the_answer_with_its_low_byte_set() {
    let owner = AimOwner {
        position: [0.0, 0.0],
        driver: Some(0x2000),
    };
    let task = AimTask {
        accumulated: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    };
    let mut target = Vec3 {
        x: 10.0,
        y: 20.0,
        z: 30.0,
    };
    let mut record = [0_u32; 4];
    let mut driver = Driver {
        go: 0x0000_0001,
        answer: [1, 2, 3, 0x1234_5600],
        calls: Vec::new(),
    };
    let answer = task.update(
        0x1000,
        &owner,
        &mut target,
        [1.0, 0.0],
        &mut record,
        &mut driver,
    );
    assert_eq!(driver.calls, vec!["advance", "driver"]);
    assert_eq!(
        target,
        Vec3 {
            x: 11.0,
            y: 22.0,
            z: 33.0
        }
    );
    assert_eq!(
        answer, 0x1234_5601,
        "the last word keeps its bits above the low byte and gains a one"
    );
    assert_eq!(record, [1, 2, 3, 0x1234_5600]);
}

/// A flag collaborator that logs its calls and answers fixed values.
#[derive(Default)]
struct Flags {
    draw: u32,
    accept: u32,
    calls: Vec<&'static str>,
}

impl FlagCallees for Flags {
    fn draw(&mut self) -> u32 {
        self.calls.push("draw");
        self.draw
    }
    fn accepts(&mut self, _component: u32, _subject: u32) -> u32 {
        self.calls.push("accepts");
        self.accept
    }
    fn find_task(&mut self, _component: u32, _owner: u32, _enable: u32) -> u32 {
        self.calls.push("find");
        0x3000
    }
    fn fill_scratch(&mut self, _owner: u32) -> u32 {
        self.calls.push("fill");
        0
    }
    fn apply_task(&mut self, _task: u32) -> u32 {
        self.calls.push("apply");
        0
    }
    fn fallback(&mut self, _owner: u32) -> u32 {
        self.calls.push("fallback");
        0
    }
}

fn flag_owner(task_flags: u8, subject: Option<SubjectFlag>, mode: u8) -> FlagOwner {
    FlagOwner {
        handle: 0x1000,
        task_flags,
        subject,
        mode,
    }
}

#[test]
fn flag_update_skips_the_subject_when_the_skip_bit_is_set() {
    let mut owner = flag_owner(
        0x08,
        Some(SubjectFlag {
            handle: 0x2000,
            flag: 0,
        }),
        0,
    );
    let mut flags = Flags {
        draw: 0,
        accept: 1,
        calls: Vec::new(),
    };
    owner.update(&mut flags);
    assert!(
        !flags.calls.contains(&"draw"),
        "no draw when the skip bit is set"
    );
    assert_eq!(
        owner.subject.unwrap().flag,
        0,
        "the subject flag is unchanged"
    );
}

#[test]
fn flag_update_without_a_subject_draws_nothing() {
    let mut owner = flag_owner(0x00, None, 0);
    let mut flags = Flags {
        draw: 0,
        accept: 0,
        calls: Vec::new(),
    };
    owner.update(&mut flags);
    assert_eq!(flags.calls, vec!["accepts", "fallback"]);
}

#[test]
fn flag_update_sets_the_bit_below_the_threshold_and_clears_it_above() {
    let set_case = |draw: u32| {
        let mut owner = flag_owner(
            0x00,
            Some(SubjectFlag {
                handle: 0x2000,
                flag: 0,
            }),
            0,
        );
        let mut flags = Flags {
            draw,
            accept: 0,
            calls: Vec::new(),
        };
        owner.update(&mut flags);
        owner.subject.unwrap().flag
    };
    assert_eq!(
        set_case(0x3FFE),
        0x0200_0000,
        "a draw below the threshold sets the bit"
    );
    assert_eq!(set_case(0x3FFF), 0, "a draw at the threshold clears it");
    let mut owner = flag_owner(
        0x00,
        Some(SubjectFlag {
            handle: 0x2000,
            flag: 0x0200_0000,
        }),
        0,
    );
    let mut flags = Flags {
        draw: 0x4000,
        accept: 0,
        calls: Vec::new(),
    };
    owner.update(&mut flags);
    assert_eq!(
        owner.subject.unwrap().flag,
        0,
        "a set bit is cleared by a draw at or above the threshold"
    );
}

#[test]
fn flag_update_applies_the_task_when_accepted_and_falls_back_otherwise() {
    let mut owner = flag_owner(0x00, None, 0);
    let mut accepted = Flags {
        draw: 0,
        accept: 0x0000_0101,
        calls: Vec::new(),
    };
    owner.update(&mut accepted);
    assert_eq!(accepted.calls, vec!["accepts", "find", "fill", "apply"]);
    let mut refused = Flags {
        draw: 0,
        accept: 0x0000_0100,
        calls: Vec::new(),
    };
    owner.update(&mut refused);
    assert_eq!(
        refused.calls,
        vec!["accepts", "fallback"],
        "only the low byte decides"
    );
}

/// An entry collaborator that logs its calls and answers fixed values.
#[derive(Default)]
struct Entry {
    token: u32,
    fetched: u32,
    mapped: u32,
    calls: Vec<String>,
}

impl EntryCallees for Entry {
    fn setup(&mut self, entry: u32) {
        self.calls.push(format!("setup {entry:#x}"));
    }
    fn resolve(&mut self, registry: u32, key: u32) -> u32 {
        self.calls.push(format!("resolve {registry:#x} {key:#x}"));
        self.token
    }
    fn fetch(&mut self, token: u32) -> u32 {
        self.calls.push(format!("fetch {token:#x}"));
        self.fetched
    }
    fn map(&mut self, index: u32) -> u32 {
        self.calls.push(format!("map {index:#x}"));
        self.mapped
    }
}

fn plain_args(source_key: Option<u32>, convert: u32) -> EntryArgs {
    EntryArgs {
        kind: 0x1_05,
        kind_flag: 0x2_07,
        source: if source_key.is_some() { 0x4000 } else { 0 },
        source_key,
        convert,
    }
}

fn blank_entry() -> TaskEntry {
    TaskEntry {
        kind: 0,
        kind_flag: 0,
        source: 0,
        cleared_99c: 1,
        handle: 0,
        mapped: 0,
        cleared_aa0: 1,
        sentinels: [0; 4],
        enabled: 0,
        cleared_bb8: 1,
    }
}

#[test]
fn entry_init_writes_the_kind_slots_defaults_and_sentinels() {
    let mut entry = blank_entry();
    let mut calls = Entry::default();
    entry.initialise(0x1000, plain_args(None, 0), 0x500, 1.0, &mut calls);
    assert_eq!(entry.kind, 0x05, "the first slot keeps its low byte");
    assert_eq!(entry.kind_flag, 0x07, "the second slot keeps its low byte");
    assert_eq!(entry.source, 0, "no source");
    assert_eq!(
        (entry.cleared_99c, entry.cleared_aa0, entry.cleared_bb8),
        (0, 0, 0)
    );
    assert_eq!(entry.enabled, 1);
    assert_eq!(entry.sentinels, [u32::MAX; 4]);
    assert_eq!(
        calls.calls,
        vec!["setup 0x1000"],
        "no resolve without a source"
    );
    assert_eq!(entry.handle, 0, "the handle keeps its earlier value");
}

#[test]
fn entry_init_resolves_and_maps_when_asked() {
    let mut entry = blank_entry();
    entry.handle = 0x0100_0000;
    let mut calls = Entry {
        token: 0x77,
        fetched: 0x8000_0000,
        mapped: 0x42,
        calls: Vec::new(),
    };
    entry.initialise(0x1000, plain_args(Some(0x99), 1), 0x500, 1.0, &mut calls);
    assert_eq!(
        calls.calls,
        vec![
            "setup 0x1000",
            "resolve 0x500 0x99",
            "fetch 0x77",
            "map 0x80000000"
        ]
    );
    assert_eq!(entry.handle, 0x8000_0000);
    assert_eq!(entry.mapped, 0x42);
}

#[test]
fn entry_init_keeps_the_handle_when_the_token_is_zero() {
    let mut entry = blank_entry();
    entry.handle = 0x1234;
    let mut calls = Entry {
        token: 0,
        fetched: 0x9999,
        mapped: 0,
        calls: Vec::new(),
    };
    entry.initialise(0x1000, plain_args(Some(0x99), 0), 0x500, 1.0, &mut calls);
    assert_eq!(entry.handle, 0x1234, "no token, no fetch, no change");
    assert_eq!(calls.calls, vec!["setup 0x1000", "resolve 0x500 0x99"]);
}

#[test]
fn entry_conversion_truncates_and_takes_the_indefinite_value_outside_the_range() {
    assert_eq!(truncate_indefinite(-999.9), -999);
    assert_eq!(truncate_indefinite(2.9), 2);
    assert_eq!(truncate_indefinite(f32::NAN), i32::MIN);
    assert_eq!(truncate_indefinite(3.0e9), i32::MIN);
    assert_eq!(truncate_indefinite(-2_147_483_648.0), i32::MIN);
}
