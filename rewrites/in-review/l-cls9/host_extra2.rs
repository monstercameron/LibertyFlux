// The goto periodic update.

use lf_peds_tasks::tasks::{GotoPed, GotoPoll, SubVerdict};

fn goto_ped(v: u32) -> Option<Handle32<GotoPed>> {
    Handle32::new(v)
}

struct GotoPollScript {
    probe_answer: u32,
    verdict: SubVerdict,
    probes: Vec<u32>,
    fallbacks: Vec<u32>,
    checks: Vec<(u32, u32)>,
    dispatches: Vec<u32>,
}

impl GotoPoll for GotoPollScript {
    fn probe(&mut self, ped: Option<Handle32<GotoPed>>) -> u32 {
        self.probes.push(Handle32::raw_or_zero(ped));
        self.probe_answer
    }

    fn fallback(&mut self, ped: Option<Handle32<GotoPed>>) {
        self.fallbacks.push(Handle32::raw_or_zero(ped));
    }

    fn check_subtask(
        &mut self,
        sub: Handle32<SubTask>,
        ped: Option<Handle32<GotoPed>>,
    ) -> SubVerdict {
        self.checks.push((sub.get(), Handle32::raw_or_zero(ped)));
        self.verdict
    }

    fn dispatch(&mut self, ped: Option<Handle32<GotoPed>>) {
        self.dispatches.push(Handle32::raw_or_zero(ped));
    }
}

fn goto_poller(probe_answer: u32, verdict: SubVerdict) -> GotoPollScript {
    GotoPollScript {
        probe_answer,
        verdict,
        probes: Vec::new(),
        fallbacks: Vec::new(),
        checks: Vec::new(),
        dispatches: Vec::new(),
    }
}

fn armed_goto() -> GotoTask {
    // Armed with a pending restamp, a long position and a live subtask.
    GotoTask::new(
        subtask(0xBEEF),
        0x1B,
        [3.0, 4.0, 0.0],
        false,
        None,
        5000,
        1000,
        5000,
        true,
        true,
        2.5,
    )
}

#[test]
fn goto_poll_restamps_then_times_the_wait() {
    // Restamp writes the tick and clears itself; the fresh deadline
    // (tick + wait) lies ahead, so the gate still runs.
    let mut task = armed_goto();
    let mut poll = goto_poller(0, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 2000, 0.0, &mut poll), None);
    assert_eq!(task.stamp(), 2000);
    assert!(!task.restamp());
    assert_eq!(poll.probes, vec![0x60]);
    assert_eq!(poll.dispatches, vec![0x60]);
    // A restamp with a zero wait lapses at once: deadline equals tick.
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        1000,
        0,
        true,
        true,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 2000, 0.0, &mut poll), None);
    assert_eq!(task.stamp(), 2000);
    assert!(poll.probes.is_empty());
    // A lapsed wait skips the gate: no probe, straight to the check.
    let mut task = GotoTask::new(
        subtask(0xBEEF),
        0x1B,
        [3.0, 4.0, 0.0],
        false,
        None,
        5000,
        1000,
        5000,
        true,
        false,
        2.5,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 7000, 0.0, &mut poll), None);
    assert_eq!(task.stamp(), 1000);
    assert!(poll.probes.is_empty());
    assert_eq!(poll.checks, vec![(0xBEEF, 0x60)]);
    // At the deadline exactly: skipped (>= is <= here).
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        1000,
        5000,
        true,
        false,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 6000, 0.0, &mut poll), None);
    assert!(poll.probes.is_empty());
    // One tick short: the gate runs and the probe fires.
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        1000,
        5000,
        true,
        false,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 5999, 0.0, &mut poll), subtask(1));
    assert_eq!(poll.probes, vec![0x60]);
    assert_eq!(poll.fallbacks, vec![0x60]);
    assert!(poll.checks.is_empty());
}

#[test]
fn goto_poll_disarmed_runs_the_gate() {
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        777,
        888,
        999,
        false,
        true,
        0.0,
    );
    let mut poll = goto_poller(0, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 2000, 0.0, &mut poll), None);
    // Disarmed: the stamp and restamp bytes are untouched.
    assert_eq!(task.stamp(), 888);
    assert!(task.restamp());
    assert_eq!(poll.probes, vec![0x60]);
}

#[test]
fn goto_poll_refused_check_falls_back() {
    let mut task = armed_goto();
    let mut poll = goto_poller(0, SubVerdict::Refused);
    assert_eq!(
        task.poll(goto_ped(0x60), 2000, 0.0, &mut poll),
        subtask(0xBEEF)
    );
    assert_eq!(poll.checks, vec![(0xBEEF, 0x60)]);
    assert_eq!(poll.fallbacks, vec![0x60]);
    assert!(poll.dispatches.is_empty());
}

#[test]
fn goto_poll_probe_keeps_without_a_subtask() {
    // The probe-keeps path never touches the subtask: no panic.
    let mut task = GotoTask::new(
        None,
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        0,
        0,
        false,
        false,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 0, 0.0, &mut poll), None);
    assert_eq!(poll.fallbacks, vec![0x60]);
    assert!(poll.checks.is_empty());
}

#[test]
#[should_panic(expected = "without a subtask")]
fn goto_poll_panics_past_the_probe_without_a_subtask() {
    let mut task = GotoTask::new(
        None,
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        0,
        0,
        false,
        false,
        0.0,
    );
    let mut poll = goto_poller(0, SubVerdict::Passed);
    let _ = task.poll(goto_ped(0x60), 0, 0.0, &mut poll);
}
