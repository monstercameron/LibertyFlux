// original: 0x00885810 rage::CWMStreamReader::get_media_duration

/// Return the media-duration field when the stream reader is ready.
///
/// The readiness byte is at receiver offset `0x0a`, and the 32-bit duration
/// value is at offset `0x20028`. An unready reader returns `u32::MAX`.
lf_checker_rt::export!(thiscall, rw_00885810(this: u32) -> u32 {
    unsafe {
        const READY_OFFSET: u32 = 0x0a;
        const MEDIA_DURATION_OFFSET: u32 = 0x20028;
        if (this.wrapping_add(READY_OFFSET) as *const u8).read() == 0 { return u32::MAX; }
        (this.wrapping_add(MEDIA_DURATION_OFFSET) as *const u32).read_unaligned()
    }
});
