// original: 0x00D9CA00 store_vehicle_transform_record

/// Store a scalar and three two-word vectors into a 28-byte record.
/// `scalar` is copied to `+0`; `vector_a`, `vector_b` and `vector_c` supply
/// two dwords each for offsets `+4..+8`, `+0x0C..+0x10` and `+0x14..+0x18`.
/// Return `this`; the callee removes all four stack arguments.

use crate::support::{read_u32, write_u32};

lf_checker_rt::export!(thiscall, rw_00D9CA00(this: u32, scalar: u32, vector_a: u32, vector_b: u32, vector_c: u32) -> u32 {
    unsafe {
        write_u32(this, scalar);
        for (dst, src) in [(4, vector_a), (8, vector_a), (0x0C, vector_b), (0x10, vector_b), (0x14, vector_c), (0x18, vector_c)] {
            let input_offset = if dst == 4 || dst == 0x0C || dst == 0x14 { 0 } else { 4 };
            write_u32(this.wrapping_add(dst), read_u32(src.wrapping_add(input_offset)));
        }
        this
    }
});
