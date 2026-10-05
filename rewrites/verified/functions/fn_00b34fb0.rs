// original: 0x00B34FB0 push_heap_16

/// Sift the 16-byte `value` up a max-heap ordered by the float key at
/// offset 0.
///
/// 16-byte-element twin of `rw_00B34ED0`: sifting continues while the value
/// key is strictly greater than the parent key. Cdecl, seven stack words,
/// no meaningful return value.
///
/// Original: 0x00B34FB0.

lf_checker_rt::export!(
    cdecl,
    rw_00B34FB0(base: u32, hole: u32, top: u32, v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
        unsafe {
            const STRIDE: u32 = 16;
            let val = [v0, v1, v2, v3];
            let vkey = f32::from_bits(v0);
            let mut h = hole as i32;
            let t = top as i32;
            let mut parent = h.wrapping_sub(1) / 2;
            if h > t {
                loop {
                    let p = base.wrapping_add((parent as u32).wrapping_mul(STRIDE));
                    let pkey = (p as *const f32).read_unaligned();
                    if !(vkey > pkey) {
                        break;
                    }
                    let dst = base.wrapping_add((h as u32).wrapping_mul(STRIDE));
                    for i in 0..4u32 {
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
            for i in 0..4u32 {
                (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(val[i as usize]);
            }
            0
        }
    }
);
