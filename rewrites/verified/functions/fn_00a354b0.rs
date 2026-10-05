// original: 0x00a354b0 vehicle_event_build_a (proposed)

/// Build a vehicle event with the flag word cleared.
///
/// Forwards seven arguments to the 8-word event builder (id 1) as
/// `(a, b, 0, c, e, f, g, d)`: the third word is the constant 0 and the
/// last is the outer fourth argument. Cdecl/7, returns the builder's
/// answer.
lf_checker_rt::export!(cdecl, rw_00a354b0(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    unsafe {
        const BUILDER: u32 = 1;
        lf_checker_rt::callee_cdecl!(BUILDER, u32, a, b, 0, c, e, f, g, d)
    }
});
