// original: 0x005dd3b0 init_task_flags_if_type_299
/// Initialise a task flag word: clear the status slot, then set the armed
/// bit when the source record has the expected type id. Returns the source.
export!(cdecl, rw_005dd3b0(src: *const u8, dst: *mut u8) -> u32 {
    unsafe {
        *((dst as *mut u8).add(0x20) as *mut u32) = 0;
        if *((src.add(0xc)) as *const u32) == 0x12b {
            *dst.add(0x14) |= 2;
        }
        src as u32
    }
});
