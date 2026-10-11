// original: 0x00CC03C0 has_required_task
/// Return one in AL when the entity has a non-null task pointer at offset
/// 0xAB0, the task's word at 0x28 has masked value 0x80 under mask 0x3C0,
/// and its word at 0x1304 equals two. The argument is a cdecl pointer.
lf_checker_rt::export!(cdecl, rw_00cc03c0(entity: *const u8) -> u32 {
    const TASK_POINTER_OFFSET: usize = 0xAB0;
    const TASK_FLAGS_OFFSET: usize = 0x28;
    const TASK_TYPE_OFFSET: usize = 0x1304;
    const REQUIRED_FLAG_BITS: u32 = 0x3C0;
    const REQUIRED_FLAGS: u32 = 0x80;
    const REQUIRED_TASK_TYPE: u32 = 2;
    let task = unsafe { entity.add(TASK_POINTER_OFFSET).cast::<u32>().read_unaligned() as *const u8 };
    if task.is_null() {
        return 0;
    }
    let flags = unsafe { task.add(TASK_FLAGS_OFFSET).cast::<u32>().read_unaligned() };
    let task_type = unsafe { task.add(TASK_TYPE_OFFSET).cast::<u32>().read_unaligned() };
    u32::from(flags & REQUIRED_FLAG_BITS == REQUIRED_FLAGS && task_type == REQUIRED_TASK_TYPE)
});
