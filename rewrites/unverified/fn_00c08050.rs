// original: 0x00c08050 stream_array_make_room (proposed)

/// Shift the tail of the 80-byte-element array right to free one slot.
///
/// `this` points to the array (`ITEMS` its buffer, `COUNT` its 16-bit length).
/// Every element from `index` up to the length moves one slot higher through
/// the shifter (callee 1, destination in `ecx`, source on the stack, highest
/// first); when the length is already at or below `index` (signed compare)
/// nothing moves. Either way the length advances by one, wrapping past
/// 0xFFFF, and the pointer to slot `index` is returned.
///
/// Original: 0x00c08050 (thiscall, one stack word; callee is thiscall).
lf_checker_rt::export!(thiscall, rw_00c08050(this: u32, index: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const STRIDE: u32 = 80;
        const SHIFT: u32 = 1;
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if (count as i32) > (index as i32) {
            let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            let mut off = count.wrapping_mul(STRIDE);
            let mut n = count.wrapping_sub(index);
            while n != 0 {
                let dst = base.wrapping_add(off);
                let _r: u32 =
                    lf_checker_rt::callee_thiscall!(SHIFT, u32, dst, dst.wrapping_sub(STRIDE));
                off = off.wrapping_sub(STRIDE);
                n = n.wrapping_sub(1);
            }
        }
        let c = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        (this.wrapping_add(COUNT) as *mut u16).write_unaligned(c.wrapping_add(1) as u16);
        let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
        base.wrapping_add(index.wrapping_mul(STRIDE))
    }
});
