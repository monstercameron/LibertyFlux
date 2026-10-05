// original: 0x00a05e90 NativeImpl_PLAYER_IS_NEAR_FIRST_PIGEON (native)
/// Copy the first-pigeon proximity vector to `dst` when one is reported.
///
/// Asks the pigeon tracker for its 16-byte result block; when the tracker
/// reports success the four words are copied to `dst`. Always answers 0
/// (only the low byte is written). Cdecl, one word.
lf_checker_rt::export!(cdecl, rw_00a05e90(dst: u32) -> u32 {
    unsafe {
        const TRACKER: u32 = 0;
        let mut block = [0u32; 4];
        let ok: u32 = lf_checker_rt::callee_cdecl!(
            TRACKER, u32, block.as_mut_ptr() as u32);
        if (ok & 0xff) != 0 {
            for i in 0..4usize {
                ((dst + (i as u32) * 4) as *mut u32).write_unaligned(block[i]);
            }
        }
        0
    }
});
