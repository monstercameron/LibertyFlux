// original: 0x00e607c0 timing_state_clear_01
/// Clear two global state words.
///
/// Stores zero to `0x019F3880` and `0x019F3884`. Takes no arguments; the
/// original leaves the return register untouched, so there is no
/// defined return value (checked with return channel `none`).
export!(cdecl, rw_00e607c0() -> u32 {
    unsafe {
        *global::<u32>(0x19F3880) = 0;
        *global::<u32>(0x19F3884) = 0;
        0
    }
});
