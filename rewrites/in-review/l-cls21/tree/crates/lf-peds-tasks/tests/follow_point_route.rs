//! Host tests of the lifted follow-point route task: the edge cases a reader
//! of the code would ask about. These run on every host; the comparison
//! against the verified rewrites is in the `lf-peds-follow-diff` crate.

use lf_peds_tasks::follow_point_route::host::{PedView, RouteTaskHost};
use lf_peds_tasks::follow_point_route::registry;
use lf_peds_tasks::follow_point_route::route::{RoutePoint, RouteTable};
use lf_peds_tasks::follow_point_route::task::{
    FLAG_REPEAT, FLAG_ROUTE_SEARCH, FollowPointRouteTask, ORDER_LEG_DONE, PED_BLOCKED, RouteTaskInit,
    SUBTASK_GO_TO_DONE, SUBTASK_LEG_DONE, SUBTASK_ROUTE, SUBTASK_TURN_DONE,
};

/// A host that records every call in order and answers with fixed values.
#[derive(Default)]
struct Log {
    calls: Vec<String>,
    kind: u32,
}

impl RouteTaskHost for Log {
    fn next_order_id(&mut self, task: u32) -> u32 {
        self.calls.push(format!("next({task})"));
        7
    }
    fn order_ped(&mut self, task: u32, order: u32, ped: u32) -> u32 {
        self.calls.push(format!("order({task},{order:#x},{ped})"));
        1
    }
    fn subtask_type(&mut self, _subtask: u32) -> u32 {
        self.kind
    }
    fn create_subtask(&mut self, task: u32, id: u32, ped: u32) -> u32 {
        self.calls.push(format!("create({task},{id:#x},{ped})"));
        2
    }
    fn plan_next_subtask(&mut self, task: u32) -> u32 {
        self.calls.push(format!("plan({task})"));
        SUBTASK_LEG_DONE
    }
    fn select_speed(&mut self, selector: u32) -> f32 {
        self.calls.push(format!("select({selector})"));
        0.5
    }
    fn move_base_ctor(&mut self, task: &mut FollowPointRouteTask, speed_bits: u32) -> u32 {
        self.calls.push(format!("base({},{speed_bits:#x})", task.handle));
        0
    }
    fn release_route(&mut self, table: u32) -> u32 {
        self.calls.push(format!("release({table})"));
        0
    }
    fn base_destroy(&mut self, task: u32) -> u32 {
        self.calls.push(format!("base_destroy({task})"));
        0
    }
    fn allocate_task(&mut self) -> u32 {
        self.calls.push("alloc".to_string());
        0
    }
    fn route_ctor(&mut self, task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
        self.calls.push(format!("route({task},{target},{start},{end},{final_value},{id})"));
        0
    }
}

fn pt(x: f32, y: f32, z: f32) -> RoutePoint {
    RoutePoint { x, y, z }
}

fn ped_at(p: RoutePoint, state: u8) -> PedView {
    PedView {
        handle: 99,
        position: p,
        state_byte: state,
    }
}

/// A task on a two-point route (0,0,0) to (10,0,0), walking from index
/// `index`, with an optional current route.
fn task(index: u32, flags: u32, current: bool) -> FollowPointRouteTask {
    FollowPointRouteTask {
        handle: 5,
        subtask: 6,
        current_route: current.then(|| {
            RouteTable::new(vec![pt(0.0, 0.0, 0.0), pt(10.0, 0.0, 0.0)])
        }),
        waypoints: RouteTable::new(vec![pt(0.0, 0.0, 0.0), pt(10.0, 0.0, 0.0)]),
        route_cursor: 1,
        waypoint_index: index,
        flags,
        ..Default::default()
    }
}

#[test]
fn handle_copy_writes_only_the_handle_slot() {
    let t = FollowPointRouteTask {
        route_cursor: 0x0102_0304,
        ..Default::default()
    };
    let mut rec = vec![0xAAu8; 0x40];
    t.copy_handle_into(&mut rec);
    assert_eq!(&rec[0x38..0x3C], &[0x04, 0x03, 0x02, 0x01]);
    assert!(rec[..0x38].iter().all(|b| *b == 0xAA));
    assert!(rec[0x3C..].iter().all(|b| *b == 0xAA));
}

#[test]
#[should_panic]
fn handle_copy_refuses_a_short_record() {
    let t = FollowPointRouteTask::default();
    let mut rec = vec![0u8; 0x38];
    t.copy_handle_into(&mut rec);
}

#[test]
fn current_point_reads_the_cursor_entry() {
    let t = FollowPointRouteTask {
        current_route: Some(RouteTable::new(vec![pt(1.0, 2.0, 3.0), pt(4.0, 5.0, 6.0)])),
        route_cursor: 1,
        ..Default::default()
    };
    assert_eq!(t.current_point(), pt(4.0, 5.0, 6.0));
}

