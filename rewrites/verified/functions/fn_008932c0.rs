// original: 0x008932C0 audsound_start_voice_ex
/// Starts an extended voice under the lock unless one is already starting.
///
/// Takes the lock callee (cdecl, the dword at `this+0x38`); when the flag
/// byte at 0x47 is set or the dword at `this+8` is NONZERO (unsigned)
/// releases the lock and returns at once. Otherwise writes the request: 1 at
/// the bytes 0x8e and 0x49 and 0x46, the low word of `w` at 0x8c and 0x40,
/// `v` at 0x88 and 0x68, all-ones at 0x6c, 0x70 and 0x74, 2 at 0x50, zero at
/// 0x54, 0x58, 0x5c and 0x78 and eight zero bytes at 0x80 (via a zeroed SSE
/// register), 1 at the word 0x4c; then stores the fresh-id callee's (cdecl,
/// no arguments) answer at 0x2c, calls the use callee (cdecl, `w`), releases
/// the lock and returns. The return value is the unlock callee's answer on
/// every path.
/// Original: 0x008932C0 (thiscall, two stack words: w, v).
export!(thiscall, rw_008932C0(this: *mut u8, w: u32, v: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        const FRESH_ID: u32 = 3;
        const USE: u32 = 4;
        const MUTEX: usize = 0x38;
        let m = *(this.add(MUTEX) as *const u32);
        let _: u32 = callee_cdecl!(LOCK, u32, m);
        if *this.add(0x47) != 0 || *(this.add(8) as *const u32) != 0 {
            return callee_cdecl!(UNLOCK, u32, m);
        }
        *this.add(0x8e) = 1;
        *(this.add(0x8c) as *mut u16) = w as u16;
        *(this.add(0x88) as *mut u32) = v;
        *this.add(0x49) = 1;
        *(this.add(0x40) as *mut u16) = w as u16;
        *(this.add(0x68) as *mut u32) = v;
        *(this.add(0x6c) as *mut u32) = 0xffff_ffff;
        *(this.add(0x50) as *mut u32) = 2;
        *(this.add(0x54) as *mut u32) = 0;
        *(this.add(0x58) as *mut u32) = 0;
        *(this.add(0x5c) as *mut u32) = 0;
        *(this.add(0x70) as *mut u32) = 0xffff_ffff;
        *(this.add(0x74) as *mut u32) = 0xffff_ffff;
        *(this.add(0x4c) as *mut u16) = 1;
        *(this.add(0x80) as *mut u64) = 0;
        *(this.add(0x78) as *mut u32) = 0;
        *this.add(0x46) = 1;
        let id: u32 = callee_cdecl!(FRESH_ID, u32,);
        *(this.add(0x2c) as *mut u32) = id;
        let _: u32 = callee_cdecl!(USE, u32, w);
        callee_cdecl!(UNLOCK, u32, m)
    }
});
