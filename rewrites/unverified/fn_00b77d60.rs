// original: 0x00b77d60 slot_release_inner (proposed)

/// Release the inner object of a slot and clear the slot.
///
/// Reads the inner pointer at `this`. When it is null there is nothing to
/// do (the original returns whatever the caller left in `eax`; the contract
/// fixes entry `eax` to 0 so both sides agree). Otherwise the flag byte of
/// the inner object selects one of two release callees, called with the
/// inner pointer, and the slot is cleared afterwards.
///
/// Original: thiscall, no stack words, plain `ret`.
lf_checker_rt::export!(thiscall, rw_00b77d60(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x28;
        const RELEASE_SET: u32 = 1;
        const RELEASE_CLEAR: u32 = 2;
        let inner = (this as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let r: u32 = if ((inner + FLAG) as *const u8).read() == 1 {
            lf_checker_rt::callee_thiscall!(RELEASE_SET, u32, inner)
        } else {
            lf_checker_rt::callee_thiscall!(RELEASE_CLEAR, u32, inner)
        };
        (this as *mut u32).write_unaligned(0);
        r
    }
});
