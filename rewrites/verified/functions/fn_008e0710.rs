// original: 0x008e0710 swap_global_resource (proposed)

/// Publish a slot's object as the shared global resource.
///
/// Selects the new object from slot `index` through the pool context at
/// `CTX` (a set `0x80` flag bit resolves to null, whose head read faults on
/// both sides); index -1 selects null directly. The previous global is
/// released (its virtual release, slot 0, with argument 1 through callee 1
/// when its count reaches zero) after the new object is retained. Cdecl,
/// one stack argument.
lf_checker_rt::export!(cdecl, rw_008e0710(index: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const CURRENT: u32 = 0x01bb_5554;
        const FLAG_BASE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const REFCOUNT_OFF: u32 = 0x0c;
        const DEAD_FLAG: u8 = 0x80;
        const CLEAR: u32 = 0xffff_ffff;
        const RELEASE_ARG: u32 = 1;
        let new = if index == CLEAR {
            0
        } else {
            let ctx = lf_checker_rt::global::<u32>(CTX).read_unaligned();
            let flag_base = ((ctx + FLAG_BASE_OFF) as *const u32).read_unaligned();
            let stride = ((ctx + STRIDE_OFF) as *const u32).read_unaligned();
            let base = (ctx as *const u32).read_unaligned();
            let entry =
                if (flag_base.wrapping_add(index) as *const u8).read() & DEAD_FLAG != 0 {
                    0
                } else {
                    base.wrapping_add(stride.wrapping_mul(index))
                };
            // Kept opaque: a dead slot reads through null and faults, and
            // the compiler would otherwise fold the null load away.
            (core::hint::black_box(entry) as *const u32).read_unaligned()
        };
        let slot = lf_checker_rt::global::<u32>(CURRENT);
        let old = slot.read_unaligned();
        slot.write_unaligned(new);
        if new != 0 {
            let rc = (new + REFCOUNT_OFF) as *mut u32;
            rc.write_unaligned(rc.read_unaligned().wrapping_add(1));
        }
        if old != 0 {
            let rc = (old + REFCOUNT_OFF) as *mut u32;
            let left = rc.read_unaligned().wrapping_sub(1);
            rc.write_unaligned(left);
            if left == 0 {
                let vtable = (old as *const u32).read_unaligned();
                let release = (vtable as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(release as usize);
                f(old, RELEASE_ARG);
            }
        }
        0
    }
});
