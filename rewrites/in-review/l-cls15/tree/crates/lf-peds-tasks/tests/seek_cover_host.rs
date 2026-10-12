//! Host tests for the lifted seek-cover and shooting methods. These run on
//! every host: they use a recording collaborator, not the 32-bit rewrites
//! (the differential tests in the test crate do that).

use lf_core::Handle32;
use lf_peds_tasks::seek_cover::{
    CoverRowArgs, CoverRoute, CoverSubTask, CoverWorker, FullArgs, OwnedSlot, PursuitNode,
    PursuitTarget, RouteArgs, RouteExtra, SeekCoverTask, SeekCoverWorld, ShootingTask,
    ShootingWorld, ShortArgs, ShotNodeArgs, ShotSubject, SmallNode, Stamp, TaskNode, InitArg,
};
use lf_peds_tasks::seek_cover::task::fold_latch_bit;

/// Recording collaborator for both classes. Answers are set per test; the
/// log records each collaborator call by name, in order.
#[derive(Default)]
struct Mock {
    log: Vec<&'static str>,
    worker: Option<u32>,
    sub: Option<u32>,
    node: Option<u32>,
    sub_latch: u8,
    row_flags: u8,
    query: [u32; 4],
    dtor_answer: u32,
    tags: Vec<u32>,
    subject_flags: u32,
    scale: u32,
    target_flags: u8,
    accept: bool,
    chain_next_ok: bool,
    last_pursuit: Option<PursuitNode>,
    last_small: Option<SmallNode>,
    last_shot: Option<ShotNodeArgs>,
    last_route: Option<RouteArgs>,
    tag_at: usize,
}

impl SeekCoverWorld for Mock {
    fn set_stamp(&mut self, _this: &mut SeekCoverTask, _stamp: Stamp) {
        self.log.push("stamp");
    }
    fn base_construct(&mut self, _this: &mut SeekCoverTask) {
        self.log.push("base");
    }
    fn init_member_60(&mut self, _this: &mut SeekCoverTask) {
        self.log.push("init60");
    }
    fn init_member_6c(&mut self, _this: &mut SeekCoverTask) {
        self.log.push("init6c");
    }
    fn reinit_shared(&mut self, _this: &mut SeekCoverTask) {
        self.log.push("reinit");
    }
    fn cover_row_flags(&mut self, _index: u32) -> u8 {
        self.row_flags
    }
    fn cover_row_query(&mut self, _this: &mut SeekCoverTask, _index: u32) -> [u32; 4] {
        self.log.push("query");
        self.query
    }
    fn release_owned(&mut self, _this: &mut SeekCoverTask, slot: OwnedSlot, _owned: u32) {
        self.log.push(match slot {
            OwnedSlot::Pos => "release_pos",
            OwnedSlot::Link => "release_link",
        });
    }
    fn release_member_6c(&mut self, _this: &mut SeekCoverTask) {
        self.log.push("rel6c");
    }
    fn release_member_60(&mut self, _this: &mut SeekCoverTask) {
        self.log.push("rel60");
    }
    fn base_destruct(&mut self, _this: &mut SeekCoverTask) -> u32 {
        self.log.push("base_dtor");
        self.dtor_answer
    }
    fn fetch_worker(&mut self, _this: &mut SeekCoverTask) -> Option<Handle32<CoverWorker>> {
        self.log.push("fetch");
        self.worker.and_then(Handle32::new)
    }
    fn run_route(
        &mut self,
        _this: &mut SeekCoverTask,
        route: CoverRoute,
        _worker: Handle32<CoverWorker>,
        args: &RouteArgs,
    ) -> Option<Handle32<CoverSubTask>> {
        self.log.push(match route {
            CoverRoute::Zero => "route0",
            CoverRoute::One => "route1",
            CoverRoute::Two => "route2",
        });
        self.last_route = Some(*args);
        self.sub.and_then(Handle32::new)
    }
    fn latch_of(&mut self, _sub: Handle32<CoverSubTask>) -> u8 {
        self.log.push("latch_of");
        self.sub_latch
    }
    fn set_latch_of(&mut self, _sub: Handle32<CoverSubTask>, latch: u8) {
        self.log.push("set_latch_of");
        self.sub_latch = latch;
    }
    fn allocate_node(&mut self) -> Option<Handle32<TaskNode>> {
        self.log.push("alloc");
        self.node.and_then(Handle32::new)
    }
    fn construct_node(&mut self, _node: Handle32<TaskNode>) {
        self.log.push("node_ctor");
    }
    fn store_small_node(&mut self, _node: Handle32<TaskNode>, fields: &SmallNode) {
        self.log.push("store_small");
        self.last_small = Some(*fields);
    }
}

