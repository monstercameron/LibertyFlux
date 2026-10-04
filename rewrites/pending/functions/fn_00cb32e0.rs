// original: 0x00CB32E0 CTaskComplexFollowPatrolRoute::vf18

/// Follow the patrol route, advancing past finished legs and waypoints.
///
/// `this` is the complex task, `ped` the ped (unused). The current
/// subtask's type (virtual slot 3, callee 1) and, for a leg (0x11d), its
/// own subtask's type (callee 2) decide: a leg guarding the wait subtask
/// (0x3ae) is replaced by whatever the route planner (callee 4) names; a
/// leg guarding a finished leg (0x11c) is replaced by the follow-up leg
/// (0x516). A route marker (0x191), and any other type whose waypoint
/// name matches the table entry (compared byte-wise against the image
/// string, which is empty, so only an empty name matches), advance the
/// waypoint index (`this+0x1a`) and likewise take the planner's answer.
/// A non-matching name restarts the route marker (0x191). Creation goes
/// through the factory (callee 3, id only).
///
/// Original: 0x00CB32E0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB32E0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const GET_TYPE_SLOT: u32 = 0x0c;
        const SUB_SUBTASK: u32 = 0x14;
        const CREATE_SUB: u32 = 3;
        const PLAN_NEXT: u32 = 4;
        const WAYPOINT_INDEX: u32 = 0x1a;
        const WAYPOINT_NAMES: u32 = 0x24;
        const NAME_ENTRY_LEN: i32 = 56;
        const NAME_TABLE: u32 = 0xfc9c85;
        const LEG: u32 = 0x11d;
        const WAIT_SUBTASK: u32 = 0x3ae;
        const LEG_DONE: u32 = 0x11c;
        const ROUTE_MARKER: u32 = 0x191;
        const FOLLOW_UP: u32 = 0x516;
        let _ = ped;
        let sub = (this.wrapping_add(SUBTASK) as *const u32).read_unaligned();
        let vtable = (sub as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let sub2_of = |task: u32| {
            let inner = (task.wrapping_add(SUB_SUBTASK) as *const u32).read_unaligned();
            if inner == 0 {
                return None;
            }
            let vt = (inner as *const u32).read_unaligned();
            let sl = (vt.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(sl as usize);
            Some(probe(inner))
        };
        if get_type(sub) == LEG {
            if let Some(WAIT_SUBTASK) = sub2_of(sub) {
                let next: u32 = lf_checker_rt::callee_thiscall!(PLAN_NEXT, u32, this);
                return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, next);
            }
        }
        if get_type(sub) == LEG {
            if let Some(LEG_DONE) = sub2_of(sub) {
                return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, FOLLOW_UP);
            }
        }
        if get_type(sub) != ROUTE_MARKER {
            let index =
                (this.wrapping_add(WAYPOINT_INDEX) as *const i16).read_unaligned() as i32;
            let names = (this.wrapping_add(WAYPOINT_NAMES) as *const u32).read_unaligned();
            let entry =
                (names as i32).wrapping_add(index.wrapping_mul(NAME_ENTRY_LEN)).wrapping_add(4)
                    as u32;
            let table = lf_checker_rt::relocated(NAME_TABLE);
            let mut at = 0u32;
            let matched = loop {
                let a = (entry.wrapping_add(at) as *const u8).read_unaligned();
                let b = (table.wrapping_add(at) as *const u8).read_unaligned();
                if a != b {
                    break false;
                }
                if a == 0 {
                    break true;
                }
                at = at.wrapping_add(1);
            };
            if !matched {
                return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ROUTE_MARKER);
            }
        }
        let index = (this.wrapping_add(WAYPOINT_INDEX) as *const u16).read_unaligned();
        (this.wrapping_add(WAYPOINT_INDEX) as *mut u16)
            .write_unaligned(index.wrapping_add(1));
        let next: u32 = lf_checker_rt::callee_thiscall!(PLAN_NEXT, u32, this);
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, next)
    }
});
