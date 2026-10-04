// original: 0x0089E010 rage::audWrapperSound::vf7
// ---------------------------------------------------------------------------
// 0x0089E010 rage::audWrapperSound::vf7: attach a wrapped voice. Runs the
// base accept, locates the voice record through the pool, and converts the
// record address back into a slot id by subtracting the bank base and
// dividing by the stride. Returns 1 with the slot latched, or 1 with the
// slot cleared when no record is found, or the base answer on early reject.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089E010(this_ptr: u32, a: u32, b: u32, c: u32) -> u32 {
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
    let ok = callee_thiscall!(1, u32, this_ptr, a, b, c);
    if ok & 0xFF == 0 {
        return ok;
    }
    let rec = unsafe { *((this_ptr.wrapping_add(0x94)) as *const u32) };
    let key = unsafe { *(rec as *const u32) };
    let found = callee_thiscall!(2, u32, relocated(0x115DC18), key, this_ptr, b, c);
    if found == 0 {
        unsafe { *((this_ptr.wrapping_add(0x48)) as *mut u8) = 0xFF };
        return 1;
    }
    let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
    let bank_table = unsafe { *global::<u32>(0x115D988) };
    let stride = unsafe { *global::<u32>(0x115D964) };
    let row = unsafe {
        *((bank_table
            .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
            .wrapping_add(BANK_BASE_OFF)) as *const u32)
    };
    let slot = found.wrapping_sub(row) / stride;
    unsafe { *((this_ptr.wrapping_add(0x48)) as *mut u8) = slot as u8 };
    1
});
