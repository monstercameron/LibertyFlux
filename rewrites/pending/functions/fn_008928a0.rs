// original: 0x008928a0 audSound_set_text
/// Store a copy of the given wide string on this sound.
///
/// Ensures the sound's helper object exists (allocating and constructing
/// it on first use), measures the input, allocates a fresh buffer for it,
/// and hands the fill and copy to helpers before NUL-terminating. Takes
/// the sound lock around the whole update and returns the unlock answer.
export!(thiscall, rw_008928a0(sound: u32, text: u32) -> u32 {
    unsafe {
        let lock = *((sound + 0x38) as *const u32);
        callee_cdecl!(1, u32, lock);
        if *((sound + 0x98) as *const u32) != 0 {
            callee_cdecl!(2, u32, lock);
            callee_thiscall!(3, u32, sound);
            callee_cdecl!(1, u32, lock);
        }
        *((sound + 0x9c) as *mut u8) = 0;
        if *((sound + 0x94) as *const u32) == 0 {
            let blk: u32 = callee_cdecl!(4, u32, 0x34);
            let mut obj = 0u32;
            if blk != 0 {
                obj = callee_thiscall!(5, u32, blk);
            }
            *((sound + 0x94) as *mut u32) = obj;
            if obj == 0 {
                *((sound + 0x98) as *mut u32) = 0;
                return callee_cdecl!(2, u32, lock);
            }
        }
        let mut len = 0u32;
        while ((text.wrapping_add(len.wrapping_mul(2))) as *const u16)
            .read_unaligned()
            != 0
        {
            len += 1;
        }
        let n = len.wrapping_add(1);
        let (bytes, overflowed) = n.overflowing_mul(2);
        let size = if overflowed { 0xffffffff } else { bytes };
        let dst: u32 = callee_cdecl!(6, u32, size);
        *((sound + 0x98) as *mut u32) = dst;
        callee_cdecl!(7, u32, dst, 0, n);
        if dst == 0 {
            return callee_cdecl!(2, u32, lock);
        }
        callee_cdecl!(8, u32, dst, text, len);
        ((dst.wrapping_add(len.wrapping_mul(2))) as *mut u16).write_unaligned(0);
        callee_cdecl!(2, u32, lock)
    }
});
