// original: 0x005e5e60 parse_hash_color
/// Parse a hash-led color string into a color word, defaulting to white.
///
/// When the string starts with '#', scans the rest with the hex helper
/// into a frame slot starting at all-ones; otherwise keeps white. Sets
/// the top byte (full opacity) and stores the word through the out
/// pointer. Returns the out pointer.
export!(stdcall, rw_005e5e60(out: *mut u32, text: *const u8) -> u32 {
    unsafe {
        let mut slot: u32 = 0xffffffff;
        let color = if *text == 0x23 {
            callee_cdecl!(
                1,
                u32,
                text.add(1) as u32,
                relocated(0x00f92a6c),
                &mut slot as *mut u32 as u32
            );
            slot | 0xff000000
        } else {
            0xffffffff
        };
        *out = color;
        out as u32
    }
});
