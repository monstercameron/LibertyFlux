// original: 0x0094e070 NativeImpl_RELEASE_TEXTURE
/// Script native `RELEASE_TEXTURE`: release a texture id through the manager.
///
/// Asks the lookup worker about `id`; a negative answer means "not present"
/// and is returned as-is. Otherwise zeroes the 13-byte record header at
/// `answer * 48` bytes into the manager and returns `answer * 6`, matching
/// the original's exit EAX on both paths.
export!(thiscall, rw_0094e070(mgr: *mut u8, id: u32) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, id, 10u32);
        if (answer as i32) < 0 {
            answer
        } else {
            let base = mgr.add(answer.wrapping_mul(48) as usize);
            *(base.add(0x2C) as *mut u32) = 0;
            *(base.add(0x04) as *mut u32) = 0;
            *(base.add(0x08) as *mut u32) = 0;
            *base.add(0x0C) = 0;
            answer.wrapping_mul(6)
        }
    }
});
