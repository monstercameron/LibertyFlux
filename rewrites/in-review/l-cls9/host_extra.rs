// The flee task.

use lf_peds_tasks::tasks::{
    FleeEntity, FleePed, FleePoll, FleeSpawn, FleeTask, GotoEntity, GotoPool, GotoTask, SubTask,
};

fn flee() -> FleeTask {
    FleeTask::new(
        Handle32::new(0xBEEF),
        0,
        0x1B,
        [3.0, 4.0, 0.0],
        false,
        None,
        0x5A,
    )
}

fn entity(v: u32) -> Option<Handle32<FleeEntity>> {
    Handle32::new(v)
}

fn subtask(v: u32) -> Option<Handle32<SubTask>> {
    Handle32::new(v)
}

fn flee_ped(v: u32) -> Option<Handle32<FleePed>> {
    Handle32::new(v)
}

struct FleeSpawnScript {
    alloc_answer: u32,
    entity_answer: u32,
    vector_answer: u32,
    allocs: Vec<u32>,
    entities: Vec<(u32, u32, u32)>,
    vectors: Vec<(u32, [u32; 3], u32)>,
}

impl FleeSpawn for FleeSpawnScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct_entity(
        &mut self,
        block: Handle32<UninitTask>,
        entity: Handle32<FleeEntity>,
        kind: u32,
    ) -> Option<Handle32<SubTask>> {
        self.entities.push((block.get(), entity.get(), kind));
        Handle32::new(self.entity_answer)
    }

    fn construct_vector(
        &mut self,
        block: Handle32<UninitTask>,
        pos: [f32; 3],
        kind: u32,
    ) -> Option<Handle32<SubTask>> {
        self.vectors.push((
            block.get(),
            [pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits()],
            kind,
        ));
        Handle32::new(self.vector_answer)
    }
}

fn spawner() -> FleeSpawnScript {
    FleeSpawnScript {
        alloc_answer: 0x1000,
        entity_answer: 0x2000,
        vector_answer: 0x3000,
        allocs: Vec::new(),
        entities: Vec::new(),
        vectors: Vec::new(),
    }
}

#[test]
fn flee_round_trips_its_words() {
    let task = flee();
    assert_eq!(task.subtask(), subtask(0xBEEF));
    assert_eq!(task.marks(), 0);
    assert_eq!(task.kind(), 0x1B);
    assert_eq!(task.pos(), [3.0, 4.0, 0.0]);
    assert!(!task.flag());
    assert_eq!(task.mode(), None);
    assert_eq!(task.state(), 0x5A);
}

#[test]
fn flee_gate_flag_paths_ignore_the_position() {
    // Set flag with a set mode fires at once: entity form.
    let task = FleeTask::new(subtask(1), 0, 7, [0.0, 0.0, 0.0], true, entity(0xE0), 0);
    let mut spawn = spawner();
    assert_eq!(
        task.spawn(f32::MAX, manager(9), &mut spawn),
        subtask(0x2000)
    );
    assert_eq!(spawn.allocs, vec![9]);
    assert_eq!(spawn.entities, vec![(0x1000, 0xE0, 7)]);
    assert!(spawn.vectors.is_empty());
    // Mixed pairs never fire, whatever the position.
    for (flag, mode) in [(true, None), (false, entity(0xE0))] {
        let task = FleeTask::new(
            subtask(1),
            0,
            7,
            [1.0e30, 1.0e30, 1.0e30],
            flag,
            mode,
            0,
        );
        let mut spawn = spawner();
        assert_eq!(task.spawn(0.0, manager(9), &mut spawn), None);
        assert!(spawn.allocs.is_empty());
    }
}

#[test]
fn flee_gate_measures_length_strictly() {
    // Squared length exactly 25: fires above, rests at or below.
    let task = FleeTask::new(subtask(1), 0, 7, [3.0, 4.0, 0.0], false, None, 0);
    let mut spawn = spawner();
    assert_eq!(
        task.spawn(24.0, manager(9), &mut spawn),
        subtask(0x3000)
    );
    assert_eq!(spawn.vectors.len(), 1);
    assert_eq!(
        spawn.vectors[0].1,
        [3.0f32.to_bits(), 4.0f32.to_bits(), 0.0f32.to_bits()]
    );
    assert_eq!(spawn.vectors[0].2, 7);
    for threshold in [25.0, 26.0, f32::INFINITY] {
        let mut spawn = spawner();
        assert_eq!(task.spawn(threshold, manager(9), &mut spawn), None);
        assert!(spawn.allocs.is_empty());
    }
    // A NaN component never exceeds the threshold.
    let task = FleeTask::new(subtask(1), 0, 7, [f32::NAN, 0.0, 0.0], false, None, 0);
    let mut spawn = spawner();
    assert_eq!(task.spawn(0.0, manager(9), &mut spawn), None);
}

#[test]
fn flee_spawn_answers_null_without_a_block() {
    let task = flee();
    let mut spawn = spawner();
    spawn.alloc_answer = 0;
    assert_eq!(task.spawn(0.0, manager(9), &mut spawn), None);
    assert_eq!(spawn.allocs, vec![9]);
    assert!(spawn.entities.is_empty());
    assert!(spawn.vectors.is_empty());
}

