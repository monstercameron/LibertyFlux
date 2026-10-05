// original: 0x00B34DF0 push_heap_28_key12

/// Sift the 28-byte `value` (passed by value as seven words) up a min-heap
/// ordered by the float key at offset 12.
///
/// `base` points at the element array (28 bytes each), `hole` is the index
/// of the empty slot and `top` the index sifting stops above. While the hole
/// is above `top` and the parent key is strictly greater than the value key
/// (`comiss`/`jbe`: an unordered/NaN comparison ends the sift), the parent
/// element moves down into the hole. The value is then stored into the final
/// hole. Cdecl, ten stack words, no meaningful return value.
///
/// Original: 0x00B34DF0.

lf_checker_rt::export!(
    cdecl,
    rw_00B34DF0(
        base: u32,
        hole: u32,
        top: u32,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
        v4: u32,
        v5: u32,
        v6: u32
    ) -> u32 {
        unsafe {
            const STRIDE: u32 = 28;
            const KEY_OFF: u32 = 12;
            let val = [v0, v1, v2, v3, v4, v5, v6];
            let vkey = f32::from_bits(v3);
            let mut h = hole as i32;
            let t = top as i32;
            let mut parent = h.wrapping_sub(1) / 2;
            if h > t {
                loop {
                    let p = base.wrapping_add((parent as u32).wrapping_mul(STRIDE));
                    let pkey = (p.wrapping_add(KEY_OFF) as *const f32).read_unaligned();
                    if !(pkey > vkey) {
                        break;
                    }
                    let dst = base.wrapping_add((h as u32).wrapping_mul(STRIDE));
                    for i in 0..7u32 {
                        let w = (p.wrapping_add(i * 4) as *const u32).read_unaligned();
                        (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
                    }
                    h = parent;
                    parent = parent.wrapping_sub(1) / 2;
                    if h <= t {
                        break;
                    }
                }
            }
            let dst = base.wrapping_add((h as u32).wrapping_mul(STRIDE));
            for i in 0..7u32 {
                (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(val[i as usize]);
            }
            0
        }
    }
);
