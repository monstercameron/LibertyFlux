// original: 0x0096ACC0 timing_worker_record_at

/// Computes a 64-byte record address from the object base, the current TLS
/// record index at `+0x70`, and the argument scaled by four before the final
/// 64-byte stride. All index arithmetic wraps as 32-bit address arithmetic.
lf_checker_rt::export!(thiscall, rw_0096acc0(this: u32, index: u32) -> u32 {
    const TLS_INDEX_GLOBAL: u32 = 0x017aba14;
    unsafe {
        let slot_index = lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL).read_unaligned() as usize;
        let thread_object = lf_checker_rt::tls_slot(slot_index);
        let current_record = (thread_object.wrapping_add(0x70) as *const u32).read_unaligned();
        let scaled_index = current_record.wrapping_add(index.wrapping_mul(4));
        this.wrapping_add(scaled_index.wrapping_shl(6))
    }
});
