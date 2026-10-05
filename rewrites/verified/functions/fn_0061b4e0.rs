// original: 0x0061B4E0 net_dispatch_by_index

/// Dispatch a record operation by table index.
///
/// `index` selects one of seven record banks at `this+0x4FD0+index*0x80`
/// (the original has no bounds check: the contract holds the index to
/// 0-6, and the rewrite masks defensively); `slot` selects the 64-byte
/// record within the bank. Forwards the record address and `value` to
/// the shared record routine. Returns the routine's answer.
/// Original: 0x0061B4E0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0061B4E0(this: u32, slot: u32, value: u32, index: u32) -> u32 {
    unsafe {
        const RECORD_CALLEE: u32 = 1;
        const BANK_BASE: u32 = 0x4FD0;
        const BANK_STRIDE: u32 = 0x80;
        const RECORD_SHIFT: u32 = 6;
        let bank = BANK_BASE + (index & 7) * BANK_STRIDE;
        let at = this
            .wrapping_add(bank)
            .wrapping_add(slot.wrapping_shl(RECORD_SHIFT));
        lf_checker_rt::callee_thiscall!(RECORD_CALLEE, u32, at, value)
    }
});
