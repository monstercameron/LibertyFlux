// original: 0x009f9de0 dword_vector_slide
use lf_k2_rt::{export};

/// Dword-vector slide, count at this+0x5C (thiscall/2 -> eax).
///
/// Same shape as `rw_s18f15` with dword elements: moves [src, this+count*4)
/// to `dst`, then subtracts `(src-dst)/4` from the count. Returns `dst`.
export!(thiscall, rw_s18f16(this: *mut u8, dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let countp = this.add(0x5C) as *mut i32;
        let count = *countp;
        let end = (this as u32).wrapping_add((count as u32).wrapping_mul(4));
        if (src as u32) != end {
            let delta = (dst as u32).wrapping_sub(src as u32);
            let mut p = src as u32;
            while p != end {
                *((delta.wrapping_add(p)) as *mut u32) = *(p as *const u32);
                p = p.wrapping_add(4);
            }
        }
        let adj = ((src as u32).wrapping_sub(dst as u32) as i32) >> 2;
        *countp = (*countp).wrapping_sub(adj);
        dst as u32
    }
});
