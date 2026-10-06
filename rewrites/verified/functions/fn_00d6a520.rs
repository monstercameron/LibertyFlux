// original: 0x00d6a520 replay_button_dtor
/// Destructor of the replay button (original 0x00D6A520, thiscall/0).
///
/// Stamps the button vtable (file VA 0x00EEACF4), then releases the owned
/// member at `this+0x1c` when non-null (callee 1, then frees it through
/// callee 2, then clears the slot) and does the same for the shared object
/// held in the global at file VA 0x01797668. Stamps the dead vtable (file VA
/// 0x00EEA88C) last. Vtables are image addresses, stored relocated.
/// Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a520(this_ptr: u32) -> u32 {
    unsafe {
        const VTABLE_LIVE: u32 = 0x00EEACF4;
        const VTABLE_DEAD: u32 = 0x00EEA88C;
        const MEMBER_OFF: u32 = 0x1c;
        const SHARED_GLOBAL: u32 = 0x01797668;
        (this_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_LIVE));
        let member = ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        if member != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, member);
            lf_checker_rt::callee_cdecl!(2, u32, member);
            ((this_ptr + MEMBER_OFF) as *mut u32).write_unaligned(0);
        }
        let slot = lf_checker_rt::relocated(SHARED_GLOBAL) as *mut u32;
        let shared = slot.read_unaligned();
        if shared != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, shared);
            lf_checker_rt::callee_cdecl!(2, u32, shared);
            slot.write_unaligned(0);
        }
        (this_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_DEAD));
        0
    }
});
