// original: 0x009085f0 blip_set_colour
/// Set a blip's colour record.
///
/// When the record has own data, stores `value` at +0x54. Returns `value` on
/// the storing path, else the id unchanged.
export!(cdecl, rw_009085F0(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            *(rec.add(0x54) as *mut u32) = value;
            value
        } else {
            id
        }
    }
});
