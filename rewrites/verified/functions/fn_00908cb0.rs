// original: 0x00908cb0 blip_flags_or
/// OR flag bits into a blip's flag word at +0x20.
///
/// Uses the record itself when it has own data, else the default blip's
/// record. Returns the updated record's id with its low word replaced by the
/// OR mask (the original loads the mask into AX before storing).
export!(cdecl, rw_00908CB0(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            let cell = rec.add(0x20) as *mut u16;
            *cell |= value as u16;
            (id & 0xFFFF0000) | (value & 0xFFFF)
        } else {
            let cur = *global::<u32>(BLIP_DEFAULT);
            let cell = blip(cur).add(0x20) as *mut u16;
            *cell |= value as u16;
            (cur & 0xFFFF0000) | (value & 0xFFFF)
        }
    }
});
