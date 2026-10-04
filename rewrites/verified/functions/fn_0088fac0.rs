// original: 0x0088fac0 audio_scan_slot_ready
/// Scans the 8 voice slots at `this+0x48` for one whose state word is 2.
///
/// Skips empty (`0xFF`) slots and slots whose entity-table entry is null.
/// Returns 1 on the first ready slot, 0 when none is ready.
export!(thiscall, rw_0088fac0(this_ptr: *const u8) -> u8 {
    unsafe {
        let base = *(relocated(0x0115D988) as *const u32) as *const u8;
        let stride = *(relocated(0x0115D964) as *const u32);
        let row_sel = *this_ptr.add(0x40) as u32;
        let row = base.add(row_sel.wrapping_mul(0x6F40) as usize);
        let mut slot: u32 = 0;
        while slot < 8 {
            let sel = *this_ptr.add(slot.wrapping_add(0x48) as usize);
            if sel != 0xFF {
                let entry = stride
                    .wrapping_mul(sel as u32)
                    .wrapping_add(*(row.add(0x6F10) as *const u32));
                if entry != 0 && *((entry as *const u8).add(6) as *const u16) == 2 {
                    return 1;
                }
            }
            slot += 1;
        }
        0
    }
});
