// original: 0x008eefe0 status_nibble_check
/// Accept a record unless its status nibble marks it otherwise.
///
/// Reads the high nibble of the flag byte at +0x1C: nibbles 1, 2 and 8 are
/// rejected (returns 0), every other value is accepted (returns 1).
export!(stdcall, rw_008eefe0(rec: u32) -> u8 {
    unsafe {
        let nibble = *((rec + 0x1C) as *const u8) >> 4;
        if nibble == 1 || nibble == 2 || nibble == 8 {
            0
        } else {
            1
        }
    }
});
