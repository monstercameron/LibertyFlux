// original: 0x008a6b70 audio_copy_216byte_block
/// Copy a 216-byte audio state block forward and return the destination.
///
/// Copies bytes 0..0xD8 in increasing address order (dwords throughout,
/// finishing with a word at 0xD0 and single bytes up to 0xD8) and returns
/// the destination pointer.
export!(thiscall, rw_008a6b70(dst: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let mut i = 0usize;
        while i < 0xD8 {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
        dst as u32
    }
});

