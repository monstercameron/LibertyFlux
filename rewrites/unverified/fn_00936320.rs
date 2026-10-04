// original: 0x00936320 net_rate_classify (proposed)

/// Classify a rate sample into one of nine codes by eight thresholds.
///
/// The sample (an f32 passed by bits) is compared against the ascending
/// thresholds 15, 40, 70, 150, 300, 500, 1000 and 2000 kept in read-only
/// data; the first threshold strictly greater than the sample selects the
/// code, a sample at or above all of them (including NaN and +inf, which
/// fail every ordered comparison) takes the last code. Codes are 0, 1, 2,
/// 3, 4, 6, 8, 9 and 0x0a: 5 and 7 are skipped. Each selection logs a
/// per-branch tag with the shared table address through one callee, then
/// the code is stored to `out` unless it is null. Always returns the table
/// address. stdcall: two stack words, callee cleans up.
lf_checker_rt::export!(stdcall, rw_00936320(sample: u32, out: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x11A4EA0;
        const TH: [u32; 8] = [
            0xFE8B20, 0xFE8B5C, 0xFE8B90, 0xFE8BD0, 0xFE8C10, 0xFE8C2C, 0xFE8C58, 0xE77F94,
        ];
        const TAG: [u32; 9] = [
            0xE879DC, 0xE879E8, 0xE879F4, 0xE87A00, 0xE87A0C, 0xE87A18, 0xE87A24, 0xE87A34, 0xE87A3C,
        ];
        const CODE: [u32; 9] = [0, 1, 2, 3, 4, 6, 8, 9, 0x0A];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let x = f32::from_bits(sample);
        let mut sel = 8usize;
        let mut i = 0usize;
        while i < 8 {
            let t = f32::from_bits(rd32(lf_checker_rt::relocated(TH[i])));
            if t > x {
                sel = i;
                break;
            }
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(1, u32, TABLE, TAG[sel]);
        let code = CODE[sel];
        if out != 0 {
            wr32(out, code);
        }
        TABLE
    }
});
