// original: 0x0089E0D0 aud_voice_work_or_clear
// ---------------------------------------------------------------------------
// 0x0089E0D0: run the voice worker for a sound object, or clear the caller's
// flag byte when the voice slot is empty or unmapped. Returns the worker's
// answer, or 0 on the early paths.
// ---------------------------------------------------------------------------
export!(cdecl, rw_0089E0D0(obj: u32, flag_out: u32) -> u32 {
    const SLOT_EMPTY: u8 = 0xFF;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
    let slot = unsafe { *((obj.wrapping_add(0x48)) as *const u8) };
    let ptr = if slot == SLOT_EMPTY {
        0
    } else {
        let bank_idx = unsafe { *((obj.wrapping_add(0x40)) as *const u8) } as u32;
        let bank_table = unsafe { *global::<u32>(0x115D988) };
        let stride = unsafe { *global::<u32>(0x115D964) };
        let row = unsafe {
            *((bank_table
                .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
                .wrapping_add(BANK_BASE_OFF)) as *const u32)
        };
        row.wrapping_add(stride.wrapping_mul(slot as u32))
    };
    if ptr == 0 {
        if flag_out != 0 {
            unsafe { *(flag_out as *mut u8) = 0 };
        }
        return 0;
    }
    callee_thiscall!(1, u32, ptr, flag_out)
});
