// original: 0x00908c90 blip_set_type
/// Set a blip's type id.
///
/// When the record has own data, stores `value` at +0x48. Returns `value` on
/// the storing path, else the id unchanged.
export!(cdecl, rw_00908C90(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            *(rec.add(0x48) as *mut u32) = value;
            value
        } else {
            id
        }
    }
});
