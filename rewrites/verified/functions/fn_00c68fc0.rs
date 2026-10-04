// original: 0x00c68fc0 slot_array_any_flagged
// Scan the 64-slot array for an entry whose table object has bit 1 of the
// word at +0x120 set. Empty (-1) entries are skipped. Returns 1 on the
// first flagged entry, else 0.
export!(thiscall, rw_00c68fc0(slots: *const u32) -> u32 {
    unsafe {
        let tab = relocated(0x1295cd8);
        for i in 0..64u32 {
            let id = *slots.add(i as usize);
            if id == 0xffff_ffff {
                continue;
            }
            let ent = *(tab.wrapping_add(id.wrapping_mul(4)) as *const u32);
            let f = *((ent as *const u8).add(0x120) as *const u32);
            if f & 2 != 0 {
                return 1;
            }
        }
        0
    }
});
