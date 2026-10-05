// original: 0x008759F0 rage::crmtNodeExpression::vf0

/// Scalar-deleting destructor: install vtable 0xfe816c, release the child at +0x20 through vtable slot +8 when non-null and clear the slot, run teardowns 0x872d00 and 0x872880, clear +0xc/+0x10 and clear +8 when non-null, run teardown 0x872ba0, free `this` when the flag bit is set, return `this`.
///
/// Original: 0x008759F0 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008759f0(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEAR_A: u32 = 2;
    const TEAR_B: u32 = 3;
    const TEAR_C: u32 = 4;
    const RELEASE_SLOT: u32 = 8;
    unsafe {
        let ch = ((this + 0x20) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe816c));
        if ch != 0 {
            let vt = (ch as *const u32).read_unaligned();
            let tgt = ((vt as *const u8).add(RELEASE_SLOT as usize) as *const u32)
                .read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(ch);
        }
        ((this + 0x20) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(TEAR_A, u32, this);
        lf_checker_rt::callee_thiscall!(TEAR_B, u32, this);
        let w8 = ((this + 8) as *const u32).read_unaligned();
        ((this + 0xc) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if w8 != 0 {
            ((this + 8) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(TEAR_C, u32, this);
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
