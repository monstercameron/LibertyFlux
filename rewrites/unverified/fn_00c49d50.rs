// original: 0x00C49D50 camera_spline_forward_frame_setup

/// Forward the spline camera's frame value and two stack-backed inputs.
///
/// The object pointer is in ECX and one scalar argument is on the stack. The
/// function calls its helper with a raw float-limit constant, a
/// pointer to a saved copy of the object pointer, a pointer to a saved copy of
/// the scalar argument, and the scalar argument itself. It returns the
/// helper's 32-bit result and pops its one stack argument. The call remains
/// visible to the checker; the two local pointer addresses differ across
/// executions, so the contract compares the pointed-to words at call time.
lf_checker_rt::export!(thiscall, rw_c49d50(this: u32, frame_value: u32) -> u32 {
    const HELPER: u32 = 1;
    const FRAME_LIMIT_BITS: u32 = 0x4974_23f0;

    let mut saved_this = this;
    let mut saved_value = frame_value;
    let this_pointer = core::ptr::addr_of_mut!(saved_this) as usize as u32;
    let value_pointer = core::ptr::addr_of_mut!(saved_value) as usize as u32;

    lf_checker_rt::callee_stdcall!(
        HELPER,
        u32,
        FRAME_LIMIT_BITS,
        this_pointer,
        value_pointer,
        frame_value,
    )
});
