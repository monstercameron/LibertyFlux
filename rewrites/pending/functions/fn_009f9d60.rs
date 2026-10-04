// original: 0x009f9d60 byte_block_slide
/// Byte-block slide with length adjust (thiscall/2 -> eax).
///
/// Copies the bytes from `src` up to `base + len` (base and length stored at
/// this+0/+4) forward to `dst`, then adjusts the stored length by `dst - src`.
/// Address arithmetic wraps at 32 bits exactly like the original's. Returns
/// `dst`. The caller must pass `src <= base + len`; anything else loops
/// forever in the original too.
export!(thiscall, rw_s18f14(this: *mut u8, dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let base = *(this as *const u32);
        let len = *(this.add(4) as *const u16);
        let end = base.wrapping_add(len as u32);
        let delta = (dst as u32).wrapping_sub(src as u32);
        if (src as u32) != end {
            let mut p = src as u32;
            while p != end {
                *((delta.wrapping_add(p)) as *mut u8) = *(p as *const u8);
                p = p.wrapping_add(1);
            }
        }
        let countp = this.add(4) as *mut u16;
        *countp = (*countp).wrapping_add(delta as u16);
        dst as u32
    }
});
