// original: 0x005D2BD0 blip_slot_assign

/// Reinitialize a blip slot and copy its four-word payload.
///
/// `this` is the destination slot. The four stack arguments are the attached
/// object, a byte-sized category, flag bits, and a pointer to a four-word
/// payload. The existing slot is detached first; the parent list is then
/// notified with the slot pointer. The payload is copied as raw words so
/// floating-point fields retain every bit. The low three bits at offset
/// `0x11` are replaced by the low three bits of the flag argument. The
/// thiscall callee removes all four stack arguments and the function returns
/// the fourth payload word.
lf_checker_rt::export!(thiscall, rw_005d2bd0(this: u32, attached_object: u32, category: u32, flags: u32, payload: u32) -> u32 {
    unsafe {
        const SLOT_CATEGORY: usize = 0x10;
        const SLOT_FLAGS: usize = 0x11;
        const SLOT_PAYLOAD: usize = 0x20;
        const SLOT_LINK_WORD: usize = 0x40;

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

        let slot = this as *mut u8;
        (slot as *mut u32).write_unaligned(attached_object);
        (slot.add(SLOT_LINK_WORD) as *mut u16).write_unaligned(0);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, attached_object, this);

        slot.add(SLOT_CATEGORY).write(category as u8);
        let old_flags = slot.add(SLOT_FLAGS).read();
        let changed_bits = (old_flags ^ flags as u8) & 0x07;
        slot.add(SLOT_FLAGS).write(old_flags ^ changed_bits);

        let source = payload as *const u32;
        let mut returned_word = 0;
        for word in 0..4usize {
            returned_word = source.add(word).read_unaligned();
            (slot.add(SLOT_PAYLOAD + word * 4) as *mut u32).write_unaligned(returned_word);
        }
        slot.add(0x30).write(0);
        returned_word
    }
});
