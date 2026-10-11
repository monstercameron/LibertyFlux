// original: 0x00cabc00 CTaskComplexMoveFollowPointRoute::CTaskComplexMoveFollowPointRoute

/// Constructs a route-following move task. A selector-derived f32 is passed
/// through the move base constructor, and five arguments are forwarded to the
/// route-task helper. The word at `+0x20`, the flag bits at `+0x3c`, and the
/// zero at `+0x38` are initialized before the route helper runs. The three
/// boolean inputs contribute their low bits to flags 0 and 3, while the route
/// flags low bit contributes to flag 2. The fifth stack word is forwarded as
/// the route end value. Existing flag bits other than 0, 2 and 3 are retained.
lf_checker_rt::export!(thiscall, rw_00cabc00(
    this: u32,
    selector: u32,
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
    const FLOAT_SELECT: u32 = 1;
    const MOVE_BASE_CTOR: u32 = 2;
    const ROUTE_CTOR: u32 = 3;
    const TASK_VTABLE_VA: u32 = 0x00ed_85bc;
    const BASE_VTABLE_VA: u32 = 0x00ed_8618;
    const ROUTE_TAG: u32 = 0x20;
    const FLAGS: u32 = 0x3c;

    let speed = lf_checker_rt::callee_cdecl!(FLOAT_SELECT, f32, selector);
    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed.to_bits());
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
