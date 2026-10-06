// original: 0x00A2E370 files_mem_register_blocks (proposed)

/// Register the files-memory subsystem's global blocks with a registrar callee.
///
/// Calls the registrar once per block (133 calls in total) with the block's
/// address and its size in bytes: one 4-byte header, three strided arrays
/// (10 rows of 16+4+4 bytes, then 20 rows of 16+4+4 bytes), a second header,
/// six 64-byte records described field by field (16, 4, 32, 4 and 1 byte),
/// and ten trailing blocks. The callee's answers are ignored; the function
/// returns 1 in the low byte while the upper three bytes keep whatever the
/// last call left in the accumulator.
///
/// Original: 0x00A2E370 (cdecl, no arguments, plain `ret`).
lf_checker_rt::export!(cdecl, rw_00A2E370() -> u32 {
    unsafe {
        const REGISTRAR: u32 = 1;
        const HEADER_A: u32 = 0x012D_D690;
        const ROWS10_BASE: u32 = 0x012D_DC80;
        const ROWS10_B: u32 = 0x012D_D640;
        const ROWS10_C: u32 = 0x012D_D668;
        const HEADER_B: u32 = 0x012D_D694;
        const ROWS20_BASE: u32 = 0x012D_DD20;
        const ROWS20_B: u32 = 0x012D_D698;
        const ROWS20_C: u32 = 0x012D_D6E8;
        const HEADER_C: u32 = 0x012D_D738;
        const REC_BASE: u32 = 0x012D_DB00;
        const REC_STRIDE: u32 = 0x40;

        #[inline(always)]
        unsafe fn reg(addr: u32, size: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_cdecl!(
                    REGISTRAR,
                    u32,
                    lf_checker_rt::relocated(addr),
                    size
                )
            }
        }

        let mut last: u32 = 0;
        last = reg(HEADER_A, 4);
        for i in 0..10u32 {
            last = reg(ROWS10_BASE + i * 0x10, 0x10);
            last = reg(ROWS10_B + i * 4, 4);
            last = reg(ROWS10_C + i * 4, 4);
        }
        last = reg(HEADER_B, 4);
        for i in 0..20u32 {
            last = reg(ROWS20_BASE + i * 0x10, 0x10);
            last = reg(ROWS20_B + i * 4, 4);
            last = reg(ROWS20_C + i * 4, 4);
        }
        last = reg(HEADER_C, 4);
        for i in 0..6u32 {
            let base = REC_BASE + i * REC_STRIDE;
            last = reg(base, 0x10);
            last = reg(base + 0x10, 4);
            last = reg(base + 0x14, 0x20);
            last = reg(base + 0x34, 4);
            last = reg(base + 0x38, 1);
        }
        const TAIL: [(u32, u32); 10] = [
            (0x012D_D8C0, 1),
            (0x012D_DE60, 0x10),
            (0x012D_D8C4, 4),
            (0x012D_D8C1, 1),
            (0x012D_DE70, 0x10),
            (0x012D_D8C8, 4),
            (0x012D_D8CC, 4),
            (0x012D_D8F0, 0x10),
            (0x012D_D8D0, 4),
            (0x012D_D8D4, 4),
        ];
        for (addr, size) in TAIL {
            last = reg(addr, size);
        }
        (last & 0xFFFF_FF00) | 1
    }
});
