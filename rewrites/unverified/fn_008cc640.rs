// original: 0x008CC640 stream_copy16 (proposed)

/// Copies 16 bytes from `src` to the global buffer `DST` through the copy
/// callee (callee 0, cdecl with destination, source and length 0x10).
///
/// One stack argument (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CC640(src: u32) -> u32 {
    unsafe {
        /// Global destination buffer.
        const DST: u32 = 0x1173230;
        /// Bytes copied.
        const LEN: u32 = 0x10;
        /// Copy callee id.
        const COPY: u32 = 0;
        let _c: u32 = lf_checker_rt::callee_cdecl!(COPY, u32, lf_checker_rt::relocated(DST), src, LEN);
        0
    }
});
