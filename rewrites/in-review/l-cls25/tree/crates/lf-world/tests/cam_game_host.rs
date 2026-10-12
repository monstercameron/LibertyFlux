//! Host tests for the lifted game camera methods.
//!
//! These run on every host and need no rewrite: they check the edge cases a
//! reader of the lifted code would ask about. The differential proof against
//! the verified 32-bit rewrites is in the `lf-camgame-diff` test crate.

use std::collections::HashMap;

use lf_core::Handle32;
use lf_world::cam_game::registry::{self, State};
use lf_world::cam_game::{
    ACTIVE_MARKER, CCamGame, CamGameWorld, CamGlobal, CamOwnerTag, CamTag, DEFAULT_ZOOM, GateBlock,
    GateTag, IDLE_MARKER, IDLE_MODE, IDLE_TIMER, PoolTag, SingletonField, SingletonTag, TailTag,
    TargetTag, WorkerTag,
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
    inhibit: u8,
    gate: Option<Handle32<GateTag>>,
    /// Whether the second block also gets a gate status object.
    gate_second: bool,
    /// Bytes of the gate status object, by offset; absent offsets read zero.
    gate_bytes: HashMap<u32, u8>,
    target: Option<Handle32<TargetTag>>,
    word_first: u16,
    word_second: u16,
    singleton: bool,
    tail: Option<Handle32<TailTag>>,
    tail_state_value: u8,
    tail_flags_value: u8,
    /// The value written to the matched byte, if the tail step ran.
    matched: Option<u8>,
    singleton_handle: Option<Handle32<SingletonTag>>,
    singleton_active: u8,
    singleton_mode: u32,
    globals: HashMap<CamGlobal, u32>,
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
        self.calls.push(format!(
            "chain_set_flags {}",
            worker.map_or(0, Handle32::get)
        ));
    }

    fn release_worker(&mut self, worker: Option<Handle32<WorkerTag>>) {
        self.calls.push(format!(
            "release_worker {}",
            worker.map_or(0, Handle32::get)
        ));
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

    fn inhibit_flag(&mut self) -> u8 {
        self.inhibit
    }

    fn gate_status(&mut self, block: GateBlock) -> Option<Handle32<GateTag>> {
        self.calls.push(format!("gate_status {block:?}"));
        self.gate
            .filter(|_| block == GateBlock::First || self.gate_second)
    }

    fn gate_byte(&mut self, gate: Handle32<GateTag>, offset: u32) -> u8 {
        let _ = gate;
        self.gate_bytes.get(&offset).copied().unwrap_or(0)
    }

    fn find_target(
        &mut self,
        _cam: &mut CCamGame,
        this: Handle32<CamTag>,
        block: GateBlock,
    ) -> Option<Handle32<TargetTag>> {
        self.calls
            .push(format!("find_target {block:?} {}", this.get()));
        self.target
    }

    fn gate_word(&mut self, block: GateBlock) -> u16 {
        match block {
            GateBlock::First => self.word_first,
            GateBlock::Second => self.word_second,
        }
    }

    fn notify_sink(
        &mut self,
        _cam: &mut CCamGame,
        this: Handle32<CamTag>,
        word: u16,
        block: GateBlock,
    ) {
        self.calls
            .push(format!("notify_sink {block:?} {} {word}", this.get()));
    }

    fn sync_target(&mut self, _cam: &mut CCamGame, this: Handle32<CamTag>) {
        self.calls.push(format!("sync_target {}", this.get()));
    }

    fn singleton_present(&mut self) -> bool {
        self.calls.push("singleton_present".to_string());
        self.singleton
    }

    fn middle_update(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>, step: u32) {
        self.calls.push(format!("middle_update {step}"));
    }

    fn tail(&mut self, _cam: &mut CCamGame, this: Handle32<CamTag>) -> Option<Handle32<TailTag>> {
        self.calls.push(format!("tail {}", this.get()));
        self.tail
    }

    fn tail_state(&mut self, _tail: Handle32<TailTag>) -> u8 {
        self.tail_state_value
    }

    fn tail_flags(&mut self, _tail: Handle32<TailTag>) -> u8 {
        self.tail_flags_value
    }

    fn set_tail_flags(&mut self, _tail: Handle32<TailTag>, flags: u8) {
        self.tail_flags_value = flags;
    }

    fn set_tail_state(&mut self, _tail: Handle32<TailTag>, state: u8) {
        self.tail_state_value = state;
    }

    fn set_matched(&mut self, matched: u8) {
        self.matched = Some(matched);
    }

    fn clear_done(&mut self) {
        self.calls.push("clear_done".to_string());
    }

    fn update_1(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_1".to_string());
    }

    fn update_2(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_2".to_string());
    }

    fn update_3(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_3".to_string());
    }

    fn update_4(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_4".to_string());
    }

    fn watch_init(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("watch_init".to_string());
    }

    fn singleton(&mut self) -> Option<Handle32<SingletonTag>> {
        self.calls.push("singleton".to_string());
        self.singleton_handle
    }

    fn singleton_field(
        &mut self,
        _singleton: Handle32<SingletonTag>,
        field: SingletonField,
    ) -> u32 {
        match field {
            SingletonField::ActiveByte => u32::from(self.singleton_active),
            SingletonField::ModeWord => self.singleton_mode,
        }
    }

    fn blend_gate_one(&mut self) -> u32 {
        self.calls.push("blend_gate_one".to_string());
        0
    }

    fn blend_gate_two(&mut self) -> u32 {
        self.calls.push("blend_gate_two".to_string());
        0
    }

    fn read_global(&mut self, global: CamGlobal) -> u32 {
        self.globals.get(&global).copied().unwrap_or(0)
    }

    fn write_global(&mut self, global: CamGlobal, bits: u32) {
        self.globals.insert(global, bits);
    }

    fn update_9(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_9".to_string());
    }

    fn update_10(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_10".to_string());
    }

    fn update_11(&mut self, _cam: &mut CCamGame, _this: Handle32<CamTag>) {
        self.calls.push("update_11".to_string());
    }
}

