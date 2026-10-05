// original: 0x008efd70 select_member_ptr
/// Pick one of three member pointers from a flag and two xor bytes.
///
/// When `flag` is set and the global switch at 0x117E6DC is nonzero,
/// returns the first member; otherwise the xor of the bytes at +0x2C9E and
/// +0x2C9C selects between the other two (0x80 picks the alternate).
export!(thiscall, rw_008efd70(this_ptr: u32, flag: u32) -> u32 {
    unsafe {
        if (flag as u8) != 0 && *global::<u32>(0x117E6DC) != 0 {
            return this_ptr.wrapping_add(0x2F58);
        }
        let cl = *((this_ptr + 0x2C9E) as *const u8)
            ^ *((this_ptr + 0x2C9C) as *const u8);
        if cl == 0x80 {
            this_ptr.wrapping_add(0x2B18)
        } else {
            this_ptr.wrapping_add(0x2C98)
        }
    }
});