impl ShootingWorld for Mock {
    fn base_destruct(&mut self, _this: &mut ShootingTask) -> u32 {
        self.log.push("base_dtor");
        0
    }
    fn free_to_pool(&mut self, _this: &mut ShootingTask) {
        self.log.push("free");
    }
    fn fetch_node(&mut self) -> Option<Handle32<TaskNode>> {
        self.log.push("alloc");
        self.node.and_then(Handle32::new)
    }
    fn construct_node(&mut self, _node: Handle32<TaskNode>) {
        self.log.push("node_ctor");
    }
    fn store_pursuit_node(&mut self, _node: Handle32<TaskNode>, fields: &PursuitNode) {
        self.log.push("store_pursuit");
        self.last_pursuit = Some(*fields);
    }
    fn construct_shot_node(&mut self, _node: Handle32<TaskNode>, args: &ShotNodeArgs) -> u32 {
        self.log.push("shot_ctor");
        self.last_shot = Some(*args);
        7
    }
    fn target_flags(&mut self, _target: Handle32<PursuitTarget>) -> u8 {
        self.target_flags
    }
    fn solve(&mut self, _this: &mut ShootingTask, _target: Handle32<PursuitTarget>) {
        self.log.push("solve");
    }
    fn dispatch_virtual(&mut self, _this: &mut ShootingTask, _arg: u32) -> u32 {
        self.log.push("dispatch");
        0x55
    }
    fn scale_factor(&mut self) -> u32 {
        self.scale
    }
    fn subject_flags(&mut self, _subject: Handle32<ShotSubject>) -> u32 {
        self.subject_flags
    }
    fn set_subject_flags(&mut self, _subject: Handle32<ShotSubject>, flags: u32) {
        self.log.push("set_subject");
        self.subject_flags = flags;
    }
    fn ask_subject(&mut self, _subject: Handle32<ShotSubject>, _a0: Handle32<InitArg>, _a1: u32, _a2: u32) -> u32 {
        self.log.push("ask");
        if self.accept { 1 } else { 0 }
    }
    fn init_member_60(&mut self, _this: &mut ShootingTask) {
        self.log.push("init60");
    }
    fn init_member_6c(&mut self, _this: &mut ShootingTask) {
        self.log.push("init6c");
    }
    fn finish_arg(&mut self, _a0: Handle32<InitArg>) {
        self.log.push("finish");
    }
    fn chain_tag(&mut self, _link: Handle32<ShotSubject>) -> u32 {
        self.log.push("tag");
        let t = self.tags.get(self.tag_at).copied().unwrap_or(0);
        self.tag_at += 1;
        t
    }
    fn chain_next(&mut self, _link: Handle32<ShotSubject>) -> Option<Handle32<ShotSubject>> {
        if self.chain_next_ok { Handle32::new(0x100) } else { None }
    }
}

fn subject() -> Option<Handle32<ShotSubject>> {
    Handle32::new(0x200)
}

// ----------------------------------------------------------------------------
// Seek cover.
// ----------------------------------------------------------------------------

#[test]
fn lane_is_the_sign_extended_mode_bits() {
    // Bits three to five hold the mode; the lane is that field, sign extended.
    assert_eq!(SeekCoverTask::lane(0b000_000), 0);
    assert_eq!(SeekCoverTask::lane(0b011_000), 3);
    assert_eq!(SeekCoverTask::lane(0b100_000), (-4i32) as u32);
    assert_eq!(SeekCoverTask::lane(0b101_000), (-3i32) as u32);
    // Bits below three do not reach the lane.
    assert_eq!(SeekCoverTask::lane(0b000_111), 0);
    assert_eq!(SeekCoverTask::lane(0xFFFF_FFFF), (-1i32) as u32);
}

