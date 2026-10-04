// original: 0x009FD3D0 frag_side_b_callback (proposed)

/// Run the side-B scheduled callback for record `rec` with carry `carry`.
///
/// When the record's flag dword is nonzero there is nothing to do and the
/// incoming `eax` (pinned to zero by the contract, as it is caller residue)
/// is returned. Otherwise the side-B delta global and `carry` are offered
/// to the matcher callee on the record's sub-object at `+0x10`; when the
/// matcher reports zero its answer is returned. On a match, the allocator
/// callee runs on the side-B allocator object with `rec` (the original also
/// pushes a constant second word, which the callee provably does not pop —
/// the caller's cleanup accounts for it — so it is uncompared and not
/// pushed here), its result goes to the side-B follow-up callee, then the
/// notifier callee runs with 1, whose answer is returned.
///
/// Original: 0x009FD3D0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_009FD3D0(carry: u32, rec: u32) -> u32 {
    unsafe {
        const DELTA: u32 = 0x016EC7AC;
        const ALLOCATOR: u32 = 0x016EC7B8;
        const SUB_OFF: u32 = 0x10;
        if (rec as *const u32).read_unaligned() != 0 {
            return 0;
        }
        let d = (lf_checker_rt::relocated(DELTA) as *const f32).read_unaligned();
        let r = lf_checker_rt::callee_thiscall!(1, u32, rec + SUB_OFF, carry, d.to_bits());
        if (r as u8) == 0 {
            return r;
        }
        let a = (lf_checker_rt::relocated(ALLOCATOR) as *const u32).read_unaligned();
        let m = lf_checker_rt::callee_thiscall!(2, u32, a, rec);
        lf_checker_rt::callee_cdecl!(3, u32, m);
        lf_checker_rt::callee_cdecl!(4, u32, 1)
    }
});
