// original: 0x00d5ac00 ccam_sector_init
/// Initialise a sector camera: clear its state word and report success.
///
/// Zeroes the dword at `+0x140` and returns 1.
///
/// Original: thiscall, no stack arguments, returns 1 in `al`.
lf_checker_rt::export!(thiscall, rw_00d5ac00 (this: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x140;
        ((this + STATE) as *mut u32).write_unaligned(0);
        1
    }
});
