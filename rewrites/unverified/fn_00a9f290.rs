// original: 0x00A9F290 stream_selection_value_150
/// Selects one of two float banks from bits 1 and 3 of a value in the
/// configured thread-local block. Each bank begins one 0xF0-byte
/// record apart; this getter returns the single-precision field at
/// byte offset 0x150 from the chosen record.
lf_checker_rt::export!(thiscall, rw_00A9F290(this: u32) -> f32 {
    unsafe {
        const TLS_INDEX_RVA: u32 = 0x17ABA14;
        const BANK_ONE_RVA: u32 = 0x1174794;
        const BANK_TWO_RVA: u32 = 0x1174790;
        const TLS_FLAGS_OFFSET: u32 = 0x8D0;
        const BANK_STRIDE: u32 = 0xF0;
        const FIELD_OFFSET: u32 = 0x150;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let tls_index = lf_checker_rt::global::<u32>(TLS_INDEX_RVA)
            .read_unaligned() as usize;
        let tls_block = lf_checker_rt::tls_slot(tls_index);
        let selector = read_u32(tls_block.wrapping_add(TLS_FLAGS_OFFSET));
        let bank_index = if selector & 0x2 != 0 || selector & 0x8 != 0 {
            lf_checker_rt::global::<u32>(BANK_TWO_RVA).read_unaligned()
        } else {
            lf_checker_rt::global::<u32>(BANK_ONE_RVA).read_unaligned()
        };
        let field = this
            .wrapping_add(bank_index.wrapping_mul(BANK_STRIDE))
            .wrapping_add(FIELD_OFFSET);
        f32::from_bits(read_u32(field))
    }
});
