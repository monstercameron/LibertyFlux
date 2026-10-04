// original: 0x008a0780 audSound_remaining_or_zero
/// Audio remaining-count helper.
///
/// Reads a signed field from the object; when it is negative the result is
/// zero, otherwise the argument minus that field is returned.
export!(thiscall, rw_008a0780(this: *const u8, value: u32) -> u32 {
    const FIELD_OFF: usize = 0x84;
    unsafe {
        let field = *(this.add(FIELD_OFF) as *const i32);
        if field < 0 {
            0
        } else {
            value.wrapping_sub(field as u32)
        }
    }
});
