// original: 0x00874F20 rage::crmtRequestBlendN::vf0

/// Scalar-deleting destructor of a wide blend request: install vtable 0xfe812c, release the children at +0x198 and +0x194 through vtable slot +8 when non-null (clearing each slot after its release), run teardowns 0x875140 and 0x874c80, free `this` when the flag bit is set, return `this`.
///
/// Original: 0x00874F20 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00874f20(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEAR_A: u32 = 2;
    const TEAR_B: u32 = 3;
    const RELEASE_SLOT: u32 = 8;
    unsafe {
        let ca = ((this + 0x198) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe812c));
        if ca != 0 {
            let vt = (ca as *const u32).read_unaligned();
            let tgt = ((vt as *const u8).add(RELEASE_SLOT as usize) as *const u32)
                .read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(ca);
        }
        let cb = ((this + 0x194) as *const u32).read_unaligned();
        ((this + 0x198) as *mut u32).write_unaligned(0);
        if cb != 0 {
            let vt = (cb as *const u32).read_unaligned();
            let tgt = ((vt as *const u8).add(RELEASE_SLOT as usize) as *const u32)
                .read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(cb);
        }
        ((this + 0x194) as *mut u32).write_unaligned(0);
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
