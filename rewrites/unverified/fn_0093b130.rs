// original: 0x0093B130 stream_quad_notify (proposed)

/// Push three constant words and the shared table through notify.
///
/// Writes three magic words to a frame buffer and notifies on the
/// buffer, then on the buffer plus 4, then plus 8, each with length 4;
/// then notifies on the shared table with four times the table count.
/// Verifies the stack cookie and answers the check's leftover (zero
/// under this contract). The cookie register is stack-dependent and is
/// not compared; snapshots cover the written constants only.
lf_checker_rt::export!(cdecl, rw_0093b130() -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const SECOND: u32 = 2;
        const THIRD: u32 = 3;
        const TABLE_CALL: u32 = 4;
        const COOKIE_CHECK: u32 = 5;
        const WORD_A: u32 = 0x2280A23;
        const WORD_B: u32 = 0x235FBA7;
        const WORD_C: u32 = 0x22C2959;
        const CHUNK: u32 = 4;
        const TABLE: u32 = 0x11A4F20;
        const COUNT: u32 = 0x11A4F7C;
        let mut buf = [WORD_A, WORD_B, WORD_C, 0u32];
        let p = &mut buf as *mut u32 as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(FIRST, u32, p, CHUNK);
        let _: u32 = lf_checker_rt::callee_cdecl!(SECOND, u32, p + 4, CHUNK);
        let _: u32 = lf_checker_rt::callee_cdecl!(THIRD, u32, p + 8, CHUNK);
        let n = lf_checker_rt::global::<u32>(COUNT).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            TABLE_CALL,
            u32,
            lf_checker_rt::relocated(TABLE),
            n << 2
        );
        lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,)
    }
});