#[test]
fn routes_are_the_low_three_bits_and_three_or_more_is_none() {
    assert_eq!(CoverRoute::from_flags(0), Some(CoverRoute::Zero));
    assert_eq!(CoverRoute::from_flags(0xFFFF_FFF9), Some(CoverRoute::One));
    assert_eq!(CoverRoute::from_flags(0x0000_0012), Some(CoverRoute::Two));
    for low in 3..8 {
        assert_eq!(CoverRoute::from_flags(0xFFFF_FFF0 | low), None);
    }
}

#[test]
fn latch_fold_takes_only_bit_zero_of_the_source() {
    assert_eq!(fold_latch_bit(0b10, 0b01), 0b11);
    assert_eq!(fold_latch_bit(0b11, 0b10), 0b10);
    assert_eq!(fold_latch_bit(0b1010_0000, 0b0000_0100), 0b1010_0000);
    assert_eq!(fold_latch_bit(0b1010_0001, 0b0000_0001), 0b1010_0001);
}

#[test]
fn full_constructor_keeps_the_slot_word_it_does_not_copy() {
    let mut t = SeekCoverTask { slot: [1, 2, 3, 0xDEAD], flags: 0xFFFF_FFFF, ..Default::default() };
    let mut m = Mock::default();
    t.construct_full(
        &mut m,
        &FullArgs { pos: 9, dir_src: &[4, 5, 6], mode_byte: 1, latch_src: 1, slot_src: &[7, 8, 9], weight: 0x3F80_0000, mode: 5 },
    );
    assert_eq!(t.slot, [7, 8, 9, 0xDEAD]);
    assert_eq!(t.index, u32::MAX);
    assert_eq!(t.mark, u32::MAX);
    assert_eq!(t.flags & 7, 1, "the full constructor sets bit zero and clears bits one and two");
    assert_eq!((t.flags >> 3) & 7, 5, "the mode goes in bits three to five");
    assert_eq!(m.log, ["base", "stamp", "init60", "init6c", "reinit"]);
}

#[test]
fn row_constructor_with_an_empty_row_skips_the_query() {
    let mut t = SeekCoverTask { flags: 0xFFFF_FFFF, ..Default::default() };
    let mut m = Mock { row_flags: 0, ..Default::default() };
    t.construct_cover_row(
        &mut m,
        &CoverRowArgs { pos: 0, dir_src: &[0; 3], flags0: 0, flags1: 0, index: 12, weight: 0, mode: 0 },
    );
    assert_eq!(m.log, ["base", "stamp", "init60", "init6c", "reinit"]);
    assert_eq!(t.index, u32::MAX, "an empty row clears the index");
    assert_eq!(t.slot, [0, 0, 0, 0], "the slot words 0..3 are cleared on this path");
}

#[test]
fn row_constructor_with_a_populated_row_takes_the_query_answer() {
    let mut t = SeekCoverTask::default();
    let mut m = Mock { row_flags: 4, query: [11, 22, 33, 44], ..Default::default() };
    t.construct_cover_row(
        &mut m,
        &CoverRowArgs { pos: 0, dir_src: &[0; 3], flags0: 0, flags1: 0, index: 3, weight: 0, mode: 0 },
    );
    assert_eq!(t.slot, [11, 22, 33, 44]);
    assert_eq!(t.index, 3);
    assert!(m.log.contains(&"query"));
}

#[test]
fn short_constructor_zeroes_the_second_vector_and_keeps_bits_six_and_up() {
    let mut t = SeekCoverTask { flags: 0xFFFF_FFFF, slot: [5, 5, 5, 5], ..Default::default() };
    let mut m = Mock::default();
    t.construct_short(
        &mut m,
        &ShortArgs { pos: 1, dir_src: &[2, 3, 4], mode_byte: 6, latch_src: 0, weight: 0, mode: 0 },
    );
    assert_eq!(t.slot, [0, 0, 0, 5]);
    assert_eq!(t.flags & 0x3F, 0, "bits zero to five are cleared, the mode is zero");
    assert_eq!(t.flags & 0xFFFF_FFC0, 0xFFFF_FFC0);
    assert_eq!(m.log, ["base", "stamp", "init60", "init6c", "reinit"]);
}

