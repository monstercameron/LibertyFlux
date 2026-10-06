// original: 0x00dcf490 csv_reset (proposed)

/// Reset for the csv reader: closes the handle and returns the object to
/// its freshly-opened shape.
///
/// `this` points to the reader. The handle at `+0x0` is passed with a zero
/// to the close callee (cdecl, two words; its answer is ignored). Then the
/// base offset at `+0x8`, the fill flag at `+0x40c`, the length at `+0x410`,
/// the position at `+0x414`, the first buffer byte at `+0xc` and the
/// current/previous char pair at `+0x418` are all zeroed. Always returns 1.
///
/// Original: 0x00DCF490 (thiscall, no stack arguments, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf490(this: u32) -> u32 {
    unsafe {
        /// Handle-close callee id.
        const CLOSE: u32 = 1;
        let handle = (this as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(CLOSE, u32, handle, 0);
        ((this + 8) as *mut u32).write_unaligned(0);
        ((this + 0x40c) as *mut u8).write(0);
        ((this + 0x410) as *mut u32).write_unaligned(0);
        ((this + 0x414) as *mut u32).write_unaligned(0);
        ((this + 0xc) as *mut u8).write(0);
        ((this + 0x418) as *mut u16).write_unaligned(0);
        1
    }
});
