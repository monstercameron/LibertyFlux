// original: 0x009451f0 streaming_count_active (proposed)

/// Count down the active streaming passes while the worker accepts them.
///
/// Reads the pass count byte. A zero count returns 0 with no call.
/// Otherwise polls the worker (cdecl, no arguments): when its low byte is
/// non-zero one pass is consumed and the count minus one is returned, else
/// the unchanged count. Returns the full zero-extended count in `eax`.
///
/// Original: 0x009451f0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009451f0() -> u32 {
    unsafe {
        const PASSES: u32 = 0x011D74F1;
        const CALLEE: u32 = 1;
        let count = lf_checker_rt::global::<u8>(PASSES).read() as u32;
        if count == 0 {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_cdecl!(CALLEE, u32,);
        if ok & 0xFF != 0 {
            count.wrapping_sub(1)
        } else {
            count
        }
    }
});