#[test]
#[should_panic(expected = "no current route")]
fn current_point_refuses_a_null_route() {
    let t = FollowPointRouteTask::default();
    let _ = t.current_point();
}

#[test]
#[should_panic]
fn waypoint_refuses_an_index_past_the_route() {
    let t = FollowPointRouteTask {
        waypoints: RouteTable::new(vec![pt(0.0, 0.0, 0.0)]),
        ..Default::default()
    };
    let _ = t.waypoint(1);
}

#[test]
fn an_empty_route_keeps_the_zero_count_domain() {
    let r = RouteTable::default();
    assert!(r.is_empty());
    assert_eq!(r.len(), 0);
}

#[test]
fn pick_orders_the_leg_done_when_the_index_has_run_past_the_route() {
    let mut t = task(2, FLAG_ROUTE_SEARCH, true);
    let mut host = Log::default();
    let r = t.pick_waypoint(&ped_at(pt(0.0, 0.0, 0.0), 0), &mut host);
    assert_eq!(r, 1);
    assert_eq!(host.calls, vec![format!("order(5,{ORDER_LEG_DONE:#x},99)")]);
    assert_eq!(t.flags & FLAG_ROUTE_SEARCH, 0, "the search flag is cleared");
}

#[test]
fn pick_returns_zero_without_calls_when_the_ped_is_blocked() {
    let mut t = task(0, 0, true);
    let mut host = Log::default();
    let r = t.pick_waypoint(&ped_at(pt(5.0, 1.0, 0.0), PED_BLOCKED), &mut host);
    assert_eq!(r, 0);
    assert!(host.calls.is_empty());
    assert_eq!(t.waypoint_index, 0, "a blocked pick leaves the index alone");
}

#[test]
fn pick_without_a_current_route_asks_for_the_next_order_and_keeps_the_index() {
    let mut t = task(0, 0, false);
    let mut host = Log::default();
    let r = t.pick_waypoint(&ped_at(pt(5.0, 1.0, 0.0), 0), &mut host);
    assert_eq!(r, 1);
    assert_eq!(host.calls, vec!["next(5)".to_string(), "order(5,0x7,99)".to_string()]);
    assert_eq!(t.waypoint_index, 0);
}

#[test]
fn pick_on_an_empty_route_stores_minus_one() {
    let mut t = FollowPointRouteTask {
        handle: 5,
        current_route: Some(RouteTable::default()),
        waypoints: RouteTable::default(),
        waypoint_index: 3,
        ..Default::default()
    };
    let mut host = Log::default();
    let _ = t.pick_waypoint(&ped_at(pt(0.0, 0.0, 0.0), 0), &mut host);
    assert_eq!(t.waypoint_index, u32::MAX, "no point chosen is stored as -1");
}

#[test]
fn pick_takes_the_projection_on_the_segment() {
    // The ped sits beside the middle of the segment from point 0 to point 1.
    let mut t = task(0, 0, true);
    let mut host = Log::default();
    let _ = t.pick_waypoint(&ped_at(pt(5.0, 1.0, 0.0), 0), &mut host);
    assert_eq!(t.waypoint_index, 1, "the segment from point 0 to point 1 wins; its end point is stored");
}

#[test]
fn pick_falls_back_to_the_nearest_point_past_the_end() {
    // Beyond point 1 the projection onto the segment is outside it, so the
    // nearest route point (point 1) wins.
    let mut t = task(0, 0, true);
    let mut host = Log::default();
    let _ = t.pick_waypoint(&ped_at(pt(20.0, 0.0, 0.0), 0), &mut host);
    assert_eq!(t.waypoint_index, 1);
}

#[test]
fn pick_is_defined_for_a_zero_length_segment_and_a_not_a_number_position() {
    let mut t = FollowPointRouteTask {
        handle: 5,
        current_route: Some(RouteTable::new(vec![pt(3.0, 3.0, 3.0), pt(3.0, 3.0, 3.0)])),
        waypoints: RouteTable::new(vec![pt(3.0, 3.0, 3.0), pt(3.0, 3.0, 3.0)]),
        waypoint_index: 0,
        ..Default::default()
    };
    let mut host = Log::default();
    let nan = f32::NAN;
    let r = t.pick_waypoint(&ped_at(pt(nan, 0.0, 0.0), 0), &mut host);
    assert_eq!(r, 1, "the order answer is returned");
    assert!(t.waypoint_index < 2 || t.waypoint_index == u32::MAX);
}

