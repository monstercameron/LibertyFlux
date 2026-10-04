// original: 0x0089E310 aud_seq_rebuild_voices
// ---------------------------------------------------------------------------
// 0x0089E310: rebuild this sequential sound's voice list. Asks the counter
// how many slots are live, releases and clears each live slot while
// notifying the sound per slot, then primes and binds the newly selected
// voice. Returns the bind result, or the counter result when there is
// nothing live, 0xFF when the selected slot is empty, or the bank address
// when the selected voice is unmapped.
// (The voice-table stride is pinned to 0 so every slot maps onto the one
// fabricated voice object; the release entry is a planted recorder stub.
// The counter also returns a second word that becomes a bind argument.)
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089E310(this_ptr: u32, arg: u32) -> u32 {
    const SLOT_EMPTY: u8 = 0xFF;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
    let bank_table = unsafe { *global::<u32>(0x115D988) };
    let stride = unsafe { *global::<u32>(0x115D964) };
    let map_voice = |slot: u8| -> u32 {
        let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
        let row = unsafe {
            *((bank_table
                .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
                .wrapping_add(BANK_BASE_OFF)) as *const u32)
        };
        row.wrapping_add(stride.wrapping_mul(slot as u32))
    };
    let mut live: u32 = 0;
    let mut scratch: u32 = 0;
    let first = callee_thiscall!(
        1,
        u32,
        this_ptr,
        &mut live as *mut u32 as u32,
        &mut scratch as *mut u32 as u32
    );
    let count = live as i32;
    if count < 0 {
        return first;
    }
    unsafe { *((this_ptr.wrapping_add(0xCC)) as *mut u32) = live };
    if count > 0 {
        for i in 0..count {
            let at = this_ptr.wrapping_add(i as u32).wrapping_add(0x48);
            let slot = unsafe { *(at as *const u8) };
            if slot != SLOT_EMPTY {
                let voice = map_voice(slot);
                if voice != 0 {
                    let vtable = unsafe { *(voice as *const u32) };
                    let release: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                        core::mem::transmute(*((vtable.wrapping_add(0x14)) as *const u32))
                    };
                    release(voice, 1);
                    unsafe { *(at as *mut u8) = SLOT_EMPTY };
                }
            }
            callee_thiscall!(3, u32, this_ptr, i as u32);
        }
    }
    let slot = unsafe { *((this_ptr.wrapping_add(count as u32).wrapping_add(0x48)) as *const u8) };
    if slot == SLOT_EMPTY {
        return 0xFF;
    }
    let voice = map_voice(slot);
    if voice == 0 {
        return bank_table;
    }
    callee_thiscall!(4, u32, voice, scratch, 0);
    callee_thiscall!(5, u32, voice, arg)
});
