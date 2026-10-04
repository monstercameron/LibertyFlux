// original: 0x00b406f0 array_segment_copy_down
/// Copy a dword run down within an object buffer and shrink its count.
///
/// Words are copied one dword at a time from `src` up to (but not including)
/// `base + count * 4` and stored at the same offset from `dst`; the object's
/// 16-bit count is then reduced by `(src - dst) / 4`. Returns `dst`.
export!(thiscall, rw_b406f0(obj: u32, dst: u32, src: u32) -> u32 {
    unsafe {
        let base = (obj as *const u32).read();
        let count = (obj as *const u16).byte_add(4).read();
        let end = base.wrapping_add((count as u32).wrapping_mul(4));
        if src != end {
            let delta = dst.wrapping_sub(src);
            let mut p = src;
            loop {
                (delta.wrapping_add(p) as *mut u32).write((p as *const u32).read());
                p = p.wrapping_add(4);
                if p == end {
                    break;
                }
            }
        }
        let n = (src.wrapping_sub(dst) as i32).wrapping_shr(2);
        let slot = (obj as *mut u16).byte_add(4);
        slot.write(slot.read().wrapping_sub(n as u16));
        dst
    }
});
