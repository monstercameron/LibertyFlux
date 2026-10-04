// original: 0x00d26960 ped_stat_base (proposed)

/// Read a ped's base stat: table byte scaled minus low bits scaled.
///
/// A null `arg` yields 0.3. Otherwise the signed word at `arg + 0x2e` selects
/// a row of the runtime stat table (fabricated by the contract) and the word
/// at `arg + 0x2c` supplies small bits: the result is
/// `(float)(row byte at +0xee) * 0.2 - (float)(word & 0xff) * 0.00078125`,
/// evaluated in that operation order.
///
/// Original: 0x00D26960 (cdecl, one stack argument). Returns the float in ST0.
lf_checker_rt::export!(cdecl, rw_00d26960(arg: u32) -> f32 {
    unsafe {
        const NULL_BITS: u32 = 0x3e99_999a; // 0.3f
        const ROW_SCALE_BITS: u32 = 0x3e4c_cccd; // 0.2f
        const BIT_SCALE_BITS: u32 = 0x3a4c_cccd; // 0.00078125f
        const INDEX_OFF: u32 = 0x2e;
        const BITS_OFF: u32 = 0x2c;
        const BITS_MASK: u32 = 0xff;
        const TABLE_VA: u32 = 0x0129_5cd8;
        const ROW_BYTE_OFF: u32 = 0xee;
        if arg == 0 {
            return f32::from_bits(NULL_BITS);
        }
        let index = unsafe { ((arg + INDEX_OFF) as *const i16).read_unaligned() } as i32;
        let table = lf_checker_rt::relocated(TABLE_VA);
        let row = unsafe { ((table + (index * 4) as u32) as *const u32).read_unaligned() };
        let cell = unsafe { ((row + ROW_BYTE_OFF) as *const u8).read() } as f32;
        let bits = unsafe { ((arg + BITS_OFF) as *const u16).read_unaligned() } as u32 & BITS_MASK;
        let hi = core::hint::black_box(cell) * core::hint::black_box(f32::from_bits(ROW_SCALE_BITS));
        let lo = core::hint::black_box(bits as f32) * core::hint::black_box(f32::from_bits(BIT_SCALE_BITS));
        core::hint::black_box(hi) - core::hint::black_box(lo)
    }
});
