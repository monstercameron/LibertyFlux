// original: 0x0092D000 CSetCurrentViewportToNULL::vf1 (symbols)

/// Virtual slot 1 of `CSetCurrentViewportToNULL`: reset the current viewport.
///
/// Calls the viewport helper (callee 1) with arguments `(0, 1)` and returns
/// its result. The pushes in the original are `1` then `0`, so the first
/// argument is 0 and the second is 1.
///
/// Original: 0x0092D000 (cdecl, no arguments). One direct call.
lf_checker_rt::export!(cdecl, rw_0092D000() -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, 0u32, 1u32)
});
