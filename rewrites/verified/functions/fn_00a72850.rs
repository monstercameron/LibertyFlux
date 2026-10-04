// original: 0x00a72850 compare_xor_pairs (proposed)
/// Compares two xor-ed byte pairs from the resolved state against a
/// threshold byte, under a global mode switch.
///
/// `cdecl`, no stack words. Fetches the record (callee 1) and the state
/// (callee 2 on the record); either null returns 0. Reads threshold `bh`
/// from its global. When the mode global is non-zero and the mode check
/// (callee 3) passes, the pair difference is forced to 0 with a flag set;
/// otherwise the difference is `byte[+0x26ee] ^ byte[+0x26ec]`. The
/// magnitude is `byte[+0x26de] ^ byte[+0x26dc]`: when it exceeds the
/// difference the result is `magnitude >= bh`; otherwise, with the flag
/// set, `0 >= bh`, and without it the recomputed difference `>= bh`.
lf_checker_rt::export!(cdecl, rw_00a72850() -> u8 {
    unsafe {
        const MODE_FLAG: u32 = 0x018b6ed7;
        const THRESH: u32 = 0x0103ce48;
        const MAG_HI: u32 = 0x26de;
        const MAG_LO: u32 = 0x26dc;
        const DIF_HI: u32 = 0x26ee;
        const DIF_LO: u32 = 0x26ec;
        let rb = |p: u32| (p as *const u8).read();
        let rec = lf_checker_rt::callee_cdecl!(1, u32,);
        if rec == 0 {
            return 0;
        }
        let st = lf_checker_rt::callee_thiscall!(2, u32, rec);
        if st == 0 {
            return 0;
        }
        let bh = rb(lf_checker_rt::relocated(THRESH));
        let (diff, forced) =
            if rb(lf_checker_rt::relocated(MODE_FLAG)) != 0
                && lf_checker_rt::callee_cdecl!(3, u32,) as u8 != 0
            {
                (0u32, true)
            } else {
                (u32::from(rb(st + DIF_HI) ^ rb(st + DIF_LO)), false)
            };
        let mag = u32::from(rb(st + MAG_HI) ^ rb(st + MAG_LO));
        if mag > diff {
            return (mag >= u32::from(bh)) as u8;
        }
        if forced {
            return (0u32 >= u32::from(bh)) as u8;
        }
        let d2 = u32::from(rb(st + DIF_HI) ^ rb(st + DIF_LO));
        (d2 >= u32::from(bh)) as u8
    }
});
