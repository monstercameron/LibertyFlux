// original: 0x00893220 audsound_start_voice
/// Starts a voice under the object lock, storing its parameters.
///
/// Takes the lock callee (cdecl, the dword at `this+0x38`); when the flag
/// byte at 0x47 is set, releases the lock and returns at once. Otherwise
/// stores the low word of `w` at 0x40, zero at the byte 0x4c, `v` at 0x6c,
/// all-ones at 0x68, 1 at 0x50 and at the byte 0x46, stores the fresh-id
/// callee's (cdecl, no arguments) answer at 0x2c, calls the use callee
/// (cdecl, `w`), releases the lock and returns. The return value is the
/// unlock callee's answer on every path.
/// Original: 0x00893220 (thiscall, two stack words: w, v).
export!(thiscall, rw_00893220(this: *mut u8, w: u32, v: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        const FRESH_ID: u32 = 3;
        const USE: u32 = 4;
        const MUTEX: usize = 0x38;
        let m = *(this.add(MUTEX) as *const u32);
        let _: u32 = callee_cdecl!(LOCK, u32, m);
        if *this.add(0x47) != 0 {
            return callee_cdecl!(UNLOCK, u32, m);
        }
        *(this.add(0x40) as *mut u16) = w as u16;
        *this.add(0x4c) = 0;
        *(this.add(0x6c) as *mut u32) = v;
        *(this.add(0x68) as *mut u32) = 0xffff_ffff;
        *(this.add(0x50) as *mut u32) = 1;
        *this.add(0x46) = 1;
        let id: u32 = callee_cdecl!(FRESH_ID, u32,);
        *(this.add(0x2c) as *mut u32) = id;
        let _: u32 = callee_cdecl!(USE, u32, w);
        callee_cdecl!(UNLOCK, u32, m)
    }
});
