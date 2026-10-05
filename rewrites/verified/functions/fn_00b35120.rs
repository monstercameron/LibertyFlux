// original: 0x00B35120 linear_insert_28_key12

/// Insert the 28-byte `value` (seven words by value) into the run ending at
/// `hole`, shifting down every element whose offset-12 key is strictly below
/// the value key.
///
/// Unguarded linear insert: the caller guarantees a stopper element (a key
/// at or above the value key, or an unordered/NaN comparison, either of
/// which ends the scan). The value lands in the freed slot. Cdecl, eight
/// stack words, no meaningful return value.
///
/// Original: 0x00B35120 (true size 177; the batch list truncates it at 170).

lf_checker_rt::export!(
    cdecl,
    rw_00B35120(hole: u32, v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32, v6: u32) -> u32 {
        unsafe {
            const STRIDE: u32 = 28;
            const KEY_OFF: u32 = 12;
            let val = [v0, v1, v2, v3, v4, v5, v6];
            let vkey = f32::from_bits(v3);
            let mut dst = hole;
            let mut src = hole.wrapping_sub(STRIDE);
            loop {
                let skey = (src.wrapping_add(KEY_OFF) as *const f32).read_unaligned();
                if !(vkey > skey) {
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
