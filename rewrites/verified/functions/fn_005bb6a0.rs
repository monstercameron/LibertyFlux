// original: 0x005BB6A0 split_packed_word_to_bytes
/// Write the four bytes of `packed` from most significant to least
/// significant through four output pointers, then return the fourth pointer.
///
/// The original is cdecl. Each pointer addresses one writable byte.
lf_checker_rt::export!(cdecl, rw_005bb6a0(packed: u32, out0: u32, out1: u32, out2: u32, out3: u32) -> u32 {
    unsafe {
        (out0 as *mut u8).write((packed >> 24) as u8);
        (out1 as *mut u8).write((packed >> 16) as u8);
        (out2 as *mut u8).write((packed >> 8) as u8);
        (out3 as *mut u8).write(packed as u8);
    }
    out3
});
