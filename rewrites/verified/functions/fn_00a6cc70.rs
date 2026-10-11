// original: 0x00A6CC70 global_list_prepend

/// Prepends one caller-owned node to the process-wide list.
///
/// `this` is the node and `stored_value` becomes its first word. The prior
/// list head is saved at `this + 8`, the marker at `this + 4` is reset to
/// `u32::MAX`, and the global head is replaced with `this`. This is a
/// 32-bit thiscall routine with one stack argument; it returns the node.
lf_checker_rt::export!(thiscall, rw_00a6cc70(this: u32, stored_value: u32) -> u32 {
    unsafe {
        const MARKER_OFFSET: u32 = 4;
        const NEXT_OFFSET: u32 = 8;
        const LIST_HEAD_VA: u32 = 0x12F9_D3C;
        const EMPTY_MARKER: u32 = u32::MAX;

        let head = lf_checker_rt::global::<u32>(LIST_HEAD_VA).read();
        (this as *mut u32).write_unaligned(stored_value);
        ((this + MARKER_OFFSET) as *mut u32).write_unaligned(EMPTY_MARKER);
        ((this + NEXT_OFFSET) as *mut u32).write_unaligned(head);
        lf_checker_rt::global::<u32>(LIST_HEAD_VA).write(this);
        this
    }
});
