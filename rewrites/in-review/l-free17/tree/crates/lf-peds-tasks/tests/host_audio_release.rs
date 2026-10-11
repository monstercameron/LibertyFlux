//! Host tests of the audio slot release on the 64-bit host. Objects are
//! handles; the resource and helper calls are recorded.

use lf_peds_tasks::ped_task::{
    AudioObjects, AudioSlot, AudioTask, CLEANUP_ENABLED, CLEANUP_MODE_SENTINEL, Deadline,
    PROGRESS_LIMIT, ReleaseGlobals, SKIP_MODE,
};

#[derive(Default)]
struct Objects {
    calls: Vec<String>,
}

impl AudioObjects for Objects {
    fn resource(&self, object: u32) -> u32 {
        object + 100
    }

    fn cleanup(&mut self, resource: u32) {
        self.calls.push(format!("cleanup {resource}"));
    }

    fn release(&mut self, object: u32) {
        self.calls.push(format!("release {object}"));
    }
}

fn open() -> ReleaseGlobals {
    ReleaseGlobals {
        cleanup_enabled: 0,
        current_mode: CLEANUP_MODE_SENTINEL,
        skip_mode: 0,
    }
}

#[test]
fn a_null_slot_does_nothing() {
    let slots = AudioTask::default();
    let mut objects = Objects::default();
    slots.release_slot(AudioSlot::Radio, open(), &mut objects);
    assert!(objects.calls.is_empty());
}

#[test]
fn cleanup_runs_before_release_when_all_conditions_hold() {
    let slots = AudioTask {
        primary: 7,
        ..AudioTask::default()
    };
    let mut objects = Objects::default();
    slots.release_slot(AudioSlot::Primary, open(), &mut objects);
    assert_eq!(objects.calls, ["cleanup 107", "release 7"]);
}

#[test]
fn each_suppressing_condition_skips_cleanup() {
    let slots = AudioTask {
        secondary: 3,
        ..AudioTask::default()
    };
    let suppressed = [
        ReleaseGlobals {
            cleanup_enabled: CLEANUP_ENABLED,
            ..open()
        },
        ReleaseGlobals {
            current_mode: 0,
            ..open()
        },
        ReleaseGlobals {
            skip_mode: SKIP_MODE,
            ..open()
        },
    ];
    for globals in suppressed {
        let mut objects = Objects::default();
        slots.release_slot(AudioSlot::Secondary, globals, &mut objects);
        assert_eq!(objects.calls, ["release 3"]);
    }
}

#[test]
fn each_slot_reads_its_own_word() {
    let slots = AudioTask {
        primary: 1,
        radio: 2,
        secondary: 3,
        ..AudioTask::default()
    };
    assert_eq!(slots.get(AudioSlot::Primary), 1);
    assert_eq!(slots.get(AudioSlot::Radio), 2);
    assert_eq!(slots.get(AudioSlot::Secondary), 3);
}

#[test]
fn window_is_open_for_a_thousand_ticks_after_either_stamp() {
    let now = 10_000;
    let task = AudioTask {
        second_deadline: now - 999,
        window_deadline: now - 2000,
        ..AudioTask::default()
    };
    assert!(task.has_unexpired_window(now));
    let closed = AudioTask {
        second_deadline: now - 1000,
        window_deadline: now - 1000,
        ..AudioTask::default()
    };
    assert!(!closed.has_unexpired_window(now));
}

#[test]
fn window_compares_across_the_wrap() {
    // A stamp just before the top of the range is still inside the window
    // when the clock has wrapped past zero.
    let task = AudioTask {
        second_deadline: u32::MAX - 5,
        ..AudioTask::default()
    };
    assert!(task.has_unexpired_window(10));
}

#[test]
fn stamps_write_only_their_own_word() {
    let mut task = AudioTask::default();
    task.stamp_deadline(Deadline::First, 77);
    assert_eq!((task.first_deadline, task.second_deadline), (77, 0));
    task.stamp_deadline(Deadline::Second, 88);
    assert_eq!((task.first_deadline, task.second_deadline), (77, 88));
}

#[test]
fn progress_first_value_never_moves_down_and_second_is_capped() {
    let mut task = AudioTask {
        first_progress: 0.8,
        second_progress: 0.9,
        ..AudioTask::default()
    };
    task.add_float_progress(0.0);
    assert_eq!(task.first_progress, 0.8);
    assert!((task.second_progress - PROGRESS_LIMIT).abs() < f32::EPSILON);
    task.add_float_progress(50.0);
    assert_eq!(task.first_progress, PROGRESS_LIMIT);
}