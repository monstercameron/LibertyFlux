// original: 0x008a08d0 indexed_voice_poke
/// Indexed voice poke with follow-up reconfiguration.
///
/// When the indexed slot holds a live voice, notifies it (thiscall/0,
/// stubbed) and runs the per-slot reconfiguration (thiscall/1, stubbed),
/// then always runs the trailing reconfiguration (thiscall/3, stubbed) and
/// returns its answer.
export!(thiscall, rw_008a08d0(this: u32, idx: u32) -> u32 {
    unsafe {
        let o = this as *const u8;
        let slot = *(((idx.wrapping_add(this)).wrapping_add(0x48)) as *const u8);
        if slot != SLOT_EMPTY {
            let bank = *o.add(0x40);
            let voice = voice_ptr(bank, slot);
            if voice != 0 {
                // Unreachable 0xff arm, kept so both sides agree if reached.
                let v = if slot == SLOT_EMPTY { 0 } else { voice };
                callee_thiscall!(1, u32, v);
                callee_thiscall!(2, u32, this, idx);
            }
        }
        let tail = *(o.add(0xd4) as *const u32);
        callee_thiscall!(3, u32, this, tail, idx, this.wrapping_add(0xb0))
    }
});
