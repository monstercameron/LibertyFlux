// original: 0x00B351E0 linear_insert_28_key0

/// Insert the 28-byte `value` into the run ending at `hole`, shifting down
/// every element whose offset-0 key is strictly above the value key.
///
/// Twin of `rw_00B35120` with the key at the element start and the operands
/// swapped: the scan continues while the stored key is strictly greater
/// than the value key, so the stopper is a key at or below it. Cdecl, eight
/// stack words, no meaningful return value.
///
/// Original: 0x00B351E0.

lf_checker_rt::export!(
    cdecl,
    rw_00B351E0(hole: u32, v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32, v6: u32) -> u32 {
        unsafe {
            const STRIDE: u32 = 28;
            let val = [v0, v1, v2, v3, v4, v5, v6];
            let vkey = f32::from_bits(v0);
            let mut dst = hole;
            let mut src = hole.wrapping_sub(STRIDE);
            loop {
                let skey = (src as *const f32).read_unaligned();
                if !(skey > vkey) {
                    break;
                }
                for i in 0..7u32 {
                    let w = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
                    (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
                }
                dst = src;
                src = src.wrapping_sub(STRIDE);
            }
            for i in 0..7u32 {
                (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(val[i as usize]);
            }
            0
        }
    }
);
