// original: 0x008a2f80 voice_state_query
/// Voice state query for a sound object (cdecl).
///
/// Resolves the slot voice; when flag bit 3 is set and a voice is live it
/// prods the voice (thiscall/0, stubbed). With no voice it returns 0, except
/// that a live slot with a null voice returns the table base with its low
/// byte cleared (the original's EAX still holds the table address when it
/// zeroes AL); otherwise it queries the voice (thiscall/1, stubbed) and
/// returns that answer.
export!(cdecl, rw_008a2f80(obj: u32, a2: u32) -> u32 {
    unsafe {
        let o = obj as *const u8;
        let slot = *o.add(0x48);
        let voice = if slot == SLOT_EMPTY {
            0
        } else {
            voice_ptr(*o.add(0x40), slot)
        };
        if *o.add(0x39) & 8 != 0 && voice != 0 {
            callee_thiscall!(1, u32, voice);
        }
        if voice == 0 {
            if slot == SLOT_EMPTY {
                return 0;
            }
            return *global::<u32>(VOICE_TABLE_GV) & 0xFFFFFF00;
        }
        callee_thiscall!(2, u32, voice, a2)
    }
});
