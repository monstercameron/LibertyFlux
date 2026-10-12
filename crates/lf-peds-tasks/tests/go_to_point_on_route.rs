//! Host tests of the lifted go-to-point-on-route task: the edge cases a reader
//! of the code would ask about. These run on every host; the differential
//! proof against the verified rewrites is in `lf-gotopoint-diff`.

use lf_peds_tasks::go_to_point_on_route::{
    CopyArgs, GoToPointOnRouteHost, GoToPointOnRouteTask, ProgressCache, TaskField,
};

/// A host that records the calls and answers from fixed values.
#[derive(Default)]
struct FakeHost {
    refreshes: Vec<u32>,
    /// Bits the refresh sets in the task's flag word, as a callee may.
    refresh_sets: u32,
    alloc_answer: u32,
    ctor_answer: u32,
    constructed: Vec<(u32, CopyArgs)>,
    allocations: u32,
    link_flags: u32,
    ped_pos: [f32; 2],
    helper_answer: u64,
    helper_args: Vec<u64>,
}

/// The helper's argument: one eighth of pi, as the bits of a double.
const PI_EIGHTH_BITS: u64 = 0x3FD9_21FB_6000_0000;

impl GoToPointOnRouteHost for FakeHost {
    fn refresh_route(&mut self, task: &mut GoToPointOnRouteTask, arg: u32) {
        self.refreshes.push(arg);
        task.flags_c4 |= self.refresh_sets;
    }

    fn allocate_task(&mut self) -> u32 {
        self.allocations += 1;
        self.alloc_answer
    }

    fn construct_copy(&mut self, new_task: u32, args: &CopyArgs) -> u32 {
        self.constructed.push((new_task, *args));
        self.ctor_answer
    }

    fn ped_link_flags(&mut self, _ped: u32) -> u32 {
        self.link_flags
    }

    fn ped_position(&mut self, _ped: u32) -> [f32; 2] {
        self.ped_pos
    }

    fn double_helper(&mut self, arg_bits: u64) -> u64 {
        self.helper_args.push(arg_bits);
        self.helper_answer
    }
}

fn task() -> GoToPointOnRouteTask {
    GoToPointOnRouteTask::default()
}

#[test]
fn progress_is_positive_only_above_zero() {
    let mut t = task();
    t.progress = 0.0;
    assert!(!t.progress_is_positive());
    t.progress = -0.0;
    assert!(!t.progress_is_positive());
    t.progress = f32::NAN;
    assert!(!t.progress_is_positive());
    t.progress = f32::from_bits(1);
    assert!(t.progress_is_positive());
    t.progress = f32::INFINITY;
    assert!(t.progress_is_positive());
}

#[test]
fn speed_is_clamped_to_zero_and_three() {
    let mut t = task();
    t.set_speed(-1.0);
    assert_eq!(t.speed, 0.0);
    t.set_speed(7.5);
    assert_eq!(t.speed, 3.0);
    t.set_speed(1.25);
    assert_eq!(t.speed, 1.25);
    // Neither bound compares greater for a NaN request, so it is stored as given.
    let nan = f32::from_bits(0x7FC0_1234);
    t.set_speed(nan);
    assert_eq!(t.speed.to_bits(), 0x7FC0_1234);
    // A negative zero is not below zero: it is stored with its sign.
    t.set_speed(-0.0);
    assert_eq!(t.speed.to_bits(), (-0.0f32).to_bits());
}

#[test]
fn position_point_is_the_first_three_words() {
    let mut t = task();
    t.position = [1.0, 2.0, 3.0, 4.0];
    assert_eq!(t.position_point(), [1.0, 2.0, 3.0]);
}

#[test]
fn position_is_replaced_only_for_a_change_or_force() {
    let mut t = task();
    t.position = [1.0, 2.0, 3.0, 0.0];
    t.flags_b0 = 0xFFFF_FFFF;

    // Equal coordinates and a force word whose low byte is zero: nothing changes,
    // even though the force word has other bits set.
    t.set_position([1.0, 2.0, 3.0, 9.0], 0x100);
    assert_eq!(t.position, [1.0, 2.0, 3.0, 0.0]);
    assert_eq!(t.flags_b0, 0xFFFF_FFFF);

    // A force word with a low byte set replaces the position even when equal.
    t.set_position([1.0, 2.0, 3.0, 9.0], 0x101);
    assert_eq!(t.position, [1.0, 2.0, 3.0, 9.0]);
    // The position-change bits (0x7800) clear; bit 0x100 is set.
    assert_eq!(t.flags_b0, 0xFFFF_87FF);

    // An unordered coordinate never compares equal, so it replaces the position.
    let mut u = task();
    u.position = [f32::NAN, 0.0, 0.0, 0.0];
    u.set_position([f32::NAN, 0.0, 0.0, 5.0], 0);
    assert_eq!(u.position[3], 5.0);
}

