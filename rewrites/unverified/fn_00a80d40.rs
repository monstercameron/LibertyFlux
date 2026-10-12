// original: 0x00a80d40 create_and_init_task_info

/// Look up task-info storage, encode its two mode bits, and invoke its initializer.
///
/// This `stdcall` function accepts a task-info identifier, a byte-valued option,
/// a mode word, and a value forwarded to the initializer. The identifier lookup
/// returns null or a task-info object. On success, the object word at `+0x08`
/// keeps its upper 28 bits; its low nibble becomes the low bit of the option in
/// bit zero and the low three mode bits shifted into bits one through three.
/// The object's vtable slot three is called as a `thiscall` with the forwarded
/// value, and the object pointer is returned. Both calls use checker stand-ins.
lf_checker_rt::export!(stdcall, rw_00a80d40(
    task_info_id: u32,
    option: u32,
    mode: u32,
    forwarded_value: u32
) -> u32 {
    const INFO_FLAGS_OFFSET: u32 = 0x08;
    const LOW_NIBBLE_CLEAR_MASK: u32 = 0xFFFF_FFF0;
    const INFO_ID_LOOKUP: u32 = 1;
    const INFO_INITIALIZER_SLOT_THREE: u32 = 2;

    let task_info = lf_checker_rt::callee_cdecl!(INFO_ID_LOOKUP, u32, task_info_id);
    if task_info == 0 {
        return 0;
    }

    let old_flags = unsafe {
        ((task_info.wrapping_add(INFO_FLAGS_OFFSET)) as *const u32).read_unaligned()
    };
    let encoded_flags = ((mode & 0x07) << 1) | (option & 0x01);
    let new_flags = (old_flags & LOW_NIBBLE_CLEAR_MASK) | encoded_flags;
    unsafe {
        ((task_info.wrapping_add(INFO_FLAGS_OFFSET)) as *mut u32)
            .write_unaligned(new_flags);
    }

    let _initializer_result = lf_checker_rt::callee_thiscall!(
        INFO_INITIALIZER_SLOT_THREE,
        u32,
        task_info,
        forwarded_value
    );
    task_info
});
