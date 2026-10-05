// original: 0x00b1fca0 store_low5bits (proposed)

/// Stores the low 5 bits of `value` through `out` and returns `out`.
///
/// Arguments are two stack words (stdcall, callee pops 8 bytes): the value
/// to mask and the destination pointer. Only the masked word is written;
/// no other state is touched. Returns the destination pointer unchanged.
lf_checker_rt::export!(stdcall, rw_00b1fca0(value: u32, out: u32) -> u32 {
    unsafe {
        const MASK: u32 = 0x1f;
        (out as *mut u32).write_unaligned(value & MASK);
    }
    out
});
