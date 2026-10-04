// original: 0x00891d20 audSound_path_lookup
/// Resolve a slash-separated voice path against this sound's bank.
///
/// Walks the path segment by segment: a lone digit selects a voice slot
/// directly, any other segment is hashed (the Joaat-style helper) and
/// resolved through the bank, and each resolved segment recurses into the
/// remainder after the separator. Returns the resolved voice id, or zero
/// when the sound is locked, the path is empty or overlong, a separator
/// sits at either end, or any segment fails to resolve.
export!(thiscall, rw_00891d20(sound: u32, path: u32) -> u32 {
    unsafe {
        fn strlen(p: u32) -> u32 {
            let mut n = 0u32;
            unsafe {
                while *((p + n) as *const u8) != 0 {
                    n += 1;
                }
            }
            n
        }
        if *((sound + 0x39) as *const u8) & 0x80 != 0
            && *((sound + 6) as *const u16) == 2
        {
            return 0;
        }
        let mut seg = [0u8; 30];
        let len = strlen(path);
        if len == 0 || len >= 0x1e {
            return 0;
        }
        let mut i = 0u32;
        let mut cut = len;
        while i < len {
            let c = *((path + i) as *const u8);
            if c == b'/' || c == b'\\' {
                cut = i;
                break;
            }
            i += 1;
        }
        if cut == len {
            if len == 1 {
                let c = *(path as *const u8);
                if c >= b'0' && c < b'8' {
                    return callee_thiscall!(1, u32, sound, (c - b'0') as u32);
                }
            }
            let h: u32 = callee_cdecl!(5, u32, path, 0);
            return callee_thiscall!(4, u32, sound, h);
        }
        if cut >= len - 1 || cut == 0 {
            return 0;
        }
        callee_cdecl!(2, u32, seg.as_mut_ptr() as u32, path, cut);
        let sub: u32;
        if strlen(seg.as_ptr() as u32) == 1 {
            let c = seg[0];
            if c.wrapping_sub(b'0') <= 7 {
                sub = callee_thiscall!(1, u32, sound, (c - b'0') as u32);
            } else {
                let h: u32 = callee_cdecl!(3, u32, seg.as_ptr() as u32, 0);
                sub = callee_thiscall!(4, u32, sound, h);
            }
        } else {
            let h: u32 = callee_cdecl!(3, u32, seg.as_ptr() as u32, 0);
            sub = callee_thiscall!(4, u32, sound, h);
        }
        if sub == 0 {
            return 0;
        }
        callee_thiscall!(6, u32, sub, path.wrapping_add(cut).wrapping_add(1))
    }
});
