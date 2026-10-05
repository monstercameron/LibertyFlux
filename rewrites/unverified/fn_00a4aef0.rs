// original: 0x00A4AEF0 vehicle_init_two_buffers (proposed)

/// Initializes two scratch buffers through one callee and combines them
/// through another, then returns 1.
///
/// Calls the filling callee twice (thiscall-style with `this` in `ecx`,
/// which the contract declares `noclean` because the original cleans the
/// stack itself: `(buffer_a, 4)` then `(buffer_b, 0x70)`), then the combining
/// callee with a third buffer in `ecx` and `this` on the stack (also
/// `noclean`: the original's frame restore cleans up). The second fill site
/// uses its own callee id: the stub clobbers `ecx`, so the original's `ecx`
/// at the second site is stub leftovers, not comparable, while the first
/// site's (`this`) is. The buffers are the
/// function's own frame slots; their addresses differ legitimately between
/// sides, so the contract skips the address arguments and snapshots the
/// pointed-to words instead (zeros on both sides: the stubs model no fills).
/// Always returns 1.
///
/// Original: 0x00A4AEF0 (thiscall, no stack words), two callees, frame args.
lf_checker_rt::export!(thiscall, rw_00A4AEF0(this: u32) -> u32 {
    unsafe {
        const FILL_CALLEE: u32 = 1;
        const FILL_SECOND: u32 = 3;
        const COMBINE_CALLEE: u32 = 2;
        const FILL_A_TAG: u32 = 4;
        const FILL_B_TAG: u32 = 0x70;
        let mut buf_a = [0u32; 4];
        let mut buf_b = [0u32; 4];
        let mut buf_c = [0u32; 4];
        lf_checker_rt::callee_thiscall!(
            FILL_CALLEE,
            u32,
            this,
            buf_a.as_mut_ptr() as u32,
            FILL_A_TAG
        );
        lf_checker_rt::callee_thiscall!(
            FILL_SECOND,
            u32,
            this,
            buf_b.as_mut_ptr() as u32,
            FILL_B_TAG
        );
        lf_checker_rt::callee_thiscall!(
            COMBINE_CALLEE,
            u32,
            buf_c.as_mut_ptr() as u32,
            this
        );
        1
    }
});
