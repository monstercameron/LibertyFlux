// original: 0x00969210 copy_worker_timing_state

/// Stores the low byte of `state` at object offset `0x2f50`, then copies the
/// twelve selected words from the timing row indexed by TLS field `+0x70`.
/// The row stride is 64 bytes; skipped words remain untouched. EAX returns
/// the last copied word.
lf_checker_rt::export!(thiscall, rw_00969210(this: u32, state: u32) -> u32 {
    const TLS_INDEX_GLOBAL: u32 = 0x017aba14;
    const ROW_TABLE: u32 = 0x0115def0;
    const ROW_STRIDE: u32 = 0x40;
    const STATE_BYTE: u32 = 0x2f50;
    const COPY_FIELDS: [(u32, u32); 12] = [
        (0x00, 0x2f60), (0x04, 0x2f64), (0x08, 0x2f68),
        (0x10, 0x2f70), (0x14, 0x2f74), (0x18, 0x2f78),
        (0x20, 0x2f80), (0x24, 0x2f84), (0x28, 0x2f88),
        (0x30, 0x2f90), (0x34, 0x2f94), (0x38, 0x2f98),
    ];

    unsafe {
        (this.wrapping_add(STATE_BYTE) as *mut u8).write(state as u8);
        let slot_index = lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL).read_unaligned() as usize;
        let thread_object = lf_checker_rt::tls_slot(slot_index);
        let row_index = (thread_object.wrapping_add(0x70) as *const u32).read_unaligned();
        let row = lf_checker_rt::relocated(ROW_TABLE).wrapping_add(row_index.wrapping_mul(ROW_STRIDE));
        let mut last = 0u32;
        for (source_offset, target_offset) in COPY_FIELDS {
            last = (row.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this.wrapping_add(target_offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