#[test]
fn gate_update_without_gates_runs_sync_chain_and_done() {
    let mut cam = CCamGame::default();
    let mut w = Recorder::default();
    cam.gate_update(&mut w, this());
    assert_eq!(
        w.calls,
        vec![
            "gate_status First",
            "sync_target 4096",
            "singleton_present",
            "gate_status Second",
            "tail 4096",
            "clear_done",
        ]
    );
    assert_eq!(
        w.matched, None,
        "no tail, so the matched byte is not written"
    );
}

#[test]
fn gate_update_runs_middle_chain_in_order_only_with_singleton() {
    let mut cam = CCamGame::default();
    let mut w = Recorder {
        singleton: true,
        ..Recorder::default()
    };
    cam.gate_update(&mut w, this());
    let steps: Vec<&String> = w
        .calls
        .iter()
        .filter(|c| c.starts_with("middle_update"))
        .collect();
    assert_eq!(steps.len(), 7);
    assert_eq!(steps.first().map(|s| s.as_str()), Some("middle_update 6"));
    assert_eq!(steps.last().map(|s| s.as_str()), Some("middle_update 12"));
}

#[test]
fn gate_update_tail_clears_busy_bit_and_records_match() {
    let mut cam = CCamGame::default();
    let mut w = Recorder {
        tail: Handle32::new(0x300),
        tail_state_value: 3,
        tail_flags_value: 0xFF,
        ..Recorder::default()
    };
    cam.gate_update(&mut w, this());
    assert_eq!(w.matched, Some(1));
    assert_eq!(w.tail_flags_value, 0xFF & !0x20);
    assert_eq!(w.tail_state_value, 0);
}

#[test]
fn gate_update_tail_leaves_flags_alone_when_not_busy() {
    let mut cam = CCamGame::default();
    let mut w = Recorder {
        tail: Handle32::new(0x300),
        tail_state_value: 2,
        tail_flags_value: 0xDF,
        ..Recorder::default()
    };
    cam.gate_update(&mut w, this());
    assert_eq!(w.matched, Some(0));
    assert_eq!(
        w.tail_flags_value, 0xDF,
        "busy bit already clear: no write needed"
    );
    assert_eq!(w.tail_state_value, 0);
}

