// original: 0x00E66E20 scale_index_to_ptr

/// Scale a global index by 2048 and add a table base, storing the pointer.
///
/// Loads the `u32` at `IDX`, shifts it left by 11 (multiplying by 2048 with
/// wraparound), adds the constant table base `TABLE_BASE`, and stores the
/// result at `OUT`. The base is loader-relocated in the original, so the
/// rewrite derives it from the relocated image base.
///
/// Original: 0x00E66E20 (cdecl, no arguments, no outgoing calls, no return value).
lf_checker_rt::export!(cdecl, rw_00e66e20() -> u32 {
    unsafe {
        const IDX: u32 = 0x012B9000;
        const OUT: u32 = 0x012B9084;
        const TABLE_BASE: u32 = 0x012B8000;
        const SHIFT: u32 = 11;
        let v = (lf_checker_rt::global::<u32>(IDX)).read_unaligned();
        let p = v.wrapping_shl(SHIFT).wrapping_add(lf_checker_rt::relocated(TABLE_BASE));
        (lf_checker_rt::global::<u32>(OUT)).write_unaligned(p);
    }
    0
});
