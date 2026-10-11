// original: 0x00cabca0 CTaskComplexMoveFollowPointRoute::CTaskComplexMoveFollowPointRoute_2

/// Constructs the route-following move task using the supplied speed word.
/// The speed is forwarded unchanged to the move base constructor. The route
/// tag at `+0x20`, packed low flag bits at `+0x3c`, both vtable pointers and
/// the zero field at `+0x38` are installed before the route helper receives
/// its five arguments. Existing flag bits other than 0, 2 and 3 are retained.
lf_checker_rt::export!(thiscall, rw_00cabca0(
    this: u32,
    speed_bits: u32,
    target: u32,
    route_tag: u32,
    route_start: u32,
    route_end: u32,
    include_start: u32,
    include_end: u32,
    route_flags: u32,
    route_id: u32,
    final_value: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const ROUTE_CTOR: u32 = 2;
    const TASK_VTABLE_VA: u32 = 0x00ed_85bc;
    const BASE_VTABLE_VA: u32 = 0x00ed_8618;
    const ROUTE_TAG: u32 = 0x20;
    const FLAGS: u32 = 0x3c;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed_bits);
    let old_flags = unsafe { ((this + FLAGS) as *const u32).read_unaligned() };
    let packed_flags = ((include_end & 1) << 3)
        | ((route_flags & 1) << 2)
        | (include_start & 1)
        | (old_flags & 0xffff_fff2);

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));
        ((this + ROUTE_TAG) as *mut u32).write_unaligned(route_tag);
        ((this + FLAGS) as *mut u32).write_unaligned(packed_flags);
        ((this + 0x38) as *mut u32).write_unaligned(0);
    }

    let _ = lf_checker_rt::callee_thiscall!(
        ROUTE_CTOR,
        u32,
        this,
        target,
        route_start,
        route_end,
        final_value,
        route_id
    );
    this
});
