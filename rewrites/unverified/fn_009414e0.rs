// original: 0x009414e0 streaming_probe_or_add (proposed)

/// Probe the streaming slot, queueing a zeroed request when it is active.
///
/// Returns 1 without doing anything unless the active bit (0x40) of the
/// flag byte at `obj + 0xf1f` is set and the linked slot (if any) at
/// `obj + 0xf50` is unmarked. Then it builds a zeroed 16-byte request on
/// the stack, hands its address to the queue worker (which fills it) and
/// returns 1. Only the low byte of the return value is set.
///
/// Original: 0x009414e0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_009414e0(obj: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0xF1F;
        const ACTIVE: u8 = 0x40;
        const NEXT: u32 = 0xF50;
        const NEXT_MARKER: u32 = 0x219;
        const CALLEE: u32 = 1;
        if ((obj + FLAGS) as *const u8).read() & ACTIVE == 0 {
            return 1;
        }
        let next = ((obj + NEXT) as *const u32).read_unaligned();
        if next != 0 && ((next + NEXT_MARKER) as *const u8).read() != 0 {
            return 1;
        }
        let mut request = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE,
            u32,
            obj,
            request.as_mut_ptr() as u32
        );
        1
    }
});
