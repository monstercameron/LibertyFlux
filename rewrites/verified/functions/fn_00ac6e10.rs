// original: 0x00AC6E10 stream_set_mode_byte0 (proposed)

/// Store the low byte of `arg` into mode byte 0 when it differs.
///
/// The original compares the stored byte with the argument byte and keeps the
/// old value on equality (cdecl, one stack word). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6E10(arg: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x0154E040;
        let g = lf_checker_rt::global::<u8>(MODE);
        let old = *g;
        let new = arg as u8;
        *g = if old != new { new } else { old };
        0
    }
});
