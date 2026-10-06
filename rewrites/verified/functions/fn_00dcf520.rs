// original: 0x00dcf520 csv_open (proposed)

/// Open a csv reader on `path` with delimiter `mode`: set up the mode,
/// open the file, and initialise the object. Returns 1, or 0 when the open
/// fails.
///
/// `this` points to the reader. The setup callee (cdecl, one word: the mode
/// name) runs first, then the open callee (cdecl: path, open mode) whose
/// handle is stored at `+0x0`; a null handle returns 0 at once. Otherwise
/// the size callee (cdecl: handle) answers the size for `+0x4`, the base
/// offset at `+0x8`, fill flag at `+0x40c`, length at `+0x410`, position at
/// `+0x414`, first buffer byte at `+0xc` and char pair at `+0x418` are
/// zeroed, and the low byte of `mode` becomes the delimiter at `+0x41a`.
///
/// Original: 0x00DCF520 (thiscall, two stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf520(this: u32, path: u32, mode: u32) -> u32 {
    unsafe {
        /// Mode-setup callee id: (mode_name).
        const SETUP: u32 = 1;
        /// File-open callee id: (path, mode).
        const OPEN: u32 = 2;
        /// Size-query callee id: (handle).
        const SIZE: u32 = 3;
        const MODE_NAME: u32 = 0xef9e0d;
        const OPEN_MODE: u32 = 0xef9f2c;
        lf_checker_rt::callee_cdecl!(SETUP, u32, lf_checker_rt::relocated(MODE_NAME));
        let handle: u32 =
            lf_checker_rt::callee_cdecl!(OPEN, u32, path, lf_checker_rt::relocated(OPEN_MODE));
        (this as *mut u32).write_unaligned(handle);
        if handle == 0 {
            return 0;
        }
        let size: u32 = lf_checker_rt::callee_cdecl!(SIZE, u32, handle);
        ((this + 4) as *mut u32).write_unaligned(size);
        ((this + 8) as *mut u32).write_unaligned(0);
        ((this + 0x40c) as *mut u8).write(0);
        ((this + 0x410) as *mut u32).write_unaligned(0);
        ((this + 0x414) as *mut u32).write_unaligned(0);
        ((this + 0xc) as *mut u8).write(0);
        ((this + 0x41a) as *mut u8).write(mode as u8);
        ((this + 0x418) as *mut u16).write_unaligned(0);
        1
    }
});
