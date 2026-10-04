// original: 0x008a0150 retriggered_voice_dispatch
/// Voice dispatch for a retriggered overlapped sound.
///
/// Looks up the voice for slot [this+0x48]; returns 2 when there is none.
/// Otherwise forwards the two arguments plus a flag word to the voice
/// handler (thiscall/3, stubbed) and returns its answer. The flag word's low
/// byte is bit 5 of the object's flag byte; its upper three bytes are the
/// this-pointer's own upper bytes (the original spills ECX then overwrites
/// only the slot's low byte), so the rewrite derives the exact same word.
export!(thiscall, rw_008a0150(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let o = this as *const u8;
        let slot = *o.add(0x48);
        if slot == SLOT_EMPTY {
            return 2;
        }
        let bank = *o.add(0x40);
        let voice = voice_ptr(bank, slot);
        if voice == 0 {
            return 2;
        }
        let bit = ((*o.add(0x39) >> 5) & 1) as u32;
        let flag_word = (this & 0xFFFFFF00) | bit;
        // The original re-tests the slot for 0xff here; unreachable (it
        // returned above), kept so both sides agree if ever reached.
        let v = if slot == SLOT_EMPTY { 0 } else { voice };
        callee_thiscall!(1, u32, v, a1, flag_word, a2)
    }
});
