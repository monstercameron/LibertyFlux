use super::task::{
    GAMER_WORDS, GamerListParams, HostSessionGlobals, HostSessionTask, HostSessionWorld,
};
use super::{EntryTag, SessionTag};
use lf_core::Handle32;
use std::collections::VecDeque;

#[derive(Default)]
struct World {
    mode: u32,
    answers: VecDeque<(&'static str, u32)>,
    trailer_words: Vec<u32>,
    calls: Vec<&'static str>,
    probe_out: VecDeque<u32>,
    empty_out: VecDeque<u32>,
    session_flags: u8,
    entry_flags: u8,
    entry: u32,
}

impl World {
    fn answer(&mut self, name: &'static str) -> u32 {
        self.answers
            .iter()
            .position(|(n, _)| *n == name)
            .and_then(|i| self.answers.remove(i))
            .map_or(0, |(_, value)| value)
    }
}

impl HostSessionWorld for World {
    fn session_state(&mut self, _: Option<Handle32<SessionTag>>) -> u32 {
        self.calls.push("session_state");
        self.mode
    }
    fn own_complete(&mut self, _: u32, _: u32) {
        self.calls.push("own_complete");
    }
    fn append_trailer(&mut self, trailer: &mut Vec<u32>, _: u32, _: u32) -> u32 {
        self.calls.push("append_trailer");
        *trailer = self.trailer_words.clone();
        self.answer("append_trailer")
    }
    fn prepare_primary(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("prepare_primary");
    }
    fn prepare_session(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("prepare_session");
    }
    fn open_plain(
        &mut self,
        _: Option<Handle32<SessionTag>>,
        _: u32,
        _: u32,
        _: &[u32; GAMER_WORDS],
        probe: &mut u32,
    ) -> u32 {
        self.calls.push("open_plain");
        if let Some(value) = self.probe_out.pop_front() {
            *probe = value;
        }
        self.answer("open_plain")
    }
    #[allow(clippy::too_many_arguments)]
    fn open_hosted(
        &mut self,
        _: Option<Handle32<SessionTag>>,
        _: u32,
        _: u32,
        _: u32,
        _: u32,
        _: &[u32; GAMER_WORDS],
        _: u32,
        probe: &mut u32,
    ) -> u32 {
        self.calls.push("open_hosted");
        if let Some(value) = self.probe_out.pop_front() {
            *probe = value;
        }
        self.answer("open_hosted")
    }
    fn promote_hosted(&mut self) {
        self.calls.push("promote_hosted");
    }
    fn resolve_host_key(
        &mut self,
        _: Option<Handle32<SessionTag>>,
        _: &[u32; 4],
        empty: &mut u32,
        probe: &mut u32,
    ) -> u32 {
        self.calls.push("resolve_host_key");
        if let Some(value) = self.empty_out.pop_front() {
            *empty = value;
        }
        if let Some(value) = self.probe_out.pop_front() {
            *probe = value;
        }
        self.answer("resolve_host_key")
    }
    fn poll_host(&mut self, _: Option<Handle32<SessionTag>>, probe: &mut u32) -> u32 {
        self.calls.push("poll_host");
        if let Some(value) = self.probe_out.pop_front() {
            *probe = value;
        }
        self.answer("poll_host")
    }
    fn progress_check(&mut self) -> u32 {
        self.calls.push("progress_check");
        self.answer("progress_check")
    }
    fn mirror_session_block(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("mirror_session_block");
    }
    fn session_flag(&mut self, _: Option<Handle32<SessionTag>>) -> u8 {
        self.calls.push("session_flag");
        self.session_flags
    }
    fn set_session_flag(&mut self, _: Option<Handle32<SessionTag>>, value: u8) {
        self.calls.push("set_session_flag");
        self.session_flags = value;
    }
    fn session_up(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("session_up");
    }
    fn notify_host(&mut self, _: u32, _: u32) {
        self.calls.push("notify_host");
    }
    fn submit_descriptor(&mut self, _: Option<Handle32<SessionTag>>, _: u32) {
        self.calls.push("submit_descriptor");
    }
    fn derive_block_a(&mut self) {
        self.calls.push("derive_block_a");
    }
    fn derive_block_b(&mut self) {
        self.calls.push("derive_block_b");
    }
    fn derive_block_c(&mut self, _: u32) {
        self.calls.push("derive_block_c");
    }
    fn derive_block_d(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("derive_block_d");
    }
    fn finish_key_block(&mut self, _: Option<Handle32<SessionTag>>, _: u32, _: &[u32], _: u32) {
        self.calls.push("finish_key_block");
    }
    fn alloc_session_entry(
        &mut self,
        _: Option<Handle32<SessionTag>>,
    ) -> Option<Handle32<EntryTag>> {
        self.calls.push("alloc_session_entry");
        Handle32::new(self.entry)
    }
    fn entry_flags(&mut self, _: Option<Handle32<EntryTag>>) -> u8 {
        self.calls.push("entry_flags");
        self.entry_flags
    }
    fn set_entry_flags(&mut self, _: Option<Handle32<EntryTag>>, value: u8) {
        self.calls.push("set_entry_flags");
        self.entry_flags = value;
    }
    fn verify_cookie(&mut self, _: u32) {
        self.calls.push("verify_cookie");
    }
    fn close_session_aux(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("close_session_aux");
    }
    fn release_session_timer(&mut self, _: Option<Handle32<SessionTag>>) {
        self.calls.push("release_session_timer");
    }
}

fn task() -> HostSessionTask {
    HostSessionTask {
        started: 0,
        countdown: 0,
        session: Handle32::new(7),
        state: 0,
        probe: 0,
        config: 0,
        first_empty: 0,
        list_split: 0,
        first_count: 0,
        second_count: 0,
        gamers: [0; GAMER_WORDS],
        open_extra: 0,
        trailer_size: 0,
        trailer: Vec::new(),
    }
}

#[test]
fn start_preserves_noncanonical_latch_word() {
    let mut host = task();
    host.started = 0xA5;
    host.state = 0x1234;
    let mut world = World {
        mode: 2,
        ..World::default()
    };

    host.start(&mut world);

    assert_eq!(host.started, 0xA5);
    assert_eq!(host.state, 0x1234);
    assert_eq!(world.calls, ["session_state", "own_complete"]);
}

#[test]
fn setup_matches_the_rewrite_count_adjustment_and_owns_trailer() {
    let mut host = task();
    let mut world = World {
        trailer_words: vec![0x1234, 0x5678],
        answers: [("append_trailer", 0xDEAD_BEEF)].into(),
        ..World::default()
    };
    let params = GamerListParams {
        list: 11,
        split: 0,
        first: 36,
        second: 0,
        extra: 13,
        trailer_arg: 17,
        trailer_size: 2,
    };
    let source = core::array::from_fn(|i| i as u32 ^ 0x55AA);

    let answer = host.setup_gamer_data(&mut world, &params, &source);

    assert_eq!(host.first_count, 40);
    assert_eq!(host.second_count, 4);
    assert_eq!(host.first_empty, 0);
    assert_eq!(host.gamers, source);
    assert_eq!(host.trailer, [0x1234, 0x5678]);
    assert_eq!(host.trailer_size, 2);
    assert_eq!(host.config, 11);
    assert_eq!(host.list_split, 0);
    assert_eq!(host.open_extra, 13);
    assert_eq!(answer, 0xDEAD_BE01);
    assert_eq!(world.calls, ["append_trailer"]);
}

#[test]
fn advance_wraps_exact_zero_countdown_and_bounds_the_loop() {
    let mut host = task();
    host.countdown = 1;
    host.state = 99;
    let mut world = World::default();
    let globals = HostSessionGlobals {
        image_base: 0x1000,
        cookie: 5,
        setup_flag: 0,
        setup_key: [0; 4],
    };

    host.advance(&mut world, &globals, 1);

    assert_eq!(host.countdown, u32::MAX);
    assert_eq!(host.state, 99);
    assert_eq!(world.calls, ["progress_check"]);
}

#[test]
fn complete_side_path_releases_pending_auxiliary_before_tail() {
    let mut host = task();
    host.probe = 1;
    let mut world = World {
        mode: 2,
        session_flags: 1,
        ..World::default()
    };
    let globals = HostSessionGlobals {
        image_base: 0x2000,
        cookie: 0xAABB,
        setup_flag: 0,
        setup_key: [0; 4],
    };

    host.complete(&mut world, &globals, 0, 9);

    assert_eq!(host.session, None);
    assert_eq!(world.session_flags, 0);
    assert_eq!(
        world.calls,
        [
            "session_state",
            "poll_host",
            "session_flag",
            "close_session_aux",
            "release_session_timer",
            "session_flag",
            "set_session_flag",
            "notify_host",
            "verify_cookie",
        ]
    );
}
