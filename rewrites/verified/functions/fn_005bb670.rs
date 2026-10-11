// original: 0x005BB670 NativeImpl_SET_PLAYER_SETTINGS_GENRE
/// Pack the low byte of four arguments into one big-endian word. The first
/// argument supplies bits 24..31 and the fourth supplies bits 0..7.
///
/// The original is cdecl and returns the complete 32-bit packed value.
lf_checker_rt::export!(cdecl, rw_005bb670(a: u32, b: u32, c: u32, d: u32) -> u32 {
    ((a & 0xff) << 24) | ((b & 0xff) << 16) | ((c & 0xff) << 8) | (d & 0xff)
});
