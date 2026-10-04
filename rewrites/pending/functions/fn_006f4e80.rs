// original: 0x006f4e80 check_header_flags
/// Flag check: bit 0 of byte 2 set and byte 5 equal to 1.
///
/// Returns 1 or 0.
rt::export!(thiscall, rw_006f4e80(this: *const u8) -> u32 {
    unsafe { ((*this.add(2) & 1) == 1 && *this.add(5) == 1) as u32 }
});
