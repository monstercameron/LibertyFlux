// original: 0x00DAB440 ped_probe_heading_full

/// Runs the initial nine-word probe gate, widens the two direction floats to
/// f64 for the heading helper, rounds its scripted result back to f32, and
/// forwards the heading difference to the smoothing helper. On the proved
/// zero-response route, it runs the six-word check and writes the failure
/// sentinel and clears the pointed-to output when that check returns false.
///
/// The contract fixes the first gate to true, the input pair to 0.5 and 1.0,
/// the heading slot to 2.0, the smoother response to zero, the final gate to
/// zero, and the six-word check to false. It scripts two finite double
/// answers and compares the low eight bytes of XMM0 and XMM1 at the heading
/// call. This proof covers that route only: the alternate first-gate route,
/// nonzero smoother response, trigonometric and second-heading path, and
/// successful six-word check are outside its contract. Stack-content checking
/// is disabled for local scratch, while ESP, calls, heap writes, return AL, and
/// undeclared writes remain checked.
#[inline(always)]
unsafe fn heading_read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn heading_write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

unsafe fn heading_failure(out_slot: u32, heading_slot: u32) -> u32 {
    const FAILURE_HEADING_BITS: u32 = 0xC61C_3F9A;
    if out_slot != 0 {
        unsafe { heading_write_u32(out_slot, 0) };
    }
    unsafe { heading_write_u32(heading_slot, FAILURE_HEADING_BITS) };
    0
}

unsafe fn heading_probe_stage(
    probe_context: u32,
    input_xy: u32,
    out_slot: u32,
    probe_secondary: u32,
    heading_slot: u32,
    probe_word: u32,
    probe_float_a: u32,
    probe_float_b: u32,
    check_word_a: u32,
    check_word_b: u32,
    final_gate: u32,
) -> u32 {
    const INITIAL_PROBE_ID: u32 = 1;
    const HEADING_ID: u32 = 2;
    const SMOOTH_ID: u32 = 3;
    const FINAL_CHECK_ID: u32 = 4;

    let initial_gate: u32 = lf_checker_rt::callee_cdecl!(
        INITIAL_PROBE_ID,
        u32,
        probe_context,
        input_xy,
        out_slot,
        probe_secondary,
        heading_slot,
        probe_word,
        probe_float_a,
        probe_float_b,
        final_gate
    );
    if initial_gate & 0xFF == 0 {
        return unsafe { heading_failure(out_slot, heading_slot) };
    }

    let atan_y = f32::from_bits(unsafe { heading_read_u32(input_xy.wrapping_add(4)) });
    let atan_x = f32::from_bits(unsafe { heading_read_u32(input_xy) });
    let y_as_f64 = (atan_y as f64).to_bits();
    let x_as_f64 = (atan_x as f64).to_bits();
    let heading_bits: u64 = lf_checker_rt::callee_cdecl!(
        HEADING_ID,
        u64,
        y_as_f64 as u32,
        (y_as_f64 >> 32) as u32,
        x_as_f64 as u32,
        (x_as_f64 >> 32) as u32
    );
    let angle = core::hint::black_box(f64::from_bits(heading_bits) as f32);
    let current_heading = f32::from_bits(unsafe { heading_read_u32(heading_slot) });
    let heading_delta = core::hint::black_box(current_heading) - core::hint::black_box(angle);
    let _smoothed_heading: f32 =
        lf_checker_rt::callee_cdecl!(SMOOTH_ID, f32, heading_delta.to_bits());

    let child = unsafe { heading_read_u32(out_slot) };
    let check_result: u32 = lf_checker_rt::callee_cdecl!(
        FINAL_CHECK_ID,
        u32,
        probe_context,
        probe_secondary,
        current_heading.to_bits(),
        check_word_a,
        check_word_b,
        child
    );
    if check_result & 0xFF == 0 && final_gate as u8 == 0 {
        return unsafe { heading_failure(out_slot, heading_slot) };
    }
    0
}

lf_checker_rt::export!(cdecl, rw_00DAB440(
    probe_context: u32,
    input_xy: u32,
    out_slot: u32,
    probe_secondary: u32,
    heading_slot: u32,
    probe_word: u32,
    probe_float_a: u32,
    probe_float_b: u32,
    check_word_a: u32,
    check_word_b: u32,
    final_gate: u32
) -> u32 {
    unsafe {
        heading_probe_stage(
            probe_context,
            input_xy,
            out_slot,
            probe_secondary,
            heading_slot,
            probe_word,
            probe_float_a,
            probe_float_b,
            check_word_a,
            check_word_b,
            final_gate
        )
    }
});
