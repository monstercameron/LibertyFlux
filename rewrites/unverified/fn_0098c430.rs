// original: 0x0098C430 audio_thread_slot_ptr

/// Returns the per-thread audio slot address for the supplied index. The
/// game stores a TLS slot number in a global word; the selected TLS pointer's
/// word at offset 0x70 supplies the row base. Address arithmetic wraps at
/// 32 bits, matching the original 32-bit pointer layout.
lf_checker_rt::export!(thiscall, rw_audio_thread_slot_ptr(this: u32, index: u32) -> u32 {
    const TLS_INDEX_GLOBAL: u32 = 0x17AB_A14;
    const TLS_ROW_BASE: u32 = 0x70;
    const SLOT_BIAS: u32 = 0x51;

    unsafe {
        let tls_index = lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL).read_unaligned();
        let tls_block = lf_checker_rt::tls_slot(tls_index as usize);
        let row_base = (tls_block.wrapping_add(TLS_ROW_BASE) as *const u32).read_unaligned();
        let row = row_base.wrapping_add(index.wrapping_add(SLOT_BIAS).wrapping_mul(4));
        this.wrapping_add(row.wrapping_shl(4))
    }
});
