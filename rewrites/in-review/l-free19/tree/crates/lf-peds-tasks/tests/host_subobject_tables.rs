//! Host tests of the sub-object table walk: the edge cases a reader of the
//! frame remap and the matrix transform would ask about. They run on every
//! host; the differential cases against the verified rewrites are in the
//! `lf-taskshapediff2` test crate.

use lf_peds_tasks::ped_task::subobject_tables::{
    FLAG_BYTES, OBJECT_BASE, OBJECT_STRIDE, SLOT_CAPACITY, TABLE_COUNT, UNINITIALISED_SCRATCH_WORD,
    object_offset,
};
use lf_peds_tasks::ped_task::{AffineBlock, CountReads, SubobjectHooks, TaskSubobjects, WalkEnd};

/// A hook that logs each call and can write the record during one call.
#[derive(Default)]
struct Log {
    calls: Vec<(usize, usize)>,
    refreshes: u32,
    /// When set, the notify with this call number writes this count.
    write_count_at: Option<(usize, usize, i32)>,
}

impl SubobjectHooks for Log {
    type Handle = u32;

    fn refresh(&mut self, _owner: &mut TaskSubobjects) {
        self.refreshes += 1;
    }

    fn notify(&mut self, owner: &mut TaskSubobjects, table: usize, slot: usize, _handle: u32) {
        self.calls.push((table, slot));
        if let Some((at, table_to_write, value)) = self.write_count_at {
            if at == self.calls.len() - 1 {
                owner.counts[table_to_write] = value;
            }
        }
    }
}

/// A record with the given counts and flag bytes and zero everywhere else.
fn record(counts: [i32; TABLE_COUNT], flags: &[u8]) -> TaskSubobjects {
    let mut owner = TaskSubobjects {
        anchor: [0.0; 3],
        out: [0.0; 3],
        spare: 0,
        counts,
        flags: [0; FLAG_BYTES],
    };
    owner.flags[..flags.len()].copy_from_slice(flags);
    owner
}

/// A block with identity rows and zero bias.
fn identity() -> AffineBlock {
    AffineBlock {
        rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        bias: [0.0; 3],
    }
}

#[test]
fn record_geometry_matches_the_tables() {
    assert_eq!(FLAG_BYTES, 11);
    assert_eq!(SLOT_CAPACITY, 8);
    assert_eq!(object_offset(0, 0), OBJECT_BASE[0]);
    assert_eq!(object_offset(2, 3), 0x200 + 3 * 0xe0);
    assert_eq!(OBJECT_STRIDE[3], 0x120);
    assert_eq!(UNINITIALISED_SCRATCH_WORD, 0);
}

#[test]
fn no_flagged_slot_makes_only_the_refresh_call() {
    let mut owner = record([3, 3, 3, 3], &[]);
    let mut log = Log::default();
    let end = owner.remap_vectors(&mut log, &identity(), 7);
    assert_eq!(log.refreshes, 1);
    assert!(log.calls.is_empty());
    // Every table walks its three slots, so the last table's answer stands.
    assert_eq!(end, WalkEnd::Object { table: 3, objects: 3 });
}

#[test]
fn all_counts_zero_or_negative_answers_none_and_walks_nothing() {
    let mut owner = record([0, -1, -5, 0], &[1; FLAG_BYTES]);
    let mut log = Log::default();
    let end = owner.apply_matrix(&mut log, &identity(), 1);
    assert_eq!(end, WalkEnd::None);
    assert!(log.calls.is_empty());
    assert_eq!(log.refreshes, 1);
    assert_eq!(owner.spare, UNINITIALISED_SCRATCH_WORD);
}

#[test]
fn table_zero_alone_answers_its_slot_count() {
    let mut owner = record([3, 0, 0, 0], &[1, 1, 1]);
    let mut log = Log::default();
    let end = owner.apply_matrix(&mut log, &identity(), 9);
    assert_eq!(end, WalkEnd::Count(3));
    assert_eq!(log.calls, vec![(0, 0), (0, 1), (0, 2)]);
}

