// original: 0x0094D730 wipe_and_link_records (proposed)

/// Wipe 100 records through callee 1, linking the changed ones via callee 2.
///
/// For each of the 100 records of 0x30 bytes at `this + 8`: callee 1 runs
/// with (record - 8, 0x30). Unless the gate byte `GATE` is non-zero, the
/// words at record - 4 (`prev`) and record + 0 (`cur`) are compared: when
/// `prev` is non-zero, or `cur` differs from it (both full-word compares),
/// callee 2 runs with (`prev`, `cur`, record + 4) and its answer is stored
/// at record + 0x24. Always returns 1 in the low byte (only `al` defined).
///
/// Original: 0x0094D730 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094D730(this: u32) -> u32 {
    unsafe {
        const RECS: u32 = 100;
        const STRIDE: u32 = 0x30;
        const GATE: u32 = 0x116D27D;
        const WIPE: u32 = 1;
        const LINK: u32 = 2;
        let mut i = 0u32;
        while i < RECS {
            let rec = this
                .wrapping_add(8)
                .wrapping_add(i.wrapping_mul(STRIDE));
            lf_checker_rt::callee_cdecl!(WIPE, u32, rec.wrapping_sub(8), 0x30);
            let gate = (lf_checker_rt::relocated(GATE) as *const u8).read();
            if gate == 0 {
                let prev = (rec.wrapping_sub(4) as *const u32).read_unaligned();
                let cur = (rec as *const u32).read_unaligned();
                if prev != 0 || cur != prev {
                    let out = rec.wrapping_add(4);
                    let r = lf_checker_rt::callee_cdecl!(LINK, u32, prev, cur, out);
                    (rec.wrapping_add(0x24) as *mut u32).write_unaligned(r);
                }
            }
            i += 1;
        }
        1
    }
});
