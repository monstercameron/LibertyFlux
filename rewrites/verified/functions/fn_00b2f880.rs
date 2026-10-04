// original: 0x00b2f880 tag6_clear
// 0xB2F880 tag6_clear (thiscall/0 -> eax).
//
// Clears a six-byte tag (byte, dword, byte) and returns it.
export!(thiscall, rw_00b2f880(rec: *mut u8) -> u32 {
    unsafe {
        *rec = 0;
        core::ptr::write_unaligned(rec.add(1) as *mut u32, 0);
        *rec.add(5) = 0;
        rec as u32
    }
});
