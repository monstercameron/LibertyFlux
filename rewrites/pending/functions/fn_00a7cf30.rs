// original: 0x00a7cf30 bu_task_maybe_notify
/// Forwards a value from the owned field to the next module, but only while
/// the node is enabled, flagged and actually owns the field.
export!(thiscall, rw_00a7cf30(obj: *mut u8) -> u32 {
    unsafe {
        if *((obj as *const u8).add(0x130) as *const u32) == 0 {
            return 0;
        }
        if *((obj as *const u8).add(0x13c) as *const u8) & 1 == 0 {
            return 0;
        }
        let inner = *((obj as *const u8).add(0x110) as *const u32);
        if inner == 0 {
            return 0;
        }
        let v = *((inner + 0x274) as *const u32);
        callee_thiscall!(1, u32, (obj as u32).wrapping_add(0x10), v)
    }
});
