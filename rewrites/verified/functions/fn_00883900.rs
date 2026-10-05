// original: 0x00883900 stream_pool_ptr_to_index (proposed)
/// Map a pool object pointer back to its slot index.
///
/// Computes `(ptr - base) / 24` with signed division (`SLOT_BYTES`), where
/// the pool base is the global at file address `0x1154088`. The original
/// divides through the multiply-high magic constant `0x2aaaaaab`; the
/// quotient is exact truncation toward zero for every input, negative
/// differences included, which plain signed division reproduces.
///
/// Original: cdecl, one stack argument, returns the index in `eax`.
lf_checker_rt::export!(cdecl, rw_00883900(ptr: u32) -> u32 {
    unsafe {
        const POOL_BASE: u32 = 0x0115_4088;
        const SLOT_BYTES: i32 = 24;
        let base = (lf_checker_rt::global::<u32>(POOL_BASE) as *const u32).read_unaligned();
        let diff = ptr.wrapping_sub(base) as i32;
        diff.wrapping_div(SLOT_BYTES) as u32
    }
});
