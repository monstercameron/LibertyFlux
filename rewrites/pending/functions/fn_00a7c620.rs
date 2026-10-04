// original: 0x00a7c620 bu_task_free_ptr_110
/// Runs the destructor on the owned field when present, frees the block and
/// always leaves the field nulled.
export!(thiscall, rw_00a7c620(obj: *mut u8) -> u32 {
    unsafe {
        let inner = *((obj as *const u8).add(0x110) as *const u32);
        if inner != 0 {
            callee_thiscall!(1, u32, inner);
            let q = callee_cdecl!(2, u32, inner);
            *((obj as *mut u8).add(0x110) as *mut u32) = 0;
            return q;
        }
        *((obj as *mut u8).add(0x110) as *mut u32) = 0;
        0
    }
});
