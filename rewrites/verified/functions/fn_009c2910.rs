// original: 0x009C2910 stream_context_handle_if_ready

/// Return the dword at `this + 0x40` when the 16-bit state at `this + 0x44`
/// equals `READY`; otherwise return zero. The state comparison is unsigned
/// and exact. This is a leaf thiscall getter and does not modify the object.
lf_checker_rt::export!(thiscall, rw_009c2910(this: u32) -> u32 {
    unsafe {
        const CONTEXT_HANDLE: usize = 0x40;
        const CONTEXT_STATE: usize = 0x44;
        const READY: u16 = 1;
        let base = this as usize as *const u8;
        let state = base.add(CONTEXT_STATE).cast::<u16>().read_unaligned();
        if state == READY {
            base.add(CONTEXT_HANDLE).cast::<u32>().read_unaligned()
        } else {
            0
        }
    }
});
