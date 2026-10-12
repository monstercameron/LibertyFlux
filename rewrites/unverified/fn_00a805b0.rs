// original: 0x00a805b0 CSimpleNMFlinchTaskInfo::create_task

/// Create a flinch task using its virtual result, resolved entity, and payload.
///
/// The receiver starts with a vtable pointer; slot `+0x30` is called as a
/// zero-argument `thiscall` method. If task-pool allocation succeeds, the
/// method calls that slot, resolves the flinch entity, and constructs the new
/// task with the receiver's payload at `+0x20`, then the resolved entity, then
/// the virtual method's result. The constructor's `EAX` is returned. The
/// indirect call remains a real table load so the contract can plant and
/// observe its stand-in.
lf_checker_rt::export!(thiscall, rw_00a805b0(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const VTABLE_POINTER: u32 = 0;
    const VTABLE_SLOT_12: u32 = 0x30;
    const EMBEDDED_PAYLOAD: u32 = 0x20;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const VIRTUAL_FLINCH_QUERY: u32 = 2;
    const FLINCH_ENTITY_LOOKUP: u32 = 3;
    const FLINCH_TASK_CONSTRUCTOR: u32 = 4;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let vtable = unsafe { ((task_info + VTABLE_POINTER) as *const u32).read_unaligned() };
    let virtual_target = unsafe {
        ((vtable.wrapping_add(VTABLE_SLOT_12)) as *const u32).read_unaligned()
    };
    let virtual_query: unsafe extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(virtual_target as usize)
    };
    let virtual_result = unsafe { virtual_query(task_info) };

    let flinch_entity = lf_checker_rt::callee_thiscall!(FLINCH_ENTITY_LOOKUP, u32, task_info);
    let payload = task_info.wrapping_add(EMBEDDED_PAYLOAD);
    lf_checker_rt::callee_thiscall!(
        FLINCH_TASK_CONSTRUCTOR,
        u32,
        task,
        payload,
        flinch_entity,
        virtual_result
    )
});
