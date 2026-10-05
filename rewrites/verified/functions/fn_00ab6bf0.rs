// original: 0x00ab6bf0 stream_is_flagged (proposed)

/// Test whether either of the low two status bits is set.
///
/// Returns 1 when the byte at `this + 0x360` has bit 0 or bit 1 set,
/// otherwise 0.
///
/// Original: 0x00ab6bf0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab6bf0(this: u32) -> u32 {
    unsafe {
        const STATUS_OFF: u32 = 0x360;
        const WANTED: u8 = 3;
        let b = ((this + STATUS_OFF) as *const u8).read();
        u32::from(b & WANTED != 0)
    }
});
