// original: 0x009e73d0 model_kind_to_task_id
/// Maps the model-info tag at `+0x2E` through the shared model
/// table to a task id: tag class `0x52/0x55/0x54/0x56` (word at table
/// entry `+0xC4`) gives `0x85/0x86/0x87/0x88`, anything else -1.
/// (cdecl, one argument.)
lf_checker_rt::export!(cdecl, rw_009e73d0(model_ptr: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0x2E;
        const CLASS_OFF: u32 = 0xC4;
        const MODEL_TABLE: u32 = 0x1295CD8;
        let tag = (model_ptr.wrapping_add(TAG_OFF) as *const i16).read_unaligned() as i32;
        let entry = lf_checker_rt::global::<u32>(MODEL_TABLE)
            .wrapping_offset(tag as isize)
            .read_unaligned();
        let class = (entry.wrapping_add(CLASS_OFF) as *const u32).read_unaligned();
        if class == 0x56 {
            0x88
        } else if class == 0x54 {
            0x87
        } else if class == 0x55 {
            0x86
        } else if class == 0x52 {
            0x85
        } else {
            0xFFFFFFFF
        }
    }
});
