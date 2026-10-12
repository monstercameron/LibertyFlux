// original: 0x00BCD5B0 NativeImpl_SET_ALL_CARS_CAN_BE_DAMAGED_4

/// For a nonzero low-byte flag, call the queue helper with 1 and return its
/// result. For a zero flag, call it with 0, pass a temporary word to the
/// selected-triplet helper, then forward that helper's result to the final
/// setter and return the setter result. All three helper calls are cdecl.
lf_checker_rt::export!(cdecl, rw_00bcd5b0(enabled_word: u32) -> u32 {
    const QUEUE_HELPER: u32 = 1;
    const SELECTED_TRIPLET_HELPER: u32 = 2;
    const FINAL_SETTER_HELPER: u32 = 3;

    if enabled_word & 0xff != 0 {
        lf_checker_rt::callee_cdecl!(QUEUE_HELPER, u32, 1u32)
    } else {
        let _queued = lf_checker_rt::callee_cdecl!(QUEUE_HELPER, u32, 0u32);
        let mut selected_value = 0x457a_0000u32;
        let selected_ptr = core::ptr::addr_of_mut!(selected_value) as u32;
        let selected_result = lf_checker_rt::callee_cdecl!(
            SELECTED_TRIPLET_HELPER,
            u32,
            selected_ptr,
        );
        lf_checker_rt::callee_cdecl!(FINAL_SETTER_HELPER, u32, selected_result)
    }
});