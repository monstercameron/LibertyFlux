// original: 0x008F66D0 NativeImpl_GET_BUFFERED_ASCII

/// Fetch buffered character `idx` for this device into the word at `out`.
///
/// `*out` is zeroed as a dword first. The function then calls the 16-bit
/// lookup through its own incoming argument slot as the out-parameter (the
/// slot is zeroed first), and copies the resulting word into `*out`.
/// Returns the lookup's result unchanged. Convention: thiscall, two stack
/// words `(idx, out)`.
lf_checker_rt::export!(thiscall, rw_008f66d0(this: u32, idx: u32, out: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        (out as *mut u32).write_unaligned(0);
        let mut slot: u32 = 0;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            this,
            idx,
            core::ptr::addr_of_mut!(slot) as u32
        );
        (out as *mut u16).write_unaligned((slot & 0xFFFF) as u16);
        r
    }
});
