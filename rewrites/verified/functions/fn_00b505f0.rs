// original: 0x00B505F0 event_node_init

/// Initialise an event-graph node: vtable, payload link, zero tail.
///
/// Layout at `this`: `+0x00` the node vtable address (file VA 0x00EAED44,
/// relocated with the image), `+0x04` the `link` argument, `+0x08` zero, and
/// 64 zero bytes from `+0x0C` to `+0x4B` (eight 8-byte stores). Returns `this`.
///
/// Original: 0x00B505F0 (thiscall, `this` in ecx, one stack word).
export!(thiscall, rw_00b505f0(this: u32, link: u32) -> u32 {
    const VTABLE_FILE_VA: u32 = 0x00EAED44;
    const LINK_OFF: u32 = 0x04;
    const ZERO_FROM: u32 = 0x08;
    const ZERO_QWORDS_AT: u32 = 0x0C;
    const ZERO_QWORDS: u32 = 8;
    unsafe {
        ((this) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
        ((this + LINK_OFF) as *mut u32).write_unaligned(link);
        ((this + ZERO_FROM) as *mut u32).write_unaligned(0);
        let mut i = 0u32;
        while i < ZERO_QWORDS {
            ((this + ZERO_QWORDS_AT + i * 8) as *mut u64).write_unaligned(0);
            i += 1;
        }
    }
    this
});