#[test]
fn advance_keeps_the_route_running_on_a_route_subtask() {
    let mut t = task(0, 0, true);
    t.subtask = 6;
    let mut host = Log { calls: vec![], kind: SUBTASK_ROUTE };
    assert_eq!(t.advance_after_subtask(42, &mut host), 0);
    assert!(host.calls.is_empty());
    assert_eq!(t.waypoint_index, 0);
}

#[test]
fn advance_starts_the_route_leg_after_a_finished_leg_or_go_to() {
    for kind in [SUBTASK_LEG_DONE, SUBTASK_GO_TO_DONE] {
        let mut t = task(0, 0, true);
        let mut host = Log { calls: vec![], kind };
        let r = t.advance_after_subtask(42, &mut host);
        assert_eq!(r, 2);
        assert_eq!(host.calls, vec![format!("create(5,{SUBTASK_ROUTE:#x},42)")]);
        assert_eq!(t.waypoint_index, 0, "a leg does not move the index");
    }
}

#[test]
fn advance_at_the_last_turn_starts_the_route_without_repeat() {
    // Index 1 is the last of two points; without the repeat flag the route ends here.
    let mut t = task(1, 0, true);
    let mut host = Log { calls: vec![], kind: SUBTASK_TURN_DONE };
    let _ = t.advance_after_subtask(42, &mut host);
    assert_eq!(t.waypoint_index, 1);
    assert_eq!(host.calls, vec![format!("create(5,{SUBTASK_ROUTE:#x},42)")]);
}

#[test]
fn advance_at_the_last_turn_repeats_when_the_repeat_flag_is_set() {
    let mut t = task(1, FLAG_REPEAT, true);
    let mut host = Log { calls: vec![], kind: SUBTASK_TURN_DONE };
    let _ = t.advance_after_subtask(42, &mut host);
    assert_eq!(t.waypoint_index, 2, "the index wraps past the last point");
    assert_eq!(host.calls, vec!["plan(5)".to_string(), format!("create(5,{SUBTASK_LEG_DONE:#x},42)")]);
}

#[test]
fn advance_on_an_index_wrap_at_the_end_of_an_empty_route_starts_the_route() {
    // With no points the index word of -1 plus one is zero, which equals the
    // count: the turn case then starts the route.
    let mut t = FollowPointRouteTask {
        handle: 5,
        subtask: 6,
        waypoint_index: u32::MAX,
        ..Default::default()
    };
    let mut host = Log { calls: vec![], kind: SUBTASK_TURN_DONE };
    let _ = t.advance_after_subtask(42, &mut host);
    assert_eq!(t.waypoint_index, u32::MAX);
    assert_eq!(host.calls, vec![format!("create(5,{SUBTASK_ROUTE:#x},42)")]);
}

#[test]
fn registry_counts_match_the_methods_lifted() {
    assert_eq!(registry::METHODS.len(), 15, "the class has fifteen listed methods");
    assert_eq!(registry::proven_count(), 10);
}


fn init(route_tag: Option<RouteTable>) -> RouteTaskInit {
    RouteTaskInit {
        speed_bits: 0x3F80_0000,
        target: 11,
        route_tag,
        route_start: 12,
        route_end: 13,
        include_start: 1,
        include_end: 0,
        route_flags: 1,
        route_id: 14,
        final_value: 15,
    }
}

/// A host whose move base sets a known flag word, to see the constructor read it back.
struct BaseSetsFlags(u32, Log);

impl RouteTaskHost for BaseSetsFlags {
    fn next_order_id(&mut self, task: u32) -> u32 { self.1.next_order_id(task) }
    fn order_ped(&mut self, task: u32, order: u32, ped: u32) -> u32 { self.1.order_ped(task, order, ped) }
    fn subtask_type(&mut self, subtask: u32) -> u32 { self.1.subtask_type(subtask) }
    fn create_subtask(&mut self, task: u32, id: u32, ped: u32) -> u32 { self.1.create_subtask(task, id, ped) }
    fn plan_next_subtask(&mut self, task: u32) -> u32 { self.1.plan_next_subtask(task) }
    fn select_speed(&mut self, selector: u32) -> f32 { self.1.select_speed(selector) }
    fn move_base_ctor(&mut self, task: &mut FollowPointRouteTask, speed_bits: u32) -> u32 {
        task.flags = self.0;
        self.1.move_base_ctor(task, speed_bits)
    }
    fn release_route(&mut self, table: u32) -> u32 { self.1.release_route(table) }
    fn base_destroy(&mut self, task: u32) -> u32 { self.1.base_destroy(task) }
    fn allocate_task(&mut self) -> u32 { self.1.allocate_task() }
    fn route_ctor(&mut self, task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
        self.1.route_ctor(task, target, start, end, final_value, id)
    }
}

