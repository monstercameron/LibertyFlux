//! Host tests of the task predicates and the task state block: the edge
//! cases a reader would ask about, on the 64-bit host. The 32-bit
//! differential cases live in the `lf-taskshapediff` test crate.

use lf_peds_tasks::ped_task::registry::State;
use lf_peds_tasks::ped_task::shape_registry;
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
    assert_eq!(proven, 3);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 0);
    for row in shape_registry::ROWS {
        assert_eq!(row.state, State::Proven);
        assert!(!row.narrows.is_empty(), "{} {} states its narrowings", row.class, row.method);
    }
}
