// original: 0x00908610 blip_set_display
/// Set a blip's display value.
///
/// When the record has own data, stores `value` at +0x4C. Returns `value` on
/// the storing path, else the id unchanged.
export!(cdecl, rw_00908610(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            *(rec.add(0x4C) as *mut u32) = value;
            value
        } else {
            id
        }
    }
});
