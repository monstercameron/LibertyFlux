// original: 0x006985b0 rage::crCreatureComponentMover::vf8
/// Scatter twelve component words back into the mover's pose buffer.
///
/// Inverse of [`rw_00698130`]: reads the slotted offsets 0x10..0x48
/// (skipping 0x1c, 0x2c, 0x3c) and writes three words out of every four of
/// the buffer at offset 0xc. Returns the last word written.
export!(thiscall, rw_006985b0(this: u32) -> u32 {
    unsafe {
        let src = (this as *const u32).add(4);
        let dst = *((this as *const u32).add(3)) as *mut u32;
        let mut last = 0u32;
        for row in 0..4usize {
            for k in 0..3usize {
                last = *src.add(row * 4 + k);
                *dst.add(row * 4 + k) = last;
            }
        }
        last
    }
});
