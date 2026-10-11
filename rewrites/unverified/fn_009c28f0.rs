// original: 0x009C28F0 stream_context_status_pointer

/// Look up the context pointer indexed by index in the streaming context
/// table. Return null for an empty slot; otherwise return the address of the
/// context status block at +0x103C0. The lookup uses 32-bit scaled-index
/// arithmetic, including wraparound for high indices.
lf_checker_rt::export!(cdecl, rw_009c28f0(index: u32) -> u32 {
    unsafe {
        const CONTEXT_TABLE: u32 = 0x0129_3e58;
        const STATUS_BLOCK: u32 = 0x103c0;
        let slot_va = CONTEXT_TABLE.wrapping_add(index.wrapping_mul(4));
        let context = lf_checker_rt::global::<u32>(slot_va).read_unaligned();
        if context == 0 {
            0
        } else {
            context.wrapping_add(STATUS_BLOCK)
        }
    }
});
