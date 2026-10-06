// original: 0x00d6a9c0 replay_notify_if_present
/// Notify the owner when the slot is present (original 0x00D6A9C0,
/// thiscall/0, register-indirect call).
///
/// When the dword at `this+0x34` is nonzero, or it is zero but the dword at
/// `this+0x3c` is nonzero, calls the function pointer at `this+0x40`
/// (callee 1, one stack argument: `this+0x34`, caller cleans up) and returns
/// 1 in al. When both are zero it returns 0 in al. Only al is meaningful
/// (the original leaves the upper bytes of eax untouched). Both comparisons
/// are equality-against-zero checks.
lf_checker_rt::export!(thiscall, rw_00d6a9c0(this_ptr: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0x34;
        const AUX_OFF: u32 = 0x3c;
        const TARGET_OFF: u32 = 0x40;
        let lo = ((this_ptr + SLOT_OFF) as *const u32).read_unaligned();
        let fire = if lo != 0 {
            true
        } else {
            ((this_ptr + AUX_OFF) as *const u32).read_unaligned() != 0
        };
        if fire {
            let target =
                ((this_ptr + TARGET_OFF) as *const u32).read_unaligned();
            let f: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this_ptr + SLOT_OFF);
            1
        } else {
            0
        }
    }
});
