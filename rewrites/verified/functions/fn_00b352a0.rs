// original: 0x00B352A0 linear_insert_16

/// Insert the 16-byte `value` into the run ending at `hole`, shifting down
/// every element whose offset-0 key is strictly above the value key.
///
/// 16-byte-element twin of `rw_00B351E0`. Cdecl, five stack words, no
/// meaningful return value.
///
/// Original: 0x00B352A0.

lf_checker_rt::export!(
    cdecl,
    rw_00B352A0(hole: u32, v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
        unsafe {
            const STRIDE: u32 = 16;
            let val = [v0, v1, v2, v3];
            let vkey = f32::from_bits(v0);
            let mut dst = hole;
            let mut src = hole.wrapping_sub(STRIDE);
            loop {
                let skey = (src as *const f32).read_unaligned();
                if !(skey > vkey) {
                    break;
                }
                for i in 0..4u32 {
                    let w = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
                    (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
                }
                dst = src;
                src = src.wrapping_sub(STRIDE);
            }
            for i in 0..4u32 {
                (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(val[i as usize]);
            }
            0
        }
    }
);
