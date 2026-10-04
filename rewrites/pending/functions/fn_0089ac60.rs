// original: 0x0089ac60 audio_voice_probe (proposed name)
/// Probe one audio voice's output level, returning it as a byte.
///
/// `obj` is the voice object, `arg1` an opaque parameter. A missing bank
/// record fails the probe with zero. Otherwise the prepare call decides:
/// a zero answer releases the record and returns zero, while a nonzero
/// answer multiplies the three gain factors, remaps the product through a
/// curve call, pushes it with the record into the level call, and returns
/// the route call's answer.
export!(thiscall, rw_0089ac60(obj: *mut u8, arg1: u32) -> u32 {
    unsafe {
        let gtable = *global::<u32>(0x0115d988);
        let stride = *global::<u32>(0x0115d964);
        let slot = *obj.add(0x48) as u32;
        if slot == 0xff {
            return 0;
        }
        let bank = *obj.add(0x40) as u32;
        let row = *((gtable as *const u8)
            .add(bank.wrapping_mul(0x6f40).wrapping_add(0x6f10) as usize)
            as *const u32);
        let rec = stride.wrapping_mul(slot).wrapping_add(row);
        if rec == 0 {
            return 0;
        }
        // The original re-reads the bank bytes before each use; nothing
        // between the reads can change them, so one record serves all uses.
        let prepared = callee_thiscall!(1, u32, obj as u32, arg1);
        if (prepared as u8) == 0 {
            callee_thiscall!(2, u32, rec);
            return 0;
        }
        let gain = *(obj.add(0xb4) as *const f32)
            * *(obj.add(0xb0) as *const f32)
            * *(obj.add(0xb8) as *const f32);
        let curved: f32 = callee_cdecl!(3, f32, gain.to_bits());
        callee_thiscall!(4, u32, rec, curved.to_bits());
        let routed = callee_thiscall!(5, u32, rec, arg1);
        (routed as u8) as u32
    }
});
