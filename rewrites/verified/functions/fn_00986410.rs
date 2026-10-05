// original: 0x00986410 audEmitter_slot_set_byte (proposed)

/// Slot byte store of the emitter audio entity: writes one byte into a slot.
///
/// `this` points at the entity, whose slots are `SLOT_STRIDE` bytes apart;
/// `index` (unsigned) selects the slot and `VALUE_OFF` is the target byte's
/// offset within a slot. Only the low byte of `value` is stored; the upper
/// bytes are ignored. No meaningful return value.
/// Original: thiscall, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00986410(this: u32, index: u32, value: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0xd0;
    const VALUE_OFF: u32 = 0xf6;
    unsafe {
        let addr = this
            .wrapping_add(index.wrapping_mul(SLOT_STRIDE))
            .wrapping_add(VALUE_OFF);
        (addr as *mut u8).write(value as u8);
    }
    0
});
