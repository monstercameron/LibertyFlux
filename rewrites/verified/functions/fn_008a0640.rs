// original: 0x008a0640 rage::audRetriggeredOverlappedSound::vf9
/// Voice retrigger of `rage::audRetriggeredOverlappedSound` (vf9).
///
/// Resolves the voice for the indexed slot [idx+this+0x48]; returns 0xff
/// for an empty slot and 0 for a null voice. Otherwise scales a pitch word
/// through a helper (cdecl/1,
/// stubbed) and forwards the result plus a flag word to the voice handler
/// (thiscall/3, stubbed). The flag word is the index argument with its low
/// byte replaced by bit 5 of the object's flag byte (the original stores
/// that bit into the argument's stack slot), derived exactly here.
export!(thiscall, rw_008a0640(this: u32, _a1: u32, idx: u32) -> u32 {
    unsafe {
        let o = this as *const u8;
        let slot_at = |i: u32| *(((i.wrapping_add(this)).wrapping_add(0x48)) as *const u8);
        let slot = slot_at(idx);
        if slot == SLOT_EMPTY {
            // EAX still holds the 0xff slot byte on this exit.
            return 0xff;
        }
        let bank = *o.add(0x40);
        if voice_ptr(bank, slot) == 0 {
            return 0;
        }
        let bit = ((*o.add(0x39) >> 5) & 1) as u32;
        let flag_word = (idx & 0xFFFFFF00) | bit;
        let pitch = *(o.add(0x3c) as *const i16) as i32 as u32;
        let scaled: u32 = callee_cdecl!(1, u32, pitch);
        let slot2 = slot_at(idx);
        // Unreachable 0xff arm (the stub cannot change the slot), kept for shape.
        let v2 = if slot2 == SLOT_EMPTY { 0 } else { voice_ptr(bank, slot2) };
        callee_thiscall!(2, u32, v2, scaled, flag_word, 0)
    }
});
