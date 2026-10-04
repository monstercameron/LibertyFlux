// original: 0x0089F040 rage::audStreamingSound::vf8
// ---------------------------------------------------------------------------
// 0x0089F040 rage::audStreamingSound::vf8: notify the base sound, then walk
// the voice slots (+0xB0, count at +0xD0) and notify each mapped voice.
// Slots past index 8 map to null. Returns the slot count (the walk counter
// sits in EAX), or the base result when the count is zero.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089F040(this_ptr: u32, arg: u32) -> u32 {
    const SLOT_EMPTY: u8 = 0xFF;
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F14;
    let first = callee_thiscall!(1, u32, this_ptr, arg);
    let count = unsafe { *((this_ptr.wrapping_add(0xD0)) as *const u32) };
    if count == 0 {
        return first;
    }
    let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
    let bank_table = unsafe { *global::<u32>(0x115D988) };
    let stride = unsafe { *global::<u32>(0x115D968) };
    let row = unsafe {
        *((bank_table
            .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
            .wrapping_add(BANK_BASE_OFF)) as *const u32)
    };
    let mut b: u8 = 0;
    loop {
        let i = b as u32;
        let slot = unsafe { *((this_ptr.wrapping_add(i).wrapping_add(0xB0)) as *const u8) };
        if slot != SLOT_EMPTY {
            let ptr = if i >= 8 {
                0
            } else {
                row.wrapping_add(stride.wrapping_mul(slot as u32))
            };
            callee_thiscall!(2, u32, ptr, arg);
        }
        b = b.wrapping_add(1);
        if (b as u32) >= count {
            break;
        }
    }
    count
});
