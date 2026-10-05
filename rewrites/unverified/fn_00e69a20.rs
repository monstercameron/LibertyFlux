// original: 0x00e69a20 veh_table_bulk_init_01 (proposed)

/// Initialises sixteen vehicle records from three shared float globals.
///
/// Loads the floats `G0`, `G1`, `G2` once, then fills `COUNT` records of
/// `STRIDE` bytes starting at `BASE`: each record gets a zero word at
/// `-0x18`, the triple (`G2`, `G1`, `G0`) at offsets `-8`, `-4`, `0` and
/// repeated at `+8`, `+12`, `+16` and `+24`, `+28`, `+32`, zero words at
/// `+0x28`, `+0x2C`, `+0x30`, the half-word `0xFFFF` at `+0x34`, and zero
/// byte and word at `+0x38`, `+0x3A`. The floats move as bit patterns,
/// so NaN payloads survive unchanged. Takes no arguments, returns
/// nothing (cdecl/0); no calls.
///
/// Original: 0x00E69A20 (cdecl, no arguments, no calls).
lf_checker_rt::export!(cdecl, rw_00e69a20() -> () {
    unsafe {
        /// Shared float globals (file VAs), loaded once.
        const G0: u32 = 0x01B4B328;
        const G1: u32 = 0x01B4B324;
        const G2: u32 = 0x01B4B320;
        /// First record (file VA).
        const BASE: u32 = 0x0166FF28;
        /// Number of records.
        const COUNT: u32 = 16;
        /// Record size in bytes.
        const STRIDE: u32 = 0x60;
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let g0 = (lf_checker_rt::global::<u32>(G0) as *const u32).read_unaligned();
        let g1 = (lf_checker_rt::global::<u32>(G1) as *const u32).read_unaligned();
        let g2 = (lf_checker_rt::global::<u32>(G2) as *const u32).read_unaligned();
        let mut rec = lf_checker_rt::relocated(BASE);
        for _ in 0..COUNT {
            wr32(rec.wrapping_sub(0x18), 0);
            wr32(rec.wrapping_sub(8), g2);
            wr32(rec.wrapping_sub(4), g1);
            wr32(rec, g0);
            wr32(rec + 8, g2);
            wr32(rec + 12, g1);
            wr32(rec + 16, g0);
            wr32(rec + 24, g2);
            wr32(rec + 28, g1);
            wr32(rec + 32, g0);
            wr32(rec + 0x28, 0);
            wr32(rec + 0x2C, 0);
            wr32(rec + 0x30, 0);
            wr32(rec + 0x34, 0xFFFF);
            ((rec + 0x38) as *mut u8).write_unaligned(0);
            ((rec + 0x3A) as *mut u16).write_unaligned(0);
            rec = rec.wrapping_add(STRIDE);
        }
    }
});
