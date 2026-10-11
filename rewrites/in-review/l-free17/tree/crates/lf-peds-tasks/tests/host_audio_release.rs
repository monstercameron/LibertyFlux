//! Host tests of the audio slot release on the 64-bit host. Objects are
//! handles; the resource and helper calls are recorded.

use lf_peds_tasks::ped_task::{
    AudioObjects, AudioSlot, AudioTask, CLEANUP_ENABLED, CLEANUP_MODE_SENTINEL, ReleaseGlobals,
    SKIP_MODE,
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
    };
    assert_eq!(slots.get(AudioSlot::Primary), 1);
    assert_eq!(slots.get(AudioSlot::Radio), 2);
    assert_eq!(slots.get(AudioSlot::Secondary), 3);
}