#[test]
fn signed_zero_coordinates_compare_equal_and_keep_their_bits() {
    let mut t = task();
    t.position = [0.0, 0.0, 0.0, 0.0];
    t.flags_b0 = 0x1234_5678;
    t.set_position([-0.0, -0.0, -0.0, 1.0], 0);
    // Equal values: the position is untouched, so the stored zeros keep their sign.
    assert_eq!(t.position[0].to_bits(), 0.0f32.to_bits());
    assert_eq!(t.position[3], 0.0);
    assert_eq!(t.flags_b0, 0x1234_5678);
}

#[test]
fn value_8c_keeps_equal_values_and_takes_unordered_ones() {
    let mut t = task();
    t.value_8c = 0.0;
    t.set_value_8c(-0.0);
    assert_eq!(t.value_8c.to_bits(), 0.0f32.to_bits());
    let nan = f32::from_bits(0x7FC0_ABCD);
    t.set_value_8c(nan);
    assert_eq!(t.value_8c.to_bits(), 0x7FC0_ABCD);
    t.set_value_8c(2.5);
    assert_eq!(t.value_8c, 2.5);
}

#[test]
fn refresh_mode_one_and_two_return_one_and_leave_the_flags() {
    let mut h = FakeHost::default();
    let mut t = task();
    t.flags_c4 = 0x0000_0000;
    assert_eq!(t.refresh_mode(&mut h, 42, 1), 1);
    assert_eq!(t.refresh_mode(&mut h, 43, 2), 1);
    assert_eq!(t.flags_c4, 0);
    assert_eq!(h.refreshes, vec![42, 43]);
}

#[test]
fn refresh_mode_other_values_clear_the_low_byte_and_set_the_flag() {
    let mut h = FakeHost::default();
    let mut t = task();
    assert_eq!(t.refresh_mode(&mut h, 0, 0x0000_0102), 0x0000_0100);
    assert_eq!(t.flags_c4 & 0x10, 0x10);
    assert_eq!(t.refresh_mode(&mut h, 0, 0x0000_0000), 0);
    assert_eq!(t.refresh_mode(&mut h, 0, 0xFFFF_FFFF), 0xFFFF_FF00);
    // Mode zero is not one or two, so it takes the second path too.
    assert_eq!(t.refresh_mode(&mut h, 0, 3), 0);
}

#[test]
fn refresh_mode_reads_the_flag_word_after_the_refresh() {
    // The refresh sets bit 0x20 while it runs; the flag word the method then
    // updates must keep it.
    let mut h = FakeHost {
        refresh_sets: 0x20,
        ..FakeHost::default()
    };
    let mut t = task();
    let out = t.refresh_mode(&mut h, 7, 5);
    assert_eq!(out, 0);
    assert_eq!(t.flags_c4, 0x30);
}

#[test]
fn clone_returns_zero_without_constructing_when_no_task_is_allocated() {
    let mut h = FakeHost::default();
    let t = task();
    assert_eq!(t.clone_task(&mut h), 0);
    assert_eq!(h.allocations, 1);
    assert!(h.constructed.is_empty());
}

#[test]
fn clone_hands_the_constructor_the_copy_words() {
    let mut h = FakeHost {
        alloc_answer: 0x0BAD_F00D,
        ctor_answer: 0x5151,
        ..FakeHost::default()
    };
    let t = GoToPointOnRouteTask {
        progress: 2.0,
        word_a0: 0xA0A0_A0A0,
        active: 0x7F,
        flags_c4: 0b10,
        ..GoToPointOnRouteTask::default()
    };
    assert_eq!(t.clone_task(&mut h), 0x5151);
    assert_eq!(h.constructed.len(), 1);
    let (new_task, args) = h.constructed[0];
    assert_eq!(new_task, 0x0BAD_F00D);
    assert_eq!(args.progress_bits, 2.0f32.to_bits());
    assert_eq!(args.block_40, TaskField(0x40));
    assert_eq!(args.block_d0, TaskField(0xD0));
    assert_eq!(args.active, 0x7F);
    assert_eq!(args.word_a0, 0xA0A0_A0A0);
    assert_eq!(args.flag_bit_0, 0);
    assert_eq!(args.flag_bit_1, 1);
}

#[test]
fn idle_advance_changes_nothing_and_fills_no_cache() {
    let mut h = FakeHost {
        helper_answer: 1,
        ..FakeHost::default()
    };
    let mut t = GoToPointOnRouteTask {
        progress: 0.25,
        entry_value: 2.5,
        ..GoToPointOnRouteTask::default()
    };
    t.active = 0;
    let mut cache = ProgressCache::default();
    t.advance_progress(&mut h, &mut cache, 1, [3.0, 4.0]);
    assert_eq!(t.progress, 0.25);
    assert_eq!(cache, ProgressCache::default());
    assert!(h.helper_args.is_empty());
}

