// original: 0x008857F0 cwm_stream_reader_get_sample_rate (proposed)

/// Return the sample-rate field when the stream reader's ready byte is set.
///
/// The receiver points to an inherited reader object. Readiness is the byte
/// at offset `0x0a`; the 32-bit sample-rate value is at offset `0x20024`.
/// An unready reader returns the all-bits-one sentinel.
lf_checker_rt::export!(thiscall, rw_008857f0(this: u32) -> u32 {
    unsafe {
        const READY_OFFSET: u32 = 0x0a;
        const SAMPLE_RATE_OFFSET: u32 = 0x20024;
        if (this.wrapping_add(READY_OFFSET) as *const u8).read() == 0 { return u32::MAX; }
        (this.wrapping_add(SAMPLE_RATE_OFFSET) as *const u32).read_unaligned()
    }
});
