// original: 0x0089DEE0 aud_voice_prime_bind2_option
// ---------------------------------------------------------------------------
// 0x0089DEE0: start this sound's voice with two arguments plus the sound's
// option bit (bit 5 of +0x39). Returns 1 when there is no voice, else the
// bind result.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089DEE0(this_ptr: u32, arg1: u32, arg2: u32) -> u32 {
    const SLOT_EMPTY: u8 = 0xFF;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
    let map_voice = |this_ptr: u32| -> u32 {
        let slot = unsafe { *((this_ptr.wrapping_add(0x48)) as *const u8) };
        if slot == SLOT_EMPTY {
            return 0;
        }
        let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
        let bank_table = unsafe { *global::<u32>(0x115D988) };
        let stride = unsafe { *global::<u32>(0x115D964) };
        let row = unsafe {
            *((bank_table
                .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
                .wrapping_add(BANK_BASE_OFF)) as *const u32)
        };
        row.wrapping_add(stride.wrapping_mul(slot as u32))
    };
    let slot = unsafe { *((this_ptr.wrapping_add(0x48)) as *const u8) };
    if slot == SLOT_EMPTY {
        return 1;
    }
    let primed = map_voice(this_ptr);
    if primed == 0 {
        return 1;
    }
    let param = unsafe { *((this_ptr.wrapping_add(0x54)) as *const u32) };
    callee_thiscall!(1, u32, primed, param, 0);
    // The option bit is stored into the low byte of the pushed-ECX slot and
    // the whole word is forwarded: this with its low byte replaced by the bit.
    let option = (unsafe { *((this_ptr.wrapping_add(0x39)) as *const u8) } >> 5) & 1;
    let bound = map_voice(this_ptr);
    let slot_word = (this_ptr & 0xFFFF_FF00) | option as u32;
    callee_thiscall!(2, u32, bound, arg1, slot_word, arg2)
});
