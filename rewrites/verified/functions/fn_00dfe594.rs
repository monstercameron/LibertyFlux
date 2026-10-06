// original: 0x00dfe594 locale_convert_framebuf
/// Convert a value through two helpers sharing one frame buffer.
///
/// Hands a four-word frame buffer in ECX to the first helper with the third
/// argument, then passes the first two arguments with the buffer address to
/// the second helper. When the buffer's flag byte is set, clears one status
/// bit in the object the buffer points at.
///
/// The contract skips both buffer addresses: the first call's input is
/// uninitialised (its four output words are scripted instead) and the second
/// call's four words are snapshotted.
export!(cdecl, rw_00dfe594(first: u32, second: u32, third: u32) -> u32 {
    unsafe {
        let mut buf = [0u32; 4];
        callee_thiscall!(1, u32, buf.as_mut_ptr() as u32, third);
        let answer = callee_cdecl!(2, u32, first, second, buf.as_ptr() as u32);
        if buf[3] & 0xff != 0 {
            let target = buf[2] as *mut u32;
            *target.add(0x1c) &= !2;
        }
        answer
    }
});
