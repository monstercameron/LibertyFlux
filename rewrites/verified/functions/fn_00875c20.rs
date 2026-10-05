// original: 0x00875C20 rage::crmtNodeExtrapolate::vf0

/// Scalar-deleting destructor: install vtable 0xfe81a8, run teardowns 0x875e30 and 0x872ba0 in order (thiscall, no arguments), free `this` through the thread manager when the flag bit is set, return `this`.
///
/// Original: 0x00875C20 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00875c20(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEAR_A: u32 = 1;
    const TEAR_B: u32 = 2;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe81a8));
        lf_checker_rt::callee_thiscall!(TEAR_A, u32, this);
        lf_checker_rt::callee_thiscall!(TEAR_B, u32, this);
        if flags & 1 != 0 {
            let tls0 = lf_checker_rt::tls_slot(0);
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free(manager, this);
        }
    }
    this
});
