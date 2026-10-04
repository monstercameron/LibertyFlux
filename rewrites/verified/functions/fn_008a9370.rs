// original: 0x008a9370 rage::audEffect::vf5
/// `rage::audEffect::vf5`: rotate one row of the slot table.
///
/// Copies entries from row `count` to row `(count + 1) % 3` of the five-wide
/// slot table at `this+0x34`, where `count` is the dword at `this+0x30`.
/// The loop bound is the byte at `this+0x70`, which sits at table index 15,
/// inside the destination rows: overlapping writes change the bound
/// mid-loop, so it is re-read every iteration exactly like the original.
/// Returns the last value moved, or the division quotient when no iteration
/// runs (matching exit EAX).
export!(thiscall, rw_008a9370(this: *mut u8) -> u32 {
    unsafe {
        let slots = this.add(0x34) as *mut u32;
        let count = *(this.add(0x30) as *const u32);
        let dividend = count.wrapping_add(1);
        let quotient = dividend / 3;
        let dst_base = (dividend % 3).wrapping_mul(5);
        let src_base = count.wrapping_mul(5);
        let mut last = quotient;
        let mut bl = 0u8;
        loop {
            let limit = *this.add(0x70);
            if bl >= limit {
                break;
            }
            let v = *slots.add(src_base.wrapping_add(bl as u32) as usize);
            *slots.add(dst_base.wrapping_add(bl as u32) as usize) = v;
            last = v;
            bl = bl.wrapping_add(1);
        }
        last
    }
});
