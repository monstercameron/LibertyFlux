// original: 0x00c23e30 stream_lookup_forward (proposed)
/// Forward `(a1, a2)` to the lookup worker as `(a2, 0, 0, &frame, &slot)`
/// where `&frame` points at a frame copy of a1 and `&slot` at the incoming
/// a1 slot after the original overwrites it with a2; a0 travels as the tag. The incoming slot is
/// dead (cdecl, popped by the caller), so the rewrite keeps an equivalent
/// local and the contract skips the stack check. a0 is unread. Returns the
/// worker's answer.
///
/// Original: 0x00c23e30 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00c23e30(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let mut frame_copy: u32 = a1;
        let mut slot_alias: u32 = a2;
        lf_checker_rt::callee_cdecl!(
            1, u32, a0, 0, 0,
            &mut frame_copy as *mut u32 as u32,
            &mut slot_alias as *mut u32 as u32
        )
    }
});
