// original: 0x008873A0 stream_free_pair (proposed)

/// Release the slot buffer and the shared stream block, when present.
///
/// Frees (callee 1) the buffer at `[this]` unless it is null, then the
/// block named by the shared-block global unless that is null. The answer
/// is the second free's answer when the global is non-null, otherwise 0
/// (the just-loaded null global still sits in `eax`).
///
/// Original: 0x008873A0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_008873A0(this: u32) -> u32 {
    unsafe {
        const BLOCK_GLOBAL: u32 = 0x0115_a45c;
        const FREE: u32 = 1;
        let first = (this as *const u32).read_unaligned();
        if first != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, first);
        }
        let block = (lf_checker_rt::relocated(BLOCK_GLOBAL) as *const u32)
            .read_unaligned();
        if block != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, block)
        } else {
            0
        }
    }
});
