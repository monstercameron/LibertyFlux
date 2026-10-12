// original: 0x005A65A0 stats_block_reset

/// Clear the renderer statistics block and return its base pointer.
///
/// This no-argument cdecl routine writes 23 zero 32-bit words, including
/// three adjacent pairs that form 64-bit counters, clears one byte flag, and
/// returns the relocated address of the block. The writes are to fixed image
/// globals, so the addresses are obtained through the runtime relocation API.
lf_checker_rt::export!(cdecl, rw_005a65a0() -> u32 {
    const STATS_BLOCK: u32 = 0x019D2E58;
    const ZERO_WORDS: [u32; 17] = [
        0x019D2E58, 0x019D2E5C, 0x019D2E60, 0x019D2E64,
        0x019D2E68, 0x019D2E6C, 0x019D2E70, 0x019D2E74,
        0x019D2E9C, 0x019D2EB8, 0x019D2EBC, 0x019D2EC0,
        0x019D2EC4, 0x019D2EC8, 0x019D2ECC, 0x019D2ED0, 0x019D2ED4,
    ];
    const ZERO_COUNTER_PAIRS: [u32; 3] = [0x019D2EA0, 0x019D2EA8, 0x019D2EB0];
    const READY_FLAG: u32 = 0x019D2ED8;

    unsafe {
        for address in ZERO_WORDS {
            lf_checker_rt::global::<u32>(address).write_unaligned(0);
        }
        for address in ZERO_COUNTER_PAIRS {
            lf_checker_rt::global::<u64>(address).write_unaligned(0);
        }
        lf_checker_rt::global::<u8>(READY_FLAG).write_unaligned(0);
    }
    lf_checker_rt::relocated(STATS_BLOCK)
});
