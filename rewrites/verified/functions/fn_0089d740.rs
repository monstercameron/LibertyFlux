// original: 0x0089d740 table_call_then_tail_jump
/// Guarded table dispatch: check twice, notify, then hand off.
///
/// Takes the object pointer in ECX and an unused word on the stack. When
/// the object's kind byte reads all set, the entry accumulator is the
/// result, so the contract pins it. When the scaled sum reads zero, the
/// result is zero. Otherwise a two-word notification is issued and the
/// sum is recomputed and forwarded to the shared successor, returning
/// whatever that call answers. The kind re-check and the post-call null
/// path cannot fire in this model: the entry already excluded the all-set
/// value and the stub writes nothing.
export!(thiscall, rw_0089d740(this: u32, _unused: u32) -> u32 {
    unsafe {
        let kind = *(this.wrapping_add(0x48) as *const u8);
        if kind == 0xFF {
            return 0xA5A5A5A5;
        }
        let row = *(this.wrapping_add(0x40) as *const u8);
        let scale = *(global::<u32>(0x0115D964) as *const u32);
        let base = *(global::<u32>(0x0115D988) as *const u32);
        let slot = (row as u32).wrapping_mul(0x6F40);
        let word = *((base.wrapping_add(slot).wrapping_add(0x6F10)) as *const u32);
        let sel = scale.wrapping_mul(kind as u32).wrapping_add(word);
        if sel == 0 {
            return 0;
        }
        let tag = *(this.wrapping_add(0x54) as *const u32);
        callee_stdcall!(2, u32, tag, 0);
        let kind2 = *(this.wrapping_add(0x48) as *const u8);
        if kind2 == 0xFF {
            return callee_thiscall!(1, u32, 0);
        }
        let row2 = *(this.wrapping_add(0x40) as *const u8);
        let slot2 = (row2 as u32).wrapping_mul(0x6F40);
        let word2 = *((base.wrapping_add(slot2).wrapping_add(0x6F10)) as *const u32);
        let sel2 = scale.wrapping_mul(kind2 as u32).wrapping_add(word2);
        callee_thiscall!(1, u32, sel2)
    }
});