#[test]
fn destructor_with_no_owned_word_releases_only_the_members() {
    let mut t = SeekCoverTask::default();
    let mut m = Mock { dtor_answer: 0xABC, ..Default::default() };
    assert_eq!(t.destruct(&mut m), 0xABC);
    assert_eq!(m.log, ["stamp", "rel6c", "rel60", "base_dtor"]);
}

#[test]
fn destructor_releases_and_clears_both_owned_words() {
    let mut t = SeekCoverTask { pos: 5, link: 6, ..Default::default() };
    let mut m = Mock::default();
    t.destruct(&mut m);
    assert_eq!((t.pos, t.link), (0, 0));
    assert_eq!(m.log, ["stamp", "release_pos", "release_link", "rel6c", "rel60", "base_dtor"]);
}

#[test]
fn router_without_a_worker_or_a_subtask_returns_none_and_touches_no_latch() {
    let mut t = SeekCoverTask { flags: 1, ..Default::default() };
    let mut m = Mock { worker: None, ..Default::default() };
    assert!(t.route_to_sub_task(&mut m).is_none());
    assert_eq!(m.log, ["fetch"]);

    let mut m = Mock { worker: Some(0x40), sub: None, ..Default::default() };
    assert!(t.route_to_sub_task(&mut m).is_none());
    assert_eq!(m.log, ["fetch", "route1"]);
}

#[test]
fn router_on_a_route_it_does_not_run_calls_nothing() {
    let mut t = SeekCoverTask { flags: 5, ..Default::default() };
    let mut m = Mock { worker: Some(0x40), sub: Some(0x80), ..Default::default() };
    assert!(t.route_to_sub_task(&mut m).is_none());
    assert!(m.log.is_empty());
}

#[test]
fn router_copies_bit_three_of_the_latch_to_the_subtask_and_keeps_the_rest() {
    let mut t = SeekCoverTask { flags: 0b10_0000 | 2, latch: 0b1000, pos: 3, mode_byte: 4, index: 77, ..Default::default() };
    let mut m = Mock { worker: Some(0x40), sub: Some(0x80), sub_latch: 0b0001_0001, ..Default::default() };
    assert!(t.route_to_sub_task(&mut m).is_some());
    assert_eq!(m.sub_latch, 0b0001_1001, "bit three is set, bit four kept");
    let args = m.last_route.expect("the route ran");
    assert_eq!(args.extra, Some(RouteExtra::Index(77)));
    assert_eq!(args.lane, (-4i32) as u32);
    assert_eq!(args.mode, 4);
}

#[test]
fn allocator_with_an_empty_pool_returns_none() {
    let mut m = Mock { node: None, ..Default::default() };
    assert!(SeekCoverTask::allocate_node_for_arg(&mut m, 9).is_none());
    assert_eq!(m.log, ["alloc"]);
}

#[test]
fn allocator_stores_the_argument_in_the_argument_word() {
    let mut m = Mock { node: Some(0x300), ..Default::default() };
    assert!(SeekCoverTask::allocate_node_for_arg(&mut m, 0xFFFF_FFFE).is_some());
    assert_eq!(m.last_small, Some(SmallNode { arg: 0xFFFF_FFFE, flag: 0, tail: 0 }));
    assert_eq!(m.log, ["alloc", "node_ctor", "store_small"]);
}

// ----------------------------------------------------------------------------
// Shooting.
// ----------------------------------------------------------------------------

#[test]
fn scalar_delete_frees_only_when_the_low_bit_is_set() {
    let mut t = ShootingTask::default();
    let mut m = Mock::default();
    t.scalar_delete(&mut m, 2);
    assert_eq!(m.log, ["base_dtor"]);
    let mut m = Mock::default();
    t.scalar_delete(&mut m, 3);
    assert_eq!(m.log, ["base_dtor", "free"]);
}

#[test]
fn pursuit_with_the_node_bit_and_an_empty_pool_returns_zero() {
    let mut t = ShootingTask::default();
    let mut m = Mock { target_flags: 4, node: None, ..Default::default() };
    assert_eq!(t.pursuit_action(&mut m, Handle32::new(0x500).unwrap()), 0);
    assert_eq!(m.log, ["alloc"]);
}

