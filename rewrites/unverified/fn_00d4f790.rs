// original: 0x00D4F790 task_clear_flag_slot (proposed)

// Resets the flag slot of the object the second argument points to: writes 0
/// to `+0x1c` and 1 to `+0x20`. The first argument is ignored. Returns nothing.
///
/// Original: 0x00D4F790 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00d4f790(_unused: u32, obj: u32) -> u32 {
    unsafe {
        ((obj + 0x1c) as *mut u32).write_unaligned(0);
        ((obj + 0x20) as *mut u8).write(1);
        0
    }
});
