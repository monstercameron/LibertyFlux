// original: 0x008FBD20 text_resolve_key_or_fallback
/// Resolve a text key through the copy-and-tag path, else directly.
///
/// When the flag byte is set, the string is null or its first byte
/// is NUL, the string goes straight to the resolver. Otherwise a
/// 32-byte zero frame is built, the copy routine fills its first 16
/// bytes, the frame is scanned from offset 12 for a NUL, the tag
/// dword is written over that NUL, and the frame goes to the
/// resolver; a non-null resolution whose first word is nonzero is
/// returned, else the original string is resolved as a fallback.
/// (The original guards its frame with a stack cookie validated by a
/// call that preserves registers; the rewrite keeps no cookie and
/// forwards an equivalent call.) Cdecl, two stack arguments.
export!(cdecl, rw_008fbd20(s: u32, flag: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x116bff0;
        const TAG: u32 = 0x0e8414c;
        const SCAN_FROM: usize = 12;
        if (flag as u8) != 0 || s == 0 || ((s as *const u8).read() == 0) {
            let ans: u32 = callee_thiscall!(3, u32, relocated(MGR), s);
            callee_thiscall!(4, u32, 0);
            return ans;
        }
        let mut frame = [0u8; 32];
        let buf = frame.as_mut_ptr() as u32;
        let _: u32 = callee_cdecl!(1, u32, buf, s, 0x10);
        let mut i = SCAN_FROM;
        while frame[i] != 0 {
            i += 1;
        }
        let tag = *global::<u32>(TAG);
        ((buf + i as u32) as *mut u32).write_unaligned(tag);
        let r: u32 = callee_thiscall!(2, u32, relocated(MGR), buf);
        if r != 0 && ((r as *const u16).read_unaligned() != 0) {
            callee_thiscall!(4, u32, 0);
            return r;
        }
        let ans: u32 = callee_thiscall!(3, u32, relocated(MGR), s);
        callee_thiscall!(4, u32, 0);
        ans
    }
});
