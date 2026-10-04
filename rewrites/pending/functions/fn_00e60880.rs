// original: 0x00e60880 timing_state_clear_02
/// Clear two global state words.
///
/// Stores zero to `0x019F393C` and `0x019F3940`. Takes no arguments; the
/// original leaves the return register untouched, so there is no
/// defined return value (checked with return channel `none`).
export!(cdecl, rw_00e60880() -> u32 {
    unsafe {
        *global::<u32>(0x19F393C) = 0;
        *global::<u32>(0x19F3940) = 0;
        0
    }
});
