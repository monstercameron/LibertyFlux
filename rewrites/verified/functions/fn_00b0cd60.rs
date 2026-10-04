// original: 0x00B0CD60 net_pool_init (proposed)

/// Initialise the pooled object table: reset all 1,500 entries, then set
/// the table header and the related mode words to their start-up values.
///
/// Each entry is `ENTRY_STRIDE` bytes at `POOL`. The per-entry reset (the
/// original's callee, inlined here because 1,500 intercepted calls would
/// exceed the call-log cap) zeroes words `0,4,8,C,14,18,1C,20,30,34,38`,
/// byte `44` and dword `48`, writes `-1` at word `10` and `1` at word
/// `42`, folds flag byte `45` to `((old & 0xFD) | 1) & 3` and masks byte
/// `46` with `0xF8`. Words `24..2C`, `40,41`, `47` and `4C..4F` keep
/// whatever they held. Word `3C` receives the callee's uninitialized
/// stack slot, pinned to zero by the contract's stack fill.
///
/// The header writes `-1` into the twenty dwords at `HDR_MINUS1`,
/// zeroes the ten qwords at `HDR_ZERO` plus the words at `HDR_W0A/B`,
/// and sets the scattered mode bytes and words (`MODE_BYTE`,
/// `MODE_LONG`, flag bytes `F0..F5`, `TAIL_COUNT`) to their constants.
///
/// Returns 1 (the inlined callee's `eax` for a nonzero argument).
///
/// The original's callee runs natively on the original side and is
/// inlined here, so this proof has no intercepted calls.
///
/// Original: 0x00B0CD60 (cdecl, no arguments, returns `eax`).
lf_checker_rt::export!(cdecl, rw_00B0CD60() -> u32 {
    unsafe {
        const POOL: u32 = 0x1615660;
        const ENTRIES: u32 = 1500;
        const ENTRY_STRIDE: u32 = 0x50;
        const HDR_MINUS1: u32 = 0x16155C8;
        const HDR_MINUS1_N: u32 = 20;
        const HDR_ZERO: u32 = 0x1615578;
        const HDR_ZERO_N: u32 = 10;
        const HDR_W0A: u32 = 0x1615574;
        const HDR_W0B: u32 = 0x161561C;
        const MODE_BYTE: u32 = 0x10400B8;
        const MODE_LONG: u32 = 0x10400B4;
        const TAIL_COUNT: u32 = 0x10400DC;
        const TAIL_COUNT_V: u32 = 0x578;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wg32(file_va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), v) }
        }
        #[inline(always)]
        unsafe fn wg8(file_va: u32, v: u8) {
            unsafe { wr8(lf_checker_rt::relocated(file_va), v) }
        }

        let pool = lf_checker_rt::relocated(POOL);
        let mut i = 0u32;
        while i < ENTRIES {
            let e = pool.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
            for off in [0u32, 4, 8, 0x0C, 0x14, 0x18, 0x1C, 0x20, 0x30, 0x34, 0x38, 0x48] {
                wr32(e + off, 0);
            }
            wr32(e + 0x10, 0xFFFF_FFFF);
            wr16(e + 0x42, 1);
            wr8(e + 0x44, 0);
            wr8(e + 0x45, ((rd8(e + 0x45) & 0xFD) | 1) & 3);
            wr8(e + 0x46, rd8(e + 0x46) & 0xF8);
            // Uninitialized stack word of the inlined callee: the contract
            // pins the stack fill to zero, so this is defined, not garbage.
            wr32(e + 0x3C, 0);
            i += 1;
        }
        let mut k = 0u32;
        while k < HDR_MINUS1_N {
            wg32(HDR_MINUS1 + k * 4, 0xFFFF_FFFF);
            k += 1;
        }
        let mut q = 0u32;
        while q < HDR_ZERO_N {
            wg32(HDR_ZERO + q * 8, 0);
            wg32(HDR_ZERO + q * 8 + 4, 0);
            q += 1;
        }
        wg32(HDR_W0A, 0);
        wg32(HDR_W0B, 0);
        wg8(MODE_BYTE, 0x0A);
        wg32(0x1615620, 0);
        wg32(MODE_LONG, 0xFFFF_FFFF);
        wg8(0x1615624, 0);
        wg8(0x1615626, 0);
        wg8(0x10400B9, 1);
        wg8(0x10400BA, 1);
        wg8(0x10400BB, 1);
        wg8(0x1615627, 0);
        wg8(0x10400BC, 1);
        wg8(0x1615628, 0);
        wg8(0x161562A, 0);
        wg8(0x1615629, 0);
        wg32(0x1615570, 0);
        wg32(0x1615618, 0);
        wg32(TAIL_COUNT, TAIL_COUNT_V);
        1
    }
});
