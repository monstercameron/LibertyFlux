// original: 0x00a7c410 bu_task_alloc_field_12c
/// Allocates a fixed-size block, constructs it in place, stores it in the
/// field and reports whether the result is non-null.
export!(thiscall, rw_00a7c410(obj: *mut u8) -> u32 {
    unsafe {
        let mem = callee_cdecl!(1, u32, 0x1490);
        if mem == 0 {
            *((obj as *mut u8).add(0x12c) as *mut u32) = 0;
            return 0;
        }
        let r = callee_thiscall!(2, u32, mem);
        *((obj as *mut u8).add(0x12c) as *mut u32) = r;
        (r & 0xFFFFFF00) | ((r != 0) as u32)
    }
});
