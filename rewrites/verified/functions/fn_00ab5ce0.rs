// original: 0x00ab5ce0 stream_hdr_reset (proposed)

/// Zero a small streaming header in place and return it.
///
/// Clears the words at `+0`, `+8` and `+0x0c`, the half-word at `+4` and the
/// bytes at `+6` and `+0x13` (byte `+7` is left alone). Returns the header
/// pointer.
///
/// Original: 0x00ab5ce0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5ce0(hdr: u32) -> u32 {
    unsafe {
        (hdr as *mut u32).write_unaligned(0);
        ((hdr + 4) as *mut u16).write_unaligned(0);
        ((hdr + 6) as *mut u8).write(0);
        ((hdr + 8) as *mut u32).write_unaligned(0);
        ((hdr + 0x0C) as *mut u32).write_unaligned(0);
        ((hdr + 0x13) as *mut u8).write(0);
        hdr
    }
});
