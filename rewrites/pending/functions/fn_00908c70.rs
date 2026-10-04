// original: 0x00908c70 blip_set_field_44
/// Set blip field +0x44.
///
/// When the record has own data, stores `value` at +0x44. Returns `value` on
/// the storing path, else the id unchanged.
export!(cdecl, rw_00908C70(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            *(rec.add(0x44) as *mut u32) = value;
            value
        } else {
            id
        }
    }
});
