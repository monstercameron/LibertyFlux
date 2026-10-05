// original: 0x00b97420 NativeImpl_SKIP_ONE_WAIT_FOR_WANTED_LEVEL

/// Skips a wait, packing a position with a trailing flag.
///
/// Same shape as `rw_00b972c0`: packs (`a0`, `a1`, `a2`) into a frame buffer
/// and calls `SKIP` with (`a3`, 0, 0, 1). No value is returned. The buffer
/// pointer is a skipped call argument with snapshot-verified contents.
///
/// Original: 0x00B97420 (cdecl, four stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b97420(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    const SKIP: u32 = 1;
    let mut buf = [a0, a1, a2];
    let _: u32 = lf_checker_rt::callee_cdecl!(
        SKIP, u32, buf.as_mut_ptr() as u32, a3, 0, 0, 1
    );
    0
});
