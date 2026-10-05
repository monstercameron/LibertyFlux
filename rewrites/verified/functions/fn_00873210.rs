// original: 0x00873210 rage::crmtObserverFunctor::vf0

/// Scalar-deleting destructor: install vtable 0xfe7fa0, free the member at +0x10 through the thread manager when non-null and clear +0x10/+0x14, install vtable 0xfe7fb4, run the member teardown (0x872800) when the word at +8 is non-null, install vtable 0xe86afc, free `this` when the flag bit is set, return `this`. The TLS manager pointer is read once and reused.
///
/// Original: 0x00873210 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00873210(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEARDOWN: u32 = 1;
    unsafe {
        let tls0 = lf_checker_rt::tls_slot(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe7fa0));
        let member = ((this + 0x10) as *const u32).read_unaligned();
        if member != 0 {
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free_it(manager, member);
        }
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        let m2 = ((this + 8) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe7fb4));
        if m2 != 0 {
            lf_checker_rt::callee_thiscall!(TEARDOWN, u32, m2, this);
        }
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00e86afc));
        if flags & 1 != 0 {
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
