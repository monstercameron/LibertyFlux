// original: 0x008efdb0 select_member_ptr2
/// Pick one of three member pointers from a flag and two xor bytes.
///
/// Same shape as [`rw_008efd70`] with a different switch (0x117E6E0),
/// byte pair (+0x2C8E/+0x2C8C) and member offsets.
export!(thiscall, rw_008efdb0(this_ptr: u32, flag: u32) -> u32 {
    unsafe {
        if (flag as u8) != 0 && *global::<u32>(0x117E6E0) != 0 {
            return this_ptr.wrapping_add(0x2F48);
        }
        let cl = *((this_ptr + 0x2C8E) as *const u8)
            ^ *((this_ptr + 0x2C8C) as *const u8);
        if cl == 0x80 {
            this_ptr.wrapping_add(0x2B28)
        } else {
            this_ptr.wrapping_add(0x2C88)
        }
    }
});
