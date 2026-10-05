// original: 0x00986C30 audEmitter_clear_slots (proposed)

/// Emitter slot reset: zeroes every slot's head word and a trailing block.
///
/// Reads the slot count (UNSIGNED) at `+COUNT_OFF` of `this` and stores 0 to
/// the head word of each slot, slots starting at `+SLOT0_OFF` and spaced
/// `SLOT_STRIDE` bytes apart. Then stores 0 to `BLOCK_WORDS` dwords starting
/// at `+BLOCK_OFF`. No meaningful return value.
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00986C30(this: u32) -> u32 {
    const COUNT_OFF: u32 = 0x8230;
    const SLOT0_OFF: u32 = 0xd4;
    const SLOT_STRIDE: u32 = 0xd0;
    const BLOCK_OFF: u32 = 0x38248;
    const BLOCK_WORDS: u32 = 0x800;
    unsafe {
        let count = ((this + COUNT_OFF) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let addr = this
                .wrapping_add(SLOT0_OFF)
                .wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            (addr as *mut u32).write_unaligned(0);
            i = i.wrapping_add(1);
        }
        let mut p = this.wrapping_add(BLOCK_OFF);
        let mut n = 0u32;
        while n < BLOCK_WORDS {
            (p as *mut u32).write_unaligned(0);
            p = p.wrapping_add(4);
            n = n.wrapping_add(1);
        }
    }
    0
});
