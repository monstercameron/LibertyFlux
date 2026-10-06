// original: 0x00892C60 audsound_match_voice_class
/// Classifies the voice request under the object lock, returning 0 to 3.
///
/// Takes the lock callee (cdecl, the dword at `this+0x38`), then compares
/// the argument word `w` and dword `v` against the fields: returns 0 when
/// both flag bytes at 0x47/0x46 are clear, `w` equals the word at 0x42 and
/// `v` equals the dword at 0x6c; returns 1 when `w` equals the word at 0x42
/// (via the 0x40 word as an alternate: equal there also reaches the check)
/// and `v` equals the dword at 0x6c; returns 2 when `w` equals the word at
/// 0x44; returns 3 otherwise. Every path releases the lock (cdecl) before
/// returning. All comparisons are equality on raw bits.
/// Original: 0x00892C60 (thiscall, two stack words: w, v).
export!(thiscall, rw_00892C60(this: *mut u8, w: u32, v: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        const MUTEX: usize = 0x38;
        let m = *(this.add(MUTEX) as *const u32);
        let _: u32 = callee_cdecl!(LOCK, u32, m);
        let w = w as u16;
        let w42 = *(this.add(0x42) as *const u16);
        let mut edi = 3u32;
        if *this.add(0x47) == 0 && *this.add(0x46) == 0 && w == w42 && *(this.add(0x6c) as *const u32) == v {
            edi = 0;
        } else if (w == w42 || w == *(this.add(0x40) as *const u16))
            && *(this.add(0x6c) as *const u32) == v
        {
            edi = 1;
        } else if w == *(this.add(0x44) as *const u16) {
            edi = 2;
        }
        let _: u32 = callee_cdecl!(UNLOCK, u32, m);
        edi
    }
});