#[test]
fn a_later_table_answers_one_past_its_last_object() {
    let mut owner = record([0, 0, 0, 2], &[0, 0, 0, 1, 1]);
    let mut log = Log::default();
    let end = owner.remap_vectors(&mut log, &identity(), 4);
    assert_eq!(
        end,
        WalkEnd::Object {
            table: 3,
            objects: 2
        }
    );
    assert_eq!(log.calls, vec![(3, 0), (3, 1)]);
    assert_eq!(object_offset(3, 2), 0x2e0 + 2 * 0x120);
}

#[test]
fn flags_overlap_between_tables() {
    // Table 0's slot 1 and table 1's slot 0 are the same flag byte: index 1.
    let mut owner = record([2, 2, 0, 0], &[0, 1, 0]);
    let mut log = Log::default();
    owner.apply_matrix(&mut log, &identity(), 0);
    assert_eq!(log.calls, vec![(0, 1), (1, 0)]);
}

#[test]
fn remap_rereads_the_count_and_the_transform_does_not() {
    let mut per_iteration = record([3, 0, 0, 0], &[1, 1, 1]);
    let mut log = Log {
        write_count_at: Some((0, 0, 1)),
        ..Log::default()
    };
    let end = per_iteration.remap_vectors(&mut log, &identity(), 0);
    assert_eq!(log.calls, vec![(0, 0)]);
    assert_eq!(end, WalkEnd::Count(1));

    let mut once = record([3, 0, 0, 0], &[1, 1, 1]);
    let mut log = Log {
        write_count_at: Some((0, 0, 1)),
        ..Log::default()
    };
    let end = once.apply_matrix(&mut log, &identity(), 0);
    assert_eq!(log.calls, vec![(0, 0), (0, 1), (0, 2)]);
    assert_eq!(end, WalkEnd::Count(3));
    assert_ne!(CountReads::PerIteration, CountReads::OncePerTable);
}

#[test]
fn negative_count_skips_its_table_and_keeps_the_earlier_answer() {
    let mut owner = record([1, -1, 0, 0], &[1, 1]);
    let mut log = Log::default();
    let end = owner.apply_matrix(&mut log, &identity(), 0);
    assert_eq!(log.calls, vec![(0, 0)]);
    assert_eq!(end, WalkEnd::Count(1));
}

#[test]
fn transform_blends_outputs_through_the_matrix_and_bias() {
    let mut owner = record([0; 4], &[]);
    owner.out = [1.0, 2.0, 3.0];
    let matrix = AffineBlock {
        rows: [[1.0, 10.0, 100.0], [1000.0, 10000.0, 100000.0], [2.0, 20.0, 200.0]],
        bias: [0.5, -0.5, 0.25],
    };
    owner.apply_matrix(&mut Log::default(), &matrix, 0);
    // row r: m[1][r]*w1 + m[0][r]*w0 + m[2][r]*w2 + bias[r]
    assert_eq!(owner.out[0], 2000.0 + 1.0 + 6.0 + 0.5);
    assert_eq!(owner.out[1], 20000.0 + 10.0 + 60.0 - 0.5);
    assert_eq!(owner.out[2], 200000.0 + 100.0 + 600.0 + 0.25);
}

#[test]
fn remap_measures_the_anchor_from_the_frame_offset() {
    let mut owner = record([0; 4], &[]);
    owner.anchor = [5.0, 7.0, 9.0];
    let frame = AffineBlock {
        rows: [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]],
        bias: [1.0, 2.0, 3.0],
    };
    owner.remap_vectors(&mut Log::default(), &frame, 0);
    // d = anchor - offset = (4, 5, 6); rows dot d.
    assert_eq!(owner.out, [4.0, 10.0, 18.0]);
}

#[test]
fn a_not_a_number_weight_reaches_every_row() {
    // The first weight multiplies a zero matrix entry in rows 1 and 2, and
    // zero times not-a-number is not-a-number, so every row is poisoned.
    let mut owner = record([0; 4], &[]);
    owner.out = [f32::NAN, 1.0, 1.0];
    owner.apply_matrix(&mut Log::default(), &identity(), 0);
    assert!(owner.out.iter().all(|value| value.is_nan()));
}

#[test]
#[should_panic(expected = "slot count above the table capacity")]
fn a_count_above_the_capacity_is_refused() {
    let mut owner = record([SLOT_CAPACITY as i32 + 1, 0, 0, 0], &[]);
    owner.apply_matrix(&mut Log::default(), &identity(), 0);
}
