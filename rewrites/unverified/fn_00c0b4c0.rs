// original: 0x00C0B4C0 population_zone_presence_count_lookup (proposed)

/// Looks up one byte from a population presence table.
///
/// Takes no arguments. Reads three global words `a`, `b` and `c`, forms
/// the row `(a + 2 * b) * 0x47` with wrapping 32-bit arithmetic, and
/// returns the byte at `TABLE + row + c`, zero-extended. All arithmetic
/// wraps modulo 2^32 exactly as the original's address computation.
///
/// Original: 0x00C0B4C0 (cdecl, no arguments; globals at 0x0169C47C,
/// 0x0169C478 and 0x0169C480, table at 0x016884F8).
lf_checker_rt::export!(cdecl, rw_00c0b4c0() -> u32 {
    unsafe {
        /// First global word feeding the row computation.
        const G0: u32 = 0x0169_C47C;
        /// Second global word feeding the row computation.
        const G1: u32 = 0x0169_C478;
        /// Third global word added as the column offset.
        const G2: u32 = 0x0169_C480;
        /// Byte table the result is read from.
        const TABLE: u32 = 0x016884F8;
        /// Row width in bytes.
        const ROW: u32 = 0x47;

        let a = (lf_checker_rt::global::<u32>(G0) as *const u32).read();
        let b = (lf_checker_rt::global::<u32>(G1) as *const u32).read();
        let row = a.wrapping_add(b.wrapping_mul(2)).wrapping_mul(ROW);
        let col = (lf_checker_rt::global::<u32>(G2) as *const u32).read();
        let addr = TABLE.wrapping_add(row).wrapping_add(col);
        (lf_checker_rt::relocated(addr) as *const u8).read() as u32
    }
});
