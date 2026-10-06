// original: 0x00b00780 tls_indexed_or_fallback
/// Thread-local slot value, or a fallback address.
///
/// cdecl `()`: reads the slot index from a global, loads that thread-local
/// slot's pointer, and when the flag word at `+0x8cc` is zero tail-calls
/// the default provider (helper 1). Otherwise returns the global base
/// plus `0x8e8` (wrapping). The slot index is pinned to fabricated slots
/// by the contract.
export!(cdecl, rw_00b00780() -> u32 {
    use lf_checker_rt::tls_slot;
    const INDEX: u32 = 0x017A_BA14;
    const BASE: u32 = 0x012F_B1B8;
    const FLAG_OFF: u32 = 0x8CC;
    const FALLBACK_OFF: u32 = 0x8E8;
    unsafe {
        let idx = *global::<u32>(INDEX);
        let obj = tls_slot(idx as usize);
        if ((obj + FLAG_OFF) as *const u32).read_unaligned() == 0 {
            callee_thiscall!(1, u32, idx)
        } else {
            (*global::<u32>(BASE)).wrapping_add(FALLBACK_OFF)
        }
    }
});
