// original: 0x00875230 rage::crmtRequestSource<202>::vf0

/// Scalar-deleting destructor of a source request: run teardown 0x875140, then behave like the plain request destructor on the sub-object at +4 (vtable 0xfe7fc8, two guarded member teardowns through 0x872800, vtables 0xfe7fb4 and 0xe86afc, conditional free, return `this`).
///
/// Original: 0x00875230 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00875230(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEAR_FIRST: u32 = 1;
    const TEARDOWN: u32 = 2;
    unsafe {
        lf_checker_rt::callee_thiscall!(TEAR_FIRST, u32, this);
        let sub = this + 4;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe7fc8));
        let m = ((sub + 8) as *const u32).read_unaligned();
        if m != 0 {
            lf_checker_rt::callee_thiscall!(TEARDOWN, u32, m, sub);
        }
        let m2 = ((sub + 8) as *const u32).read_unaligned();
        (sub as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe7fb4));
        if m2 != 0 {
            lf_checker_rt::callee_thiscall!(TEARDOWN, u32, m2, sub);
        }
        (sub as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00e86afc));
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
