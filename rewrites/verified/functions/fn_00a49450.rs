// original: 0x00a49450 vehicle_count_nonempty_slots
/// Count the nonzero dwords among the first `n` occupant slots.
///
/// `n` is the byte at `this+0x1070`; the slots are dwords starting at
/// `this+0xF54` (thiscall, no stack arguments). An `n` of zero returns 0
/// without reading any slot.
export!(thiscall, rw_00a49450(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x1070;
        const SLOTS_OFF: u32 = 0xf54;
        let n = ((this.wrapping_add(COUNT_OFF)) as *const u8).read() as u32;
        let mut count = 0u32;
        let mut p = this.wrapping_add(SLOTS_OFF);
        let mut left = n;
        while left != 0 {
            if (p as *const u32).read_unaligned() != 0 {
                count += 1;
            }
            p = p.wrapping_add(4);
            left -= 1;
        }
        count
    }
});
