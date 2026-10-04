// original: 0x00925E50 input_size_from_count
/// Map a count to a buffer size: 0 below 256, else `count*count*16 + 24`.
///
/// All arithmetic wraps modulo 2^32 exactly like the original's `imul`/`shl`.
export!(cdecl, rw_00925E50(a: u32) -> u32 {
    if a < 0x100 {
        0
    } else {
        a.wrapping_mul(a).wrapping_shl(4).wrapping_add(0x18)
    }
});
