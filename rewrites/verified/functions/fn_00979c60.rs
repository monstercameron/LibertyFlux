// original: 0x00979c60 audio_bus_param_forward
/// Table selector: combine a scaled row stride with a table word, hand off.
///
/// Takes the object pointer in ECX. When the object's kind byte reads all
/// set, control transfers to the shared successor with a null selector.
/// Otherwise the selector is the scale value times the kind byte plus the
/// table word selected by the row byte, and that selector is forwarded
/// instead, returning whatever that call answers.
export!(thiscall, rw_00979c60(this: u32) -> u32 {
    unsafe {
        let kind = *(this.wrapping_add(4) as *const u8);
        if kind == 0xFF {
            return callee_thiscall!(1, u32, 0);
        }
        let row = *(this.wrapping_add(0x40) as *const u8);
        let scale = *(global::<u32>(0x0115D968) as *const u32);
        let base = *(global::<u32>(0x0115D988) as *const u32);
        let slot = (row as u32).wrapping_mul(0x6F40);
        let word = *((base.wrapping_add(slot).wrapping_add(0x6F14)) as *const u32);
        let sel = scale.wrapping_mul(kind as u32).wrapping_add(word);
        callee_thiscall!(1, u32, sel)
    }
});
