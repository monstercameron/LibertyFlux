// original: 0x009f9da0 word_vector_slide
/// Word-vector slide, count at this+0x14 (thiscall/2 -> eax).
///
/// Moves the words from `src` up to `this + count*2` forward to `dst`, then
/// subtracts `(src-dst)/2` from the stored count. Reads the count once up
/// front but applies the final update to current memory, exactly like the
/// original (the regions may overlap the count field). Returns `dst`. The
/// caller must pass `src` inside the vector with matching parity; anything
/// else loops forever in the original too.
export!(thiscall, rw_s18f15(this: *mut u8, dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let countp = this.add(0x14) as *mut i32;
        let count = *countp;
        let end = (this as u32).wrapping_add((count as u32).wrapping_mul(2));
        if (src as u32) != end {
            let delta = (dst as u32).wrapping_sub(src as u32);
            let mut p = src as u32;
            while p != end {
                *((delta.wrapping_add(p)) as *mut u16) = *(p as *const u16);
                p = p.wrapping_add(2);
            }
        }
        // `sub; sar`: signed (src-dst) shifted arithmetically.
        let adj = ((src as u32).wrapping_sub(dst as u32) as i32) >> 1;
        *countp = (*countp).wrapping_sub(adj);
        dst as u32
    }
});
