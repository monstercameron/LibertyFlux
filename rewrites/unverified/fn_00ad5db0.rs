// original: 0x00AD5DB0 thread_float_get

/// Read a float selected by the current thread's state word. The TLS index
/// comes from the image global at `+0x13ABA14`; the object at that slot has
/// its selection bits at `+0x8D0`. Bit 1 or bit 3 selects the alternate
/// global table index; otherwise the default index is used. The selected
/// single-precision value is returned in ST(0).
lf_checker_rt::export!(cdecl, rw_00ad5db0() -> f32 {
    const TLS_INDEX: u32 = 0x017ABA14;
    const TLS_STATE_OFFSET: u32 = 0x8D0;
    const DEFAULT_INDEX: u32 = 0x01174794;
    const ALTERNATE_INDEX: u32 = 0x01174790;
    const FLOAT_TABLE: u32 = 0x0154EC50;

    let slot = unsafe { lf_checker_rt::global::<u32>(TLS_INDEX).read_volatile() as usize };
    let state = lf_checker_rt::tls_slot(slot);
    let state_word = unsafe { (state.wrapping_add(TLS_STATE_OFFSET) as *const u32).read_unaligned() };
    let alternate = state_word & ((1 << 1) | (1 << 3)) != 0;
    let table_index_address = if alternate { ALTERNATE_INDEX } else { DEFAULT_INDEX };
    let table_index = unsafe { lf_checker_rt::global::<u32>(table_index_address).read_volatile() };
    let value_address = FLOAT_TABLE.wrapping_add(table_index.wrapping_mul(4));
    unsafe { lf_checker_rt::global::<f32>(value_address).read_volatile() }
});
