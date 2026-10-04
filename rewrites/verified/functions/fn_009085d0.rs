// original: 0x009085d0 blip_set_alpha
/// Set a blip's alpha byte.
///
/// When the record has own data (flag byte at +0x8 set), stores the low byte
/// of `value` at +0x58. Returns the id with its low byte replaced by the
/// stored value on the storing path, else the id unchanged.
export!(cdecl, rw_009085D0(id: u32, value: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        if *rec.add(0x08) != 0 {
            *rec.add(0x58) = value as u8;
            (id & 0xFFFFFF00) | (value & 0xFF)
        } else {
            id
        }
    }
});
