// original: 0x008e0770 assign_retained_slot (proposed)

/// Assign a retained object to a slot, releasing the previous one.
///
/// Takes a reference on `new` (when non-null) by bumping its count word at
/// `+0x0c`, drops the reference held at `holder + 8` (running its virtual
/// release, slot 0, with argument 1 through callee 1 when the count reaches
/// zero), and stores `new` into the slot. Cdecl, two stack arguments.
lf_checker_rt::export!(cdecl, rw_008e0770(holder: u32, new: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 8;
        const REFCOUNT_OFF: u32 = 0x0c;
        const RELEASE_ARG: u32 = 1;
        if new != 0 {
            let rc = (new + REFCOUNT_OFF) as *mut u32;
            rc.write_unaligned(rc.read_unaligned().wrapping_add(1));
        }
        let old = ((holder + SLOT_OFF) as *const u32).read_unaligned();
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
        ((holder + SLOT_OFF) as *mut u32).write_unaligned(new);
        0
    }
});
