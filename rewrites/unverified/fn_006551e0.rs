// original: 0x006551e0 ptx_random_table_refill (proposed)

/// Refill the 256-entry random float table from the 64-bit multiply-add
/// generator state. Each step multiplies the low word by `0x5cdcfaa7`, adds
/// the high word with carry into the upper half, masks the low 23 bits,
/// converts exactly to float and scales by the constant factor, storing the
/// float into the table and writing the new state words back. Clears the
/// ready flag byte first. Cdecl, no arguments; returns the last masked
/// integer (left in eax).
lf_checker_rt::export!(cdecl, rw_006551e0() -> u32 {
    unsafe {
        const MULTIPLIER: u64 = 0x5cdc_faa7;
        const MANTISSA_MASK: u32 = 0x7f_ffff;
        const TABLE_ENTRIES: usize = 256;
        const STATE_LO: u32 = 0x0111_01a0;
        const STATE_HI: u32 = 0x0111_01a4;
        const READY_FLAG: u32 = 0x018b_749a;
        const TABLE_BASE: u32 = 0x018b_7588;
        const SCALE: u32 = 0x00fe_864c;

        let scale = f32::from_bits(
            (lf_checker_rt::relocated(SCALE) as *const u32).read_unaligned(),
        );
        let mut low =
            (lf_checker_rt::relocated(STATE_LO) as *const u32).read_unaligned();
        let mut high =
            (lf_checker_rt::relocated(STATE_HI) as *const u32).read_unaligned();
        (lf_checker_rt::relocated(READY_FLAG) as *mut u8).write(0);
        let mut masked = 0u32;
        let mut slot = lf_checker_rt::relocated(TABLE_BASE);
        for _ in 0..TABLE_ENTRIES {
            let product = (low as u64).wrapping_mul(MULTIPLIER);
            let (next_low, carry) =
                (product as u32).overflowing_add(high);
            let next_high = ((product >> 32) as u32).wrapping_add(carry as u32);
            masked = next_low & MANTISSA_MASK;
            let scaled = core::hint::black_box(masked as f32)
                * core::hint::black_box(scale);
            high = next_high;
            (lf_checker_rt::relocated(STATE_LO) as *mut u32)
                .write_unaligned(next_low);
            (slot as *mut u32).write_unaligned(scaled.to_bits());
            (lf_checker_rt::relocated(STATE_HI) as *mut u32)
                .write_unaligned(next_high);
            low = next_low;
            slot = slot.wrapping_add(4);
        }
        masked
    }
});
