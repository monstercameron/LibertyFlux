// original: 0x008838e0 stream_pool_index_to_ptr (proposed)
/// Map a pool slot index to its object pointer, or null when out of range.
///
/// The pool base comes from the global at file address `0x1154088` and the
/// slot count (a half-word) from `0x1154098`-relative `0x115408e`. Slot `i`
/// lives at `base + i * 24` (`SLOT_BYTES`); an index at or above the count
/// yields null. The comparison is unsigned.
///
/// Original: cdecl, one stack argument, returns the pointer in `eax`.
lf_checker_rt::export!(cdecl, rw_008838e0(index: u32) -> u32 {
    unsafe {
        const POOL_BASE: u32 = 0x0115_4088;
        const POOL_COUNT: u32 = 0x0115_408e;
        const SLOT_BYTES: u32 = 24;
        let count =
            (lf_checker_rt::global::<u16>(POOL_COUNT) as *const u16).read_unaligned() as u32;
        if index >= count {
            0
        } else {
            let base =
                (lf_checker_rt::global::<u32>(POOL_BASE) as *const u32).read_unaligned();
            base.wrapping_add(index.wrapping_mul(SLOT_BYTES))
        }
    }
});