#[test]
fn constructor_reads_the_flag_word_back_from_the_move_base() {
    // The base leaves 0xF0 in the flag word: bits 0, 2 and 3 come from the
    // arguments (start include 1, end include 0, route flag 1 gives bit 2),
    // and the rest of the base's bits are kept.
    let mut host = BaseSetsFlags(0x0000_00F0, Log::default());
    let t = FollowPointRouteTask::construct(5, init(None), &mut host);
    assert_eq!(t.flags, 0xF5);
    assert_eq!(t.flags & 0x1, 1);
    assert_eq!(t.flags & 0x8, 0);
    assert_eq!(t.flags & 0x4, 4);
    assert_eq!(t.waypoint_index, 0);
    assert_eq!(t.speed_bits, 0x3F80_0000);
    assert_eq!(t.route_target, 11);
}

#[test]
fn constructor_calls_the_base_then_the_route_with_the_arguments_in_order() {
    let mut host = Log::default();
    let _ = FollowPointRouteTask::construct(5, init(None), &mut host);
    assert_eq!(
        host.calls,
        vec![
            "base(5,0x3f800000)".to_string(),
            "route(5,11,12,13,15,14)".to_string(),
        ]
    );
}

#[test]
fn selector_constructor_asks_the_speed_helper_first() {
    let mut host = Log::default();
    let t = FollowPointRouteTask::construct_from_selector(5, 9, init(None), &mut host);
    assert_eq!(host.calls[0], "select(9)");
    assert_eq!(host.calls[1], format!("base(5,{:#x})", 0.5f32.to_bits()));
    assert_eq!(t.speed_bits, 0.5f32.to_bits());
}

#[test]
fn clone_returns_none_when_the_allocator_has_nothing() {
    let t = FollowPointRouteTask { handle: 5, ..Default::default() };
    let mut host = Log::default();
    assert!(t.clone_task(&mut host).is_none());
    assert_eq!(host.calls, vec!["alloc".to_string()]);
}

#[test]
fn clone_constructs_the_copy_from_the_source_words() {
    struct Alloc77(Log);
    impl RouteTaskHost for Alloc77 {
        fn next_order_id(&mut self, task: u32) -> u32 { self.0.next_order_id(task) }
        fn order_ped(&mut self, task: u32, order: u32, ped: u32) -> u32 { self.0.order_ped(task, order, ped) }
        fn subtask_type(&mut self, subtask: u32) -> u32 { self.0.subtask_type(subtask) }
        fn create_subtask(&mut self, task: u32, id: u32, ped: u32) -> u32 { self.0.create_subtask(task, id, ped) }
        fn plan_next_subtask(&mut self, task: u32) -> u32 { self.0.plan_next_subtask(task) }
        fn select_speed(&mut self, selector: u32) -> f32 { self.0.select_speed(selector) }
        fn move_base_ctor(&mut self, task: &mut FollowPointRouteTask, speed_bits: u32) -> u32 {
            self.0.move_base_ctor(task, speed_bits)
        }
        fn release_route(&mut self, table: u32) -> u32 { self.0.release_route(table) }
        fn base_destroy(&mut self, task: u32) -> u32 { self.0.base_destroy(task) }
        fn allocate_task(&mut self) -> u32 {
            self.0.calls.push("alloc".to_string());
            77
        }
        fn route_ctor(&mut self, task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
            self.0.route_ctor(task, target, start, end, final_value, id)
        }
    }
    // Flag bit 3 set, bit 2 clear, bit 0 set: the constructor's end include is 1 and its route flag 0.
    let src = FollowPointRouteTask {
        handle: 5,
        speed_bits: 0x4000_0000,
        route_target: 21,
        route_start: 22,
        route_end: 23,
        final_value: 24,
        current_route: Some(RouteTable::new(vec![pt(1.0, 2.0, 3.0)])),
        waypoint_index: 25,
        flags: 0x9,
        ..Default::default()
    };
    let mut host = Alloc77(Log::default());
    let copy = src.clone_task(&mut host).expect("allocation answered");
    assert_eq!(copy.handle, 77);
    assert_eq!(copy.current_route, src.current_route);
    assert_eq!(
        host.0.calls,
        vec![
            "alloc".to_string(),
            "base(77,0x40000000)".to_string(),
            "route(77,21,22,23,24,25)".to_string(),
        ]
    );
    // The flag bits the constructor takes come from the source flag word.
    assert_eq!(copy.flags & 0x1, 1);
    assert_eq!(copy.flags & 0x8, 0x8);
    assert_eq!(copy.flags & 0x4, 0);
}



