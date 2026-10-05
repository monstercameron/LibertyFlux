// original: 0x00AF0CE0 stream_totals_init (proposed)

/// Initialise the stream totals row and publish the joined float.
///
/// Runs the probe callee over six seed constants, maintaining five total
/// words (each a difference of earlier ones; all read back as written
/// since the stubbed callees have no other writers here), then joins two
/// more seeds, resolves the joined name to a float and stores it to the
/// published global. Returns the float bits.
///
/// Original: 0x00AF0CE0 (cdecl, no stack words, four direct callees).
lf_checker_rt::export!(cdecl, rw_00af0ce0() -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const STEP_CALLEE: u32 = 2;
        const JOIN_CALLEE: u32 = 3;
        const FLOAT_CALLEE: u32 = 4;
        const ACC: u32 = 0x015DBDEC;
        const T0: u32 = 0x015DBDF0;
        const T1: u32 = 0x015DBDF4;
        const T2: u32 = 0x015DBDF8;
        const T3: u32 = 0x015DBDFC;
        const PUBLISH: u32 = 0x0103F998;
        const TABLE_OBJ: u32 = 0x015DE394;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a)).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a)).write_unaligned(v) }
        }
        lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, lf_checker_rt::relocated(0x00EA817C));
        wr(ACC, 0);
        lf_checker_rt::callee_cdecl!(STEP_CALLEE, u32, lf_checker_rt::relocated(0x00EA818C));
        wr(T0, rd(ACC));
        lf_checker_rt::callee_cdecl!(STEP_CALLEE, u32, lf_checker_rt::relocated(0x00EA81A4));
        wr(T1, rd(ACC).wrapping_sub(rd(T0)));
        lf_checker_rt::callee_cdecl!(STEP_CALLEE, u32, lf_checker_rt::relocated(0x00EA81BC));
        wr(T2, rd(ACC).wrapping_sub(rd(T1)).wrapping_sub(rd(T0)));
        lf_checker_rt::callee_cdecl!(STEP_CALLEE, u32, lf_checker_rt::relocated(0x00EA81DC));
        wr(T3, rd(ACC).wrapping_sub(rd(T2)).wrapping_sub(rd(T1)).wrapping_sub(rd(T0)));
        lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, lf_checker_rt::relocated(0x00EA7952));
        let f = rd(PUBLISH);
        let h: u32 = lf_checker_rt::callee_cdecl!(
            JOIN_CALLEE, u32,
            lf_checker_rt::relocated(0x00EA8204),
            lf_checker_rt::relocated(0x00EA81F4));
        let flt: f32 = lf_checker_rt::callee_thiscall!(
            FLOAT_CALLEE, f32, lf_checker_rt::relocated(TABLE_OBJ), h, f);
        wr(PUBLISH, flt.to_bits());
        flt.to_bits()
    }
});