#[test]
fn gate_block_stops_before_the_target_when_pair_test_is_low() {
    // Verdict set (selector nonzero, first folded pair high, second low),
    // but the pair test is low: the target is never asked for.
    let mut cam = CCamGame::default();
    let mut w = Recorder {
        gate: Handle32::new(0x500),
        gate_bytes: [
            (0x328D, 1),
            (0x269C, 0x00),
            (0x269E, 0x80),
            (0x269F, 0x00),
            (0x29BC, 0x00),
            (0x29BE, 0x00),
        ]
        .into_iter()
        .collect(),
        ..Recorder::default()
    };
    cam.gate_update(&mut w, this());
    assert!(!w.calls.iter().any(|c| c.starts_with("find_target")));
}

#[test]
fn gate_block_with_verdict_and_pair_asks_target_and_notifies_with_block_word() {
    let mut cam = CCamGame::default();
    let mut w = Recorder {
        gate: Handle32::new(0x500),
        gate_bytes: [
            (0x328D, 1),
            (0x269C, 0x00),
            (0x269E, 0x80),
            (0x269F, 0x00),
            (0x29BC, 0x00),
            (0x29BE, 0xFF),
        ]
        .into_iter()
        .collect(),
        target: Handle32::new(0x600),
        word_first: 0x1111,
        word_second: 0x2222,
        ..Recorder::default()
    };
    cam.gate_update(&mut w, this());
    assert!(w.calls.iter().any(|c| c == "notify_sink First 4096 4369"));
}

#[test]
fn inhibit_flag_blocks_the_zero_selector_verdict() {
    // Selector zero: the verdict needs the inhibit flag clear.
    let mut cam = CCamGame::default();
    let bytes: std::collections::HashMap<u32, u8> = [
        (0x328D, 0),
        (0x269C, 0x00),
        (0x269E, 0x00),
        (0x269F, 0x80),
        (0x29BC, 0x00),
        (0x29BE, 0xFF),
    ]
    .into_iter()
    .collect();
    let mut open = Recorder {
        gate: Handle32::new(0x500),
        gate_bytes: bytes.clone(),
        target: Handle32::new(0x600),
        inhibit: 0,
        ..Recorder::default()
    };
    cam.gate_update(&mut open, this());
    assert!(open.calls.iter().any(|c| c.starts_with("find_target")));

    let mut shut = Recorder {
        gate: Handle32::new(0x500),
        gate_bytes: bytes,
        target: Handle32::new(0x600),
        inhibit: 1,
        ..Recorder::default()
    };
    cam.gate_update(&mut shut, this());
    assert!(!shut.calls.iter().any(|c| c.starts_with("find_target")));
}

fn this() -> Handle32<CamTag> {
    Handle32::new(0x1000).expect("non-zero identity")
}

#[test]
fn registry_counts_three_proven_methods() {
    assert_eq!(registry::proven_count(), 5);
    let missing = registry::ROWS
        .iter()
        .filter(|r| r.state == State::Missing)
        .count();
    assert_eq!(missing, 2);
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
        vec!["destroy_body 4096", "pool_manager", "pool_release 119 4096"]
    );
}

#[test]
fn destructor_passes_zero_manager_as_none() {
    let mut cam = CCamGame::default();
    let mut w = Recorder::default();
    cam.deleting_destructor(&mut w, this(), 1);
    assert_eq!(
        w.calls.last().map(String::as_str),
        Some("pool_release 0 4096")
    );
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
    assert_eq!(
        (cam.pending_1c8, cam.pending_1cc, cam.pending_1d0),
        (0, 0, 0)
    );
    assert_eq!(cam.mode, 9, "restart leaves the mode word alone");
    assert_eq!(
        w.calls[1], "create_worker 85 1 0 4096",
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
    // Bits 1 to 3 are cleared before the reset step, bit 0 after it.
    assert_eq!(cam.flags_1c5, (0xFF & 0xF1) & 0xFE);
    assert_eq!(cam.default_word, 0xABCD);
    assert_eq!(cam.mode, IDLE_MODE);
    assert_eq!(cam.timer, IDLE_TIMER);
    assert_eq!(cam.zoom_bits, DEFAULT_ZOOM.to_bits());
    assert_eq!(cam.zoom_bits, 0x3F80_0000);
    assert_eq!(
        w.calls,
        vec!["shared_default_word", "reset_state_block 4096",]
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
