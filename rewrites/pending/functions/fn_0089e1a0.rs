// original: 0x0089E1A0 rage::audSequentialSound::audSequentialSound_2
// ---------------------------------------------------------------------------
// 0x0089E1A0 rage::audSequentialSound second constructor: stamp the vtable,
// release every live voice slot (releasing through the voice's own release
// entry), clear the slots, release the latched selector, then tail into the
// base sound constructor. Returns the base constructor's result.
// (The voice-table stride is pinned to 0 by the contract so every slot maps
// onto the one fabricated voice object; the release entry is intercepted by
// planting the recorder address in the fabricated vtable.)
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089E1A0(this_ptr: u32) -> u32 {
    const SLOT_EMPTY: u8 = 0xFF;
    const SLOT_COUNT: u32 = 8;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
    unsafe { *(this_ptr as *mut u32) = relocated(0xE7A2EC) };
    let bank_table = unsafe { *global::<u32>(0x115D988) };
    let stride = unsafe { *global::<u32>(0x115D964) };
    for k in 0..SLOT_COUNT {
        let at = this_ptr.wrapping_add(k).wrapping_add(0x48);
        let slot = unsafe { *(at as *const u8) };
        if slot == SLOT_EMPTY {
            continue;
        }
        let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
        let row = unsafe {
            *((bank_table
                .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
                .wrapping_add(BANK_BASE_OFF)) as *const u32)
        };
        let voice = row.wrapping_add(stride.wrapping_mul(slot as u32));
        if voice == 0 {
            continue;
        }
        // Release through the voice's own vtable entry, exactly like the
        // original: both sides land on the same planted recorder stub.
        let vtable = unsafe { *(voice as *const u32) };
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(*((vtable.wrapping_add(0x14)) as *const u32)) };
        release(voice, 1);
        unsafe { *(at as *mut u8) = SLOT_EMPTY };
    }
    let sel = unsafe { *((this_ptr.wrapping_add(0xD1)) as *const u8) };
    if sel != SLOT_EMPTY {
        let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
        let row = unsafe {
            *((bank_table
                .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
                .wrapping_add(BANK_BASE_OFF)) as *const u32)
        };
        let voice = row.wrapping_add(stride.wrapping_mul(sel as u32));
        callee_thiscall!(2, u32, relocated(0x115D8A0), voice, bank_idx);
        unsafe { *((this_ptr.wrapping_add(0xD1)) as *mut u8) = SLOT_EMPTY };
    }
    callee_thiscall!(3, u32, this_ptr)
});
