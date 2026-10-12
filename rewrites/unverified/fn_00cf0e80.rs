// original: 0x00CF0E80 selector_snap_value

/// Snap a selected value to a proposed value when it lies outside a fixed
/// inclusive window.
///
/// The cdecl arguments are a pointer to the current `f32` and a candidate
/// `f32`. The window width is read from the image's `f32` constant. The
/// candidate is stored only when it is strictly greater than `current +
/// width` or strictly less than `current - width`; equality stays in the
/// window. Ordered comparisons make NaN take the no-store path. The result is
/// a Boolean in AL, and the pointed-to word is otherwise left unchanged.
lf_checker_rt::export!(cdecl, rw_fn_00cf0e80(current_ptr: u32, candidate: f32) -> u32 {
    const WINDOW_WIDTH_VA: u32 = 0x00fe_8b5c;
    let width = unsafe { lf_checker_rt::global::<f32>(WINDOW_WIDTH_VA).read_unaligned() };
    let current = unsafe { (current_ptr as *const f32).read_unaligned() };
    let upper_edge = core::hint::black_box(current) + core::hint::black_box(width);
    if candidate > upper_edge {
        unsafe { (current_ptr as *mut f32).write_unaligned(candidate) };
        return 1;
    }
    let lower_edge = core::hint::black_box(current) - core::hint::black_box(width);
    if lower_edge > candidate {
        unsafe { (current_ptr as *mut f32).write_unaligned(candidate) };
        1
    } else {
        0
    }
});
