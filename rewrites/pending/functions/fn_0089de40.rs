// original: 0x0089DE40 aud_voice_prime_bind_notify
// ---------------------------------------------------------------------------
// 0x0089DE40: start this sound's voice: map the slot, prime it with the
// sound's parameter (+0x54), bind the caller's argument, then notify the
// sound. Returns the notification result, or 0 when there is no voice.
// (Entry EAX pinned to 0: the empty-slot path returns it untouched.)
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089DE40(this_ptr: u32, arg: u32) -> u32 {
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
        return 0;
    }
    let primed = map_voice(this_ptr);
    if primed == 0 {
        return 0;
    }
    let param = unsafe { *((this_ptr.wrapping_add(0x54)) as *const u32) };
    callee_thiscall!(1, u32, primed, param, 0);
    let bound = map_voice(this_ptr);
    callee_thiscall!(2, u32, bound, arg);
    callee_thiscall!(3, u32, this_ptr)
});
