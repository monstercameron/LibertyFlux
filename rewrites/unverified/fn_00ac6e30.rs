// original: 0x00AC6E30 stream_set_mode_byte1 (proposed)

/// Store the low byte of `arg` into mode byte 1 when it differs.
///
/// Same shape as its neighbour for mode byte 0 (cdecl, one stack word).
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6E30(arg: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x0154E041;
        let g = lf_checker_rt::global::<u8>(MODE);
        let old = *g;
        let new = arg as u8;
        *g = if old != new { new } else { old };
        0
    }
});
