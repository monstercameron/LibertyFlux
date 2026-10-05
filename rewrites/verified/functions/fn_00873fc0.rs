// original: 0x00873FC0 rage::crmtMotionTree::vf0

/// Scalar-deleting destructor: install vtable 0xfe8084, run teardown 0x874060, install vtable 0xe86afc, free `this` when the flag bit is set, return `this`.
///
/// Original: 0x00873FC0 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00873fc0(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEARDOWN: u32 = 1;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe8084));
        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00e86afc));
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