struct FleePollScript {
    probe_answer: u32,
    check_answer: u32,
    probes: Vec<u32>,
    checks: Vec<u32>,
    dispatches: Vec<u32>,
}

impl FleePoll for FleePollScript {
    fn probe(&mut self, ped: Option<Handle32<FleePed>>) -> u32 {
        self.probes.push(Handle32::raw_or_zero(ped));
        self.probe_answer
    }

    fn check(&mut self, ped: Option<Handle32<FleePed>>) -> u32 {
        self.checks.push(Handle32::raw_or_zero(ped));
        self.check_answer
    }

    fn dispatch(&mut self, ped: Option<Handle32<FleePed>>) {
        self.dispatches.push(Handle32::raw_or_zero(ped));
    }
}

fn poller(probe_answer: u32, check_answer: u32) -> FleePollScript {
    FleePollScript {
        probe_answer,
        check_answer,
        probes: Vec::new(),
        checks: Vec::new(),
        dispatches: Vec::new(),
    }
}

#[test]
fn flee_poll_keeps_on_a_fired_probe() {
    // Live gate (long position, clear flag and mode) + set low byte.
    let mut task = flee();
    let mut poll = poller(1, 0);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), subtask(0xBEEF));
    assert_eq!(poll.probes, vec![0x60]);
    assert!(poll.checks.is_empty());
    assert!(poll.dispatches.is_empty());
    assert_eq!(task.marks(), 0);
    // Only the low byte decides: 0x100 refuses.
    let mut task = flee();
    let mut poll = poller(0x100, 0);
    assert_eq!(
        task.poll(flee_ped(0x60), 0.0, &mut poll),
        subtask(0xBEEF)
    );
    assert_eq!(poll.checks, vec![0x60]);
}

#[test]
fn flee_poll_checks_then_dispatches() {
    // Refused check keeps the subtask without dispatching.
    let mut task = flee();
    let mut poll = poller(0, 0);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), subtask(0xBEEF));
    assert_eq!(task.marks(), 0);
    assert!(poll.dispatches.is_empty());
    // Passed check marks and dispatches, answering null.
    let mut task = flee();
    let mut poll = poller(0, 0x1FF);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), None);
    assert_eq!(task.marks(), 2);
    assert_eq!(poll.dispatches, vec![0x60]);
    // The gate bit skips the check but still dispatches.
    let mut task = FleeTask::new(subtask(1), 1, 7, [0.0, 0.0, 0.0], false, None, 0);
    let mut poll = poller(0, 0xFF);
    assert_eq!(task.poll(flee_ped(0x60), 99.0, &mut poll), None);
    assert!(poll.probes.is_empty());
    assert!(poll.checks.is_empty());
    assert_eq!(poll.dispatches, vec![0x60]);
    assert_eq!(task.marks(), 1);
}

#[test]
fn flee_poll_never_probes_a_dead_gate() {
    let mut task = FleeTask::new(subtask(1), 0, 7, [0.0, 0.0, 0.0], false, None, 0);
    let mut poll = poller(0xFF, 0xFF);
    assert_eq!(task.poll(flee_ped(0x60), 99.0, &mut poll), None);
    assert!(poll.probes.is_empty());
    assert_eq!(poll.checks, vec![0x60]);
}

// The goto task.

fn goto_task() -> GotoTask {
    GotoTask::new(
        subtask(0xBEEF),
        0x1B,
        [1.0, 2.0, 3.0],
        true,
        Handle32::new(0xE0),
        5000,
        1000,
        5000,
        true,
        false,
        2.5,
    )
}

fn goto_entity(v: u32) -> Option<Handle32<GotoEntity>> {
    Handle32::new(v)
}

struct GotoPoolScript {
    alloc_answer: u32,
    construct_answer: u32,
    allocs: Vec<u32>,
    constructs: Vec<(u32, u32)>,
}

impl GotoPool for GotoPoolScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        member: u32,
    ) -> Option<Handle32<GotoTask>> {
        self.constructs.push((block.get(), member));
        Handle32::new(self.construct_answer)
    }
}

#[test]
fn goto_round_trips_its_words() {
    let task = goto_task();
    assert_eq!(task.subtask(), subtask(0xBEEF));
    assert_eq!(task.kind(), 0x1B);
    assert_eq!(task.pos(), [1.0, 2.0, 3.0]);
    assert!(task.flag());
    assert_eq!(task.mode(), goto_entity(0xE0));
    assert_eq!(task.wait(), 5000);
    assert_eq!(task.stamp(), 1000);
    assert_eq!(task.wait_copy(), 5000);
    assert!(task.armed());
    assert!(!task.restamp());
    assert_eq!(task.speed(), 2.5);
}

#[test]
fn goto_clone_builds_from_the_member() {
    let task = goto_task();
    let mut pool = GotoPoolScript {
        alloc_answer: 0x1000,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(
        task.clone_task(manager(9), &mut pool),
        Handle32::new(0x2000)
    );
    assert_eq!(pool.allocs, vec![9]);
    assert_eq!(pool.constructs, vec![(0x1000, 0x1B)]);
}

#[test]
fn goto_clone_answers_null_without_constructing() {
    let task = goto_task();
    let mut pool = GotoPoolScript {
        alloc_answer: 0,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(task.clone_task(manager(9), &mut pool), None);
    assert!(pool.constructs.is_empty());
}
