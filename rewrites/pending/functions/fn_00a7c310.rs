// original: 0x00a7c310 bu_task_detach_and_teardown
/// Unlinks the node, asks its gate slot whether teardown should proceed, then
/// clears the peer state words and releases the two owned fields.
export!(thiscall, rw_00a7c310(obj: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        let vt = *(obj as *const u32);
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 0x1c) as *const u32) as usize);
        let r = gate(obj as u32);
        if r & 0xFF == 0 {
            return r;
        }
        let peer = *((obj as *const u8).add(0x110) as *const u32);
        if peer != 0 {
            *((peer + 0x27c) as *mut u8) = 0;
            *((peer + 0x270) as *mut u32) = 0;
        }
        callee_thiscall!(3, u32, obj as u32);
        let q = callee_thiscall!(4, u32, obj as u32);
        (q & 0xFFFFFF00) | 1
    }
});
