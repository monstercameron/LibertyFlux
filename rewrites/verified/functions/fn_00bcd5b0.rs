// original: 0x00bcd5b0 NativeImpl_SET_ALL_CARS_CAN_BE_DAMAGED_4

/// Dispatches the pending-action helper from the low byte of one cdecl word.
/// A nonzero byte calls the helper with one. A zero byte calls it with zero,
/// fills a three-word local triplet through the triplet helper, then forwards
/// that buffer and the fixed radius bits to the follow-up native helper. The
/// result is the first helper's EAX on the nonzero branch and the follow-up
/// helper's EAX on the zero branch.
lf_checker_rt::export!(cdecl, rw_00bcd5b0(enabled_word: u32) -> u32 {
    const PROBE_CALLEE: u32 = 1;
    const TRIPLET_CALLEE: u32 = 2;
    const FOLLOWUP_CALLEE: u32 = 3;
    const RADIUS_BITS: u32 = 0x457A0000;

    if enabled_word & 0xFF != 0 {
        lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, 1u32)
    } else {
        let mut triplet = [0u32; 3];
        let triplet_pointer = triplet.as_mut_ptr() as u32;
        let _probe_result = lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, 0u32);
        let _triplet_result =
            lf_checker_rt::callee_cdecl!(TRIPLET_CALLEE, u32, triplet_pointer);
        lf_checker_rt::callee_cdecl!(
            FOLLOWUP_CALLEE,
            u32,
            triplet_pointer,
            RADIUS_BITS,
        )
    }
});
