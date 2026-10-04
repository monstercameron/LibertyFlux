// original: 0x00CB3B50 CTaskComplexMoveFollowPointRoute::vf18

/// Step the point-route follower to the next waypoint when one finishes.
///
/// `this` is the complex task, `ped` the ped. The current subtask's type
/// (virtual slot 3, callee 1, asked up to four times) decides: the route
/// id itself (0x516) keeps the subtask (null); a finished leg (0x11c) or
/// a finished go-to (0x11a) starts the next leg (0x516). A finished turn
/// (0x384) advances the waypoint index (`this+0x38`) only past the last
/// waypoint or when the repeat flag (`this+0x3c` bit 2) is set, otherwise
/// it also starts the next leg. Any other type, and the advance case,
/// increment the index and create whatever subtask the route planner
/// (callee 3) names. Creation goes through the factory (callee 2) as
/// (id, ped).
///
/// Original: 0x00CB3B50 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3B50(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const GET_TYPE_SLOT: u32 = 0x0c;
        const CREATE_SUB: u32 = 2;
        const PLAN_NEXT: u32 = 3;
        const WAYPOINT_INDEX: u32 = 0x38;
        const WAYPOINTS: u32 = 0x34;
        const OPTIONS: u32 = 0x3c;
        const REPEAT_BIT: u8 = 4;
        const ROUTE: u32 = 0x516;
        const LEG_DONE: u32 = 0x11c;
        const GO_TO_DONE: u32 = 0x11a;
        const TURN_DONE: u32 = 0x384;
        let sub = (this.wrapping_add(SUBTASK) as *const u32).read_unaligned();
        let vtable = (sub as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if get_type(sub) == ROUTE {
            return 0;
        }
        if get_type(sub) == LEG_DONE {
            return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ROUTE, ped);
        }
        if get_type(sub) == GO_TO_DONE {
            return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ROUTE, ped);
        }
        if get_type(sub) == TURN_DONE {
            let index = (this.wrapping_add(WAYPOINT_INDEX) as *const u32).read_unaligned();
            let points = (this.wrapping_add(WAYPOINTS) as *const u32).read_unaligned();
            let count = (points as *const u32).read_unaligned();
            let opts = (this.wrapping_add(OPTIONS) as *const u8).read_unaligned();
            if index.wrapping_add(1) == count && opts & REPEAT_BIT == 0 {
                return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ROUTE, ped);
            }
        }
        let index = (this.wrapping_add(WAYPOINT_INDEX) as *const u32).read_unaligned();
        (this.wrapping_add(WAYPOINT_INDEX) as *mut u32).write_unaligned(index.wrapping_add(1));
        let next: u32 = lf_checker_rt::callee_thiscall!(PLAN_NEXT, u32, this);
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, next, ped)
    }
});
