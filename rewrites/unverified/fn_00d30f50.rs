// original: 0x00D30F50 find_route_sub_task
/// Finds the current route task and checks its type, its child type, and the previous task slot
/// in a game-populated task-pool table. The task-pool index is multiplied by its fixed record
/// stride, while the task index is signed for the index-minus-one guard. The stdcall function
/// takes one route pointer, returns a Boolean in AL, and uses scripted stand-ins for the direct
/// lookup and object virtual methods.
lf_checker_rt::export!(stdcall, rw_00d30f50(this: u32, route: u32) -> u8 {
    const TASK_POOL_TABLE_VA: u32 = 0x0167f780;
    const TASK_POOL_STRIDE: u32 = 0x6c;
    const CURRENT_TASK_SLOT: u32 = 0x18;
    const PREVIOUS_TASK_SLOT: u32 = 0x14;
    const TASK_TYPE_SLOT: u32 = 0x0c;
    const ROUTE_TASK_TYPE: u32 = 0x11d;
    const FOLLOW_ROUTE_KIND: u32 = 0x3ae;
    const FOLLOW_ROUTE_ALT_KIND: u32 = 0x3a9;
    const ACCEPTED_PREVIOUS_KINDS: [u32; 5] = [0x384, 0x3af, 0x3ae, 0x3a9, 0x389];
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    let route_manager = unsafe { read_u32(route, 0x224) };
    let lookup_context = route_manager.wrapping_add(0x44);
    let descriptor = lf_checker_rt::callee_thiscall!(1, u32, lookup_context, 0x111);
    if descriptor == 0 { return 0; }
    let pool_index = unsafe { read_u32(descriptor, 0x14) };
    let task_index = unsafe { read_u32(descriptor, 0x18) };
    let pool_offset = pool_index.wrapping_mul(TASK_POOL_STRIDE);
    let pool_base = lf_checker_rt::global::<u8>(TASK_POOL_TABLE_VA).wrapping_add(pool_offset as usize) as u32;
    let current_offset = CURRENT_TASK_SLOT.wrapping_add(task_index.wrapping_mul(4));
    let current_task = unsafe { read_u32(pool_base, current_offset) };
    let current_type = lf_checker_rt::callee_thiscall!(2, u32, current_task);
    if current_type != ROUTE_TASK_TYPE { return 0; }
    let current_subtask = unsafe { read_u32(current_task, 0x14) };
    if current_subtask == 0 { return 0; }
    let first_child_kind = lf_checker_rt::callee_thiscall!(3, u32, current_subtask);
    if first_child_kind != FOLLOW_ROUTE_KIND {
        let second_child_kind = lf_checker_rt::callee_thiscall!(3, u32, current_subtask);
        if second_child_kind != FOLLOW_ROUTE_ALT_KIND { return 0; }
    }
    if (task_index.wrapping_sub(1) as i32) < 0 { return 0; }
    let previous_offset = PREVIOUS_TASK_SLOT.wrapping_add(task_index.wrapping_mul(4));
    let previous_task = unsafe { read_u32(pool_base, previous_offset) };
    if previous_task == 0 { return 0; }
    let previous_type = lf_checker_rt::callee_thiscall!(2, u32, previous_task);
    if previous_type != ROUTE_TASK_TYPE { return 0; }
    let previous_subtask = unsafe { read_u32(previous_task, 0x14) };
    if previous_subtask != 0 {
        let previous_kind = lf_checker_rt::callee_thiscall!(3, u32, previous_subtask);
        if ACCEPTED_PREVIOUS_KINDS.contains(&previous_kind) { return 1; }
    }
    (lf_checker_rt::callee_thiscall!(2, u32, previous_task) == 0x640) as u8
});
