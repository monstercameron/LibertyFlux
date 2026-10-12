//! Host tests for the lifted game camera methods.
//!
//! These run on every host and need no rewrite: they check the edge cases a
//! reader of the lifted code would ask about. The differential proof against
//! the verified 32-bit rewrites is in the `lf-camgame-diff` test crate.

use lf_core::Handle32;
use lf_world::cam_game::registry::{self, State};
use lf_world::cam_game::{
    ACTIVE_MARKER, CCamGame, CamGameWorld, CamOwnerTag, CamTag, DEFAULT_ZOOM, IDLE_MARKER,
    IDLE_MODE, IDLE_TIMER, PoolTag, WorkerTag,
};

/// A world that records each call and answers from fixed values.
#[derive(Default)]
struct Recorder {
    calls: Vec<String>,
    manager: Option<Handle32<PoolTag>>,
    worker: Option<Handle32<WorkerTag>>,
    default_word: u32,
    /// Byte value the reset step writes into the first flag byte.
    reset_flags: Option<u8>,
}

impl CamGameWorld for Recorder {
    fn destroy_body(&mut self, _cam: &mut CCamGame, this: Handle32<CamTag>) {
        self.calls.push(format!("destroy_body {}", this.get()));
    }

    fn pool_manager(&mut self) -> Option<Handle32<PoolTag>> {
        self.calls.push("pool_manager".to_string());
        self.manager
    }

    fn pool_release(&mut self, manager: Option<Handle32<PoolTag>>, this: Handle32<CamTag>) {
        self.calls.push(format!(
            "pool_release {} {}",
            manager.map_or(0, Handle32::get),
            this.get()
        ));
    }

    fn teardown(&mut self, _cam: &mut CCamGame, this: Handle32<CamTag>) {
        self.calls.push(format!("teardown {}", this.get()));
    }

    fn create_worker(
        &mut self,
        owner: Option<Handle32<CamOwnerTag>>,
        mode: u32,
        extra: u32,
        this: Handle32<CamTag>,
    ) -> Option<Handle32<WorkerTag>> {
        self.calls.push(format!(
            "create_worker {} {mode} {extra} {}",
            owner.map_or(0, Handle32::get),
            this.get()
        ));
        self.worker
    }

    fn chain_set_flags(&mut self, worker: Option<Handle32<WorkerTag>>) {
        self.calls
            .push(format!("chain_set_flags {}", worker.map_or(0, Handle32::get)));
    }

    fn release_worker(&mut self, worker: Option<Handle32<WorkerTag>>) {
        self.calls
            .push(format!("release_worker {}", worker.map_or(0, Handle32::get)));
    }

    fn reset_state_block(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
        self.calls.push(format!("reset_state_block {}", this.get()));
        if let Some(flags) = self.reset_flags {
            cam.flags_1c4 = flags;
        }
    }

    fn shared_default_word(&mut self) -> u32 {
        self.calls.push("shared_default_word".to_string());
        self.default_word
    }
}

fn this() -> Handle32<CamTag> {
    Handle32::new(0x1000).expect("non-zero identity")
}

#[test]
fn registry_counts_three_proven_methods() {
    assert_eq!(registry::proven_count(), 3);
    let missing = registry::ROWS
        .iter()
        .filter(|r| r.state == State::Missing)
        .count();
    assert_eq!(missing, 4);
}

#[test]
fn destructor_releases_only_when_low_bit_set() {
    let mut cam = CCamGame::default();
    let mut w = Recorder::default();
    cam.deleting_destructor(&mut w, this(), 0x2);
    assert_eq!(w.calls, vec!["destroy_body 4096"]);

    let mut w = Recorder {
        manager: Handle32::new(0x77),
        ..Recorder::default()
    };
    cam.deleting_destructor(&mut w, this(), 0xFFFF_FFFF);
    assert_eq!(
        w.calls,
        vec![
            "destroy_body 4096",
            "pool_manager",
            "pool_release 119 4096"
        ]
    );
}

#[test]
fn destructor_passes_zero_manager_as_none() {
    let mut cam = CCamGame::default();
    let mut w = Recorder::default();
    cam.deleting_destructor(&mut w, this(), 1);
    assert_eq!(w.calls.last().map(String::as_str), Some("pool_release 0 4096"));
}

#[test]
fn restart_passes_absent_owner_and_worker_as_none() {
    let mut cam = CCamGame::default();
    let mut w = Recorder::default();
    cam.restart(&mut w, this());
    assert_eq!(
        w.calls,
        vec![
            "teardown 4096",
            "create_worker 0 1 0 4096",
            "chain_set_flags 0",
            "release_worker 0",
        ]
    );
}

#[test]
fn restart_clears_active_marker_and_pending_words_only() {
    let mut cam = CCamGame {
        owner: Handle32::new(0x55),
        mode: 9,
        flags_1c4: 0xFF,
        pending_1c8: 1,
        pending_1cc: 2,
        pending_1d0: 3,
        ..CCamGame::default()
    };
    let mut w = Recorder {
        worker: Handle32::new(0x66),
        ..Recorder::default()
    };
    cam.restart(&mut w, this());
    assert_eq!(cam.flags_1c4, 0xFF & !ACTIVE_MARKER);
    assert_eq!((cam.pending_1c8, cam.pending_1cc, cam.pending_1d0), (0, 0, 0));
    assert_eq!(cam.mode, 9, "restart leaves the mode word alone");
    assert_eq!(
        w.calls[1],
        "create_worker 85 1 0 4096",
        "owner is passed as read after the teardown"
    );
}

#[test]
fn idle_reset_reads_flag_byte_after_the_reset_step() {
    let mut cam = CCamGame {
        flags_1c4: 0x00,
        flags_1c5: 0xFF,
        ..CCamGame::default()
    };
    // The reset step leaves 0xFF in the first flag byte: bits 3 to 5 go,
    // the idle marker is set, the rest stays.
    let mut w = Recorder {
        default_word: 0xABCD,
        reset_flags: Some(0xFF),
        ..Recorder::default()
    };
    cam.reset_to_idle(&mut w, this());
    assert_eq!(cam.flags_1c4, (0xFF & 0xC7) | IDLE_MARKER);
    assert_eq!(cam.flags_1c5, 0xFE);
    assert_eq!(cam.default_word, 0xABCD);
    assert_eq!(cam.mode, IDLE_MODE);
    assert_eq!(cam.timer, IDLE_TIMER);
    assert_eq!(cam.zoom_bits, DEFAULT_ZOOM.to_bits());
    assert_eq!(cam.zoom_bits, 0x3F80_0000);
    assert_eq!(
        w.calls,
        vec![
            "shared_default_word",
            "reset_state_block 4096",
        ]
    );
}

#[test]
fn idle_reset_without_a_reset_step_change_clears_the_low_flag_bits_first() {
    let mut cam = CCamGame {
        flags_1c4: 0xFF,
        ..CCamGame::default()
    };
    let mut w = Recorder::default();
    cam.reset_to_idle(&mut w, this());
    // Low three bits cleared before the step, bits 3 to 5 cleared after it.
    assert_eq!(cam.flags_1c4, (0xF8 & 0xC7) | IDLE_MARKER);
}

#[test]
fn zero_owner_is_none_and_nonzero_is_kept() {
    let cam = CCamGame::default();
    assert!(cam.owner.is_none());
    let cam = CCamGame {
        owner: Handle32::new(7),
        ..CCamGame::default()
    };
    assert_eq!(cam.owner.map(Handle32::get), Some(7));
}
