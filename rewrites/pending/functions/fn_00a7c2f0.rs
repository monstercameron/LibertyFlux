// original: 0x00a7c2f0 bu_task_poll_and_reset
/// Polls the node through its second method-table slot; when the poll reports
/// nonzero, runs the shared reset helper on the node and reports handled.
export!(thiscall, rw_00a7c2f0(obj: *mut u8) -> u32 {
    unsafe {
        let vt = *(obj as *const u32);
        let poll: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 8) as *const u32) as usize);
        let r = poll(obj as u32);
        if r & 0xFF == 0 {
            return r;
        }
        let q = callee_thiscall!(2, u32, obj as u32, 0);
        (q & 0xFFFFFF00) | 1
    }
});
