// original: 0x008a3160 multitrack_slot_sweep
/// Slot sweep driving each live voice through its update pair.
///
/// Walks `count` slots from [this+0x48]; live voices go through the update
/// pair (thiscall/2 then thiscall/1, stubbed) while empty or null-voice
/// slots are skipped. Returns the slot count (the loop-bottom address math
/// leaves it in EAX, discarding every callee answer), or the saved position
/// word when the count is zero and the loop never runs.
export!(thiscall, rw_008a3160(this: u32, a1: u32) -> u32 {
    unsafe {
        let o = this as *const u8;
        let count = *(o.add(0xb0) as *const u32);
        let saved = *(o.add(0x54) as *const u32);
        let bank = *o.add(0x40);
        if count == 0 {
            return saved;
        }
        let mut i = 0u32;
        while i < count {
            let slot = *o.add(0x48 + (i as usize));
            i += 1;
            if slot == SLOT_EMPTY {
                continue;
            }
            let v = voice_ptr(bank, slot);
            if v == 0 {
                continue;
            }
            let _r1: u32 = callee_thiscall!(1, u32, v, saved, 0);
            let v2 = voice_ptr(bank, slot);
            let _r2: u32 = callee_thiscall!(2, u32, v2, a1);
        }
        count
    }
});
