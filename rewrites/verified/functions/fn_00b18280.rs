// original: 0x00B18280 object_set_state_3

/// Set a CObject's state byte to 3 and clear its pending state word.
///
/// `this` is a 32-bit pointer to the object. The routine writes one byte at
/// offset `0x22a` and an unaligned 32-bit zero at offset `0x224`, then returns
/// without a defined result. It uses the Microsoft x86 thiscall convention.
lf_checker_rt::export!(thiscall, rw_00B18280(this: u32) -> u32 {
    unsafe {
        const STATE_KIND: u32 = 0x22a;
        const PENDING_STATE: u32 = 0x224;
        ((this + STATE_KIND) as *mut u8).write(3);
        ((this + PENDING_STATE) as *mut u32).write_unaligned(0);
    }
    0
});
