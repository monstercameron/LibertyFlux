// original: 0x00d29550 targeting_aux_rebind (proposed)

/// Rebind the auxiliary handle from the current target chain.
///
/// Follows `this + 0x24c` to the target block and its word at `+0x224` to the
/// data block, copies the word at `+0x260` there into `this + 0x240` and
/// clears `this + 0x244`; then, when the word at `this + 0x248` is nonzero,
/// passes its address to the release helper (intercepted) and clears it. The
/// original leaves `eax` untouched, so no return channel is compared.
///
/// Original: 0x00D29550 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d29550(this: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x24c;
        const DATA_OFF: u32 = 0x224;
        const VALUE_OFF: u32 = 0x260;
        const AUX_OFF: u32 = 0x240;
        const AUX2_OFF: u32 = 0x244;
        const REF_OFF: u32 = 0x248;
        let target = unsafe { ((this + TARGET_OFF) as *const u32).read_unaligned() };
        let data = unsafe { ((target + DATA_OFF) as *const u32).read_unaligned() };
        let value = unsafe { ((data + VALUE_OFF) as *const u32).read_unaligned() };
        unsafe { ((this + AUX_OFF) as *mut u32).write_unaligned(value) };
        unsafe { ((this + AUX2_OFF) as *mut u32).write_unaligned(0) };
        let slot = this + REF_OFF;
        if unsafe { (slot as *const u32).read_unaligned() } != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, slot);
            unsafe { (slot as *mut u32).write_unaligned(0) };
        }
        0
    }
});
