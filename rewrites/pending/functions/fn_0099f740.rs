// original: 0x0099f740 state_word_check
/// Check the audio object's state word through its handle.
///
/// Returns 1 only when the object is non-null, its flag byte at +0x26C has
/// bit 2 set, its handle at +0xB30 is non-null, and the dword at +0x1304
/// past that handle equals 1; otherwise 0.
export!(stdcall, rw_0099f740(obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        if *((obj.wrapping_add(0x26C)) as *const u8) & 4 == 0 {
            return 0;
        }
        let h = *((obj.wrapping_add(0xB30)) as *const u32);
        if h == 0 {
            return 0;
        }
        ((*((h.wrapping_add(0x1304)) as *const u32)) == 1) as u32
    }
});