#[test]
fn held_link_takes_the_entry_value() {
    let mut h = FakeHost {
        link_flags: 0b10,
        ..FakeHost::default()
    };
    let mut t = GoToPointOnRouteTask {
        entry_value: 5.0,
        active: 1,
        ..GoToPointOnRouteTask::default()
    };
    let mut cache = ProgressCache::default();
    t.advance_progress(&mut h, &mut cache, 1, [3.0, 4.0]);
    assert_eq!(t.progress, 5.0);
    assert_eq!(cache, ProgressCache::default());
}

#[test]
fn entry_at_or_below_one_is_kept_and_a_nan_entry_is_advanced() {
    let mut h = FakeHost::default();
    let mut t = GoToPointOnRouteTask {
        entry_value: 1.0,
        active: 1,
        ..GoToPointOnRouteTask::default()
    };
    let mut cache = ProgressCache::default();
    t.advance_progress(&mut h, &mut cache, 1, [3.0, 4.0]);
    assert_eq!(t.progress, 1.0);
    assert_eq!(cache, ProgressCache::default());

    // A NaN entry value is not at or below one, so the advance runs: it fills
    // the caches, and then the failed threshold test keeps the NaN.
    let nan = f32::from_bits(0x7FC0_0042);
    t.entry_value = nan;
    t.advance_progress(&mut h, &mut cache, 1, [0.0, 0.0]);
    assert_eq!(t.progress.to_bits(), 0x7FC0_0042);
    assert_eq!(cache.flags & 0x7, 0x7);
}

#[test]
fn first_advance_fills_the_cache_and_calls_the_helper_once() {
    let answer = 2.75f64.to_bits();
    let mut h = FakeHost {
        helper_answer: answer,
        ..FakeHost::default()
    };
    let mut t = GoToPointOnRouteTask {
        entry_value: 1.5,
        active: 1,
        ..GoToPointOnRouteTask::default()
    };
    let mut cache = ProgressCache::default();
    // The ped stands at the origin and the target is five away: the threshold
    // (3.0 for this entry value) fails, so the entry value is kept.
    t.advance_progress(&mut h, &mut cache, 1, [3.0, 4.0]);
    assert_eq!(t.progress, 1.5);
    assert_eq!(cache.flags, 0x7);
    assert_eq!(cache.weight_a, 1.5f32.to_bits());
    assert_eq!(cache.weight_b, 2.0f32.to_bits());
    assert_eq!(cache.scale, 2.75f32.to_bits());
    assert_eq!(h.helper_args, vec![PI_EIGHTH_BITS]);

    // The second call finds every cache filled and does not call the helper.
    t.advance_progress(&mut h, &mut cache, 1, [3.0, 4.0]);
    assert_eq!(h.helper_args.len(), 1);
}

#[test]
fn full_advance_blends_the_cosine_with_the_entry_value() {
    // A cache already filled with scale 2.0: the cosine is 0.5, so the blend
    // is 0.25 * 0.5 + 2.0 = 2.125 for an entry value of 2.5.
    let mut h = FakeHost::default();
    let mut t = GoToPointOnRouteTask {
        entry_value: 2.5,
        active: 1,
        anchor: [1.0, 0.0],
        ..GoToPointOnRouteTask::default()
    };
    let mut cache = ProgressCache {
        flags: 0x7,
        weight_a: 1.5f32.to_bits(),
        weight_b: 2.0f32.to_bits(),
        scale: 2.0f32.to_bits(),
    };
    t.advance_progress(&mut h, &mut cache, 1, [0.5, 0.0]);
    assert_eq!(t.progress, 2.125);
    assert!(h.helper_args.is_empty());
}

#[test]
fn zero_scale_skips_the_divide_and_uses_one() {
    // With a zero scale the cosine is taken as one: t = 1, so the blend is
    // 1 * 1 * 0.5 + 2.0 = 2.5 for an entry value of 2.5, and it is not
    // raised above the entry value.
    let mut h = FakeHost::default();
    let mut t = GoToPointOnRouteTask {
        entry_value: 2.5,
        active: 1,
        anchor: [1.0, 0.0],
        ..GoToPointOnRouteTask::default()
    };
    let mut cache = ProgressCache {
        flags: 0x7,
        weight_a: 1.5f32.to_bits(),
        weight_b: 2.0f32.to_bits(),
        scale: 0,
    };
    t.advance_progress(&mut h, &mut cache, 1, [0.5, 0.0]);
    assert_eq!(t.progress, 2.5);
}
