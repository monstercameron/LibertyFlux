// original: 0x00a354e0 vehicle_event_build_b (proposed)

/// Build a vehicle event with the flag word set.
///
/// Twin of `vehicle_event_build_a`: forwards seven arguments to the 8-word
/// event builder (id 1) as `(a, b, 1, c, e, f, g, d)`. Cdecl/7, returns the
/// builder's answer.
lf_checker_rt::export!(cdecl, rw_00a354e0(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    unsafe {
        const BUILDER: u32 = 1;
        lf_checker_rt::callee_cdecl!(BUILDER, u32, a, b, 1, c, e, f, g, d)
    }
});
