// original: 0x0089DFA0 aud_voice_test
// ---------------------------------------------------------------------------
// 0x0089DFA0: test this sound's voice with the argument. Returns false when
// the voice slot is empty or unmapped, else the callee's boolean result.
// (Entry EAX is pinned to 0 by the contract: the empty-slot path returns it
// untouched, which no rewrite could otherwise reproduce.)
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089DFA0(this_ptr: u32, arg: u32) -> u32 {
    const SLOT_EMPTY: u8 = 0xFF;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
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
    let ptr = row.wrapping_add(stride.wrapping_mul(slot as u32));
    if ptr == 0 {
        return 0;
    }
    let answer = callee_thiscall!(1, u32, ptr, arg);
    if answer & 0xFF != 0 { (answer & 0xFFFF_FF00) | 1 } else { answer & 0xFFFF_FF00 }
});
