// original: 0x00908c50 blip_set_field_24
/// Set blip field +0x24.
///
/// When the record has own data, stores `value` at +0x24. Returns `value` on
/// the storing path, else the id unchanged.
export!(cdecl, rw_00908C50(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            *(rec.add(0x24) as *mut u32) = value;
            value
        } else {
            id
        }
    }
});
