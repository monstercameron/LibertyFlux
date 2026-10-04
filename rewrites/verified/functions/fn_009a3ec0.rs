// original: 0x009a3ec0 audio_check_watermark
/// Original 0x009a3ec0 (unnamed): compare against a global watermark.
///
/// Returns 1 when `arg` is at or above the global dword plus 1000
/// (unsigned comparison), else 0.
export!(stdcall, rw_009a3ec0(arg: u32) -> u32 {
    let mark = unsafe { (relocated(0x012845C0) as *const u32).read() };
    u32::from(arg >= mark.wrapping_add(1000))
});
