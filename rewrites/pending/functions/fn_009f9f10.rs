// original: 0x009f9f10 slot_array_prefix_drop
/// Slot-array prefix drop: copy the dwords from `src` up to the used end
/// down to `dst`, shrink the used count by the dropped dwords, return `dst`.
export!(thiscall, rw_009f9f10(obj: u32, dst: u32, src: u32) -> u32 {
    const COUNT_OFF: usize = 0x84;
    unsafe {
        let countp = (obj as *mut u32).byte_add(COUNT_OFF);
        let end = (obj as usize).wrapping_add((*countp as usize).wrapping_mul(4));
        let mut cur = src as usize;
        while cur != end {
            let v = (cur as *const u32).read();
            (dst.wrapping_sub(src).wrapping_add(cur as u32) as *mut u32).write(v);
            cur = cur.wrapping_add(4);
        }
        let dropped = (src as i32).wrapping_sub(dst as i32) >> 2;
        *countp = (*countp).wrapping_sub(dropped as u32);
        dst
    }
});
