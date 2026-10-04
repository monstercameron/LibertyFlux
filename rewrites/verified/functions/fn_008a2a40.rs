// original: 0x008a2a40 simple_voice_position_update
/// Position update for a simple sound voice.
///
/// Reads the voice length and loop flag through the slot voice, folds the
/// play position into the loop when looping, then drives the voice through
/// its update pair (thiscall/2 + thiscall/1, stubbed) and finalizes
/// (thiscall/0, stubbed). When the position runs past a non-looping voice
/// it releases the slot instead (thiscall/1, stubbed) and returns that
/// answer. A 0xff slot faults reading through the null voice, exactly like
/// the original (both sides fault identically).
export!(thiscall, rw_008a2a40(this: u32, a1: u32) -> u32 {
    unsafe {
        let o = this as *const u8;
        let slot = *o.add(0x48);
        let bank = *o.add(0x40);
        let mut pos = *(o.add(0x54) as *const u32);
        let v1 = voice_ptr(bank, slot);
        let is_loop = core::ptr::read_unaligned(((v1.wrapping_add(0xee))) as *const u8) & 4 != 0;
        let len = core::ptr::read_unaligned(((v1.wrapping_add(0xe0))) as *const u32);
        if is_loop {
            if (len as i32) > 0 {
                let q = (pos as i32).wrapping_div(len as i32);
                pos = (pos as i32).wrapping_sub(q.wrapping_mul(len as i32)) as u32;
            }
            let v2 = voice_ptr(bank, slot);
            if v2 == 0 {
                return 0; // dead: v1 was mapped so v2 == v1
            }
            callee_thiscall!(1, u32, v2, pos, 0);
            callee_thiscall!(2, u32, v2, a1);
            return callee_thiscall!(3, u32, this);
        }
        if (len as i32) >= (pos as i32) {
            let v2 = voice_ptr(bank, slot);
            if v2 == 0 {
                return 0; // dead: same reason
            }
            callee_thiscall!(1, u32, v2, pos, 0);
            callee_thiscall!(2, u32, v2, a1);
            return callee_thiscall!(3, u32, this);
        }
        let v3 = voice_ptr(bank, slot);
        if v3 == 0 {
            return 0; // dead: same reason
        }
        callee_thiscall!(4, u32, this, 0)
    }
});
