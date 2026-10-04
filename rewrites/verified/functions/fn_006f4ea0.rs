// original: 0x006f4ea0 input_field_selector
/// Selects one of three field pointers inside an input object by its kind byte.
///
/// The byte at offset 5 holds a small kind tag: kinds 2 and 3 select the
/// field at offset 0x18, kinds 0, 1 and 4 select the field at offset 0x0c,
/// and any other value selects the object base itself.
export!(thiscall, rw_006f4ea0(this: u32) -> u32 {
    unsafe {
        let kind = *((this + 5) as *const u8);
        if kind > 4 {
            this
        } else if kind == 2 || kind == 3 {
            this.wrapping_add(0x18)
        } else {
            this.wrapping_add(0x0c)
        }
    }
});