#[test]
fn pursuit_with_the_node_bit_builds_a_pursuit_node() {
    let mut t = ShootingTask::default();
    let mut m = Mock { target_flags: 4, node: Some(0x300), ..Default::default() };
    assert_eq!(t.pursuit_action(&mut m, Handle32::new(0x500).unwrap()), 0x300);
    assert_eq!(m.last_pursuit, Some(PursuitNode { arg: 0, flag16: 1, flag8: 1 }));
    assert_eq!(m.log, ["alloc", "node_ctor", "store_pursuit"]);
}

#[test]
fn pursuit_without_the_node_bit_solves_then_dispatches() {
    let mut t = ShootingTask::default();
    let mut m = Mock { target_flags: 0, ..Default::default() };
    assert_eq!(t.pursuit_action(&mut m, Handle32::new(0x500).unwrap()), 0x55);
    assert_eq!(m.log, ["solve", "dispatch"]);
}

#[test]
fn firing_node_scales_a_negative_count_by_the_factor() {
    let t = ShootingTask { base: SeekCoverTask { pos: 42, ..Default::default() }, ..Default::default() };
    let mut m = Mock { node: Some(0x300), scale: 2.0f32.to_bits(), ..Default::default() };
    assert_eq!(t.shot_node(&mut m, (-3i32) as u32), 7);
    let a = m.last_shot.expect("the node was built");
    assert_eq!(f32::from_bits(a.scaled_bits), -6.0);
    assert_eq!((a.kind, a.pos, a.tail), (0xF, 42, 0xFFFF_FFFF));
}

#[test]
fn firing_node_with_an_empty_pool_returns_zero() {
    let t = ShootingTask::default();
    let mut m = Mock { node: None, ..Default::default() };
    assert_eq!(t.shot_node(&mut m, 1), 0);
    assert!(m.last_shot.is_none());
}

#[test]
fn subject_that_declines_fails_before_any_member_runs() {
    let mut t = ShootingTask { subject: subject(), ..Default::default() };
    let mut m = Mock { accept: false, subject_flags: 0, ..Default::default() };
    assert_eq!(t.initialise_against_subject(&mut m, Handle32::new(0x600).unwrap(), 1, 2), 0);
    assert_eq!(m.log, ["ask"]);
}

#[test]
fn subject_that_accepts_is_marked_prepared_and_initialised() {
    let mut t = ShootingTask { subject: subject(), ..Default::default() };
    let mut m = Mock { accept: true, subject_flags: 0, ..Default::default() };
    assert_eq!(t.initialise_against_subject(&mut m, Handle32::new(0x600).unwrap(), 1, 2), 1);
    assert_eq!(m.subject_flags, 2);
    assert_eq!(m.log, ["ask", "set_subject", "init60", "init6c", "finish"]);
}

#[test]
fn ready_subject_is_not_asked_again() {
    let mut t = ShootingTask { subject: subject(), ..Default::default() };
    let mut m = Mock { subject_flags: 1, ..Default::default() };
    assert_eq!(t.initialise_against_subject(&mut m, Handle32::new(0x600).unwrap(), 1, 2), 1);
    assert_eq!(m.log, ["init60", "init6c", "finish"]);
}

#[test]
fn chain_needs_all_three_tags_and_a_set_owned_word() {
    let mut t = ShootingTask { subject: subject(), ..Default::default() };
    t.base.link = 1;
    let mut m = Mock { tags: vec![0x11D, 0x41A, 0x76D], chain_next_ok: true, ..Default::default() };
    assert_eq!(t.has_full_chain(&mut m), 1);

    t.base.link = 0;
    let mut m = Mock { tags: vec![0x11D, 0x41A, 0x76D], chain_next_ok: true, ..Default::default() };
    assert_eq!(t.has_full_chain(&mut m), 0, "the owned word must be set");

    t.base.link = 1;
    let mut m = Mock { tags: vec![0x11D, 0x41B, 0x76D], chain_next_ok: true, ..Default::default() };
    assert_eq!(t.has_full_chain(&mut m), 0);
    assert_eq!(m.log, ["tag", "tag"], "the walk stops at the wrong tag");
}

#[test]
fn chain_with_a_null_next_link_stops_at_the_null() {
    let t = ShootingTask { subject: subject(), ..Default::default() };
    let mut m = Mock { tags: vec![0x11D], chain_next_ok: false, ..Default::default() };
    assert_eq!(t.has_full_chain(&mut m), 0);
    assert_eq!(m.log, ["tag"]);
}
