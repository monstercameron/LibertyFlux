// original: 0x009863F0 audEmitter_slot_flag_check (proposed)

/// Slot flag test of the emitter audio entity: reports whether the 32-bit
/// flag word of one slot is nonzero.
///
/// `this` points at the entity, whose slots are `SLOT_STRIDE` bytes apart;
/// `index` (unsigned) selects the slot and `FLAG_OFF` is the flag word's
/// offset within a slot. Returns 1 when the word is nonzero, else 0.
/// The index multiply wraps mod 2^32, as the original's `imul` does.
/// Original: thiscall, one stack word, return in `al` with `eax` otherwise 0.
lf_checker_rt::export!(thiscall, rw_009863F0(this: u32, index: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0xd0;
    const FLAG_OFF: u32 = 0xc8;
    unsafe {
        let addr = this
            .wrapping_add(index.wrapping_mul(SLOT_STRIDE))
            .wrapping_add(FLAG_OFF);
        let word = (addr as *const u32).read_unaligned();
        (word != 0) as u32
    }
});
