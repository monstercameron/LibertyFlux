// original: 0x00698130 rage::crCreatureComponentMover::vf3
/// Gather twelve words from the mover's pose buffer into the component.
///
/// Copies three words out of every four (skipping every fourth word) from
/// the buffer at offset 0xc into the matching slotted offsets 0x10..0x48,
/// which skip 0x1c, 0x2c and 0x3c the same way. The stack argument is
/// ignored. Returns the last word copied.
export!(thiscall, rw_00698130(this: u32, _arg: u32) -> u32 {
    unsafe {
        let src = *((this as *const u32).add(3)) as *const u32;
        let dst = (this as *mut u32).add(4);
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
