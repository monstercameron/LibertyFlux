// original: 0x00c242a0 cam_interp_destroy (proposed)
/// Tear down a camera interpolator: install the base virtual table, run
/// the base destructor, drop the two reference-counted links (decrementing
/// the count byte at +0x13b of each target and nulling the field; note the
/// original nulls +0x14c for both, leaving +0x150 set, and that quirk is
/// reproduced), destroy and free the two owned sub-objects, then tail-call
/// the deallocation worker. Returns the worker's result.
///
/// Original: 0x00c242a0 (thiscall, no stack words; tail call).
lf_checker_rt::export!(thiscall, rw_00c242a0(obj: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ec57c0;
        const REFCOUNT_OFF: u32 = 0x13b;
        (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        let r0 = (obj.wrapping_add(0x14c) as *const u32).read_unaligned();
        if r0 != 0 {
            let p = r0.wrapping_add(REFCOUNT_OFF) as *mut u8;
            p.write(p.read().wrapping_sub(1));
            (obj.wrapping_add(0x14c) as *mut u32).write_unaligned(0);
        }
        let r1 = (obj.wrapping_add(0x150) as *const u32).read_unaligned();
        if r1 != 0 {
            let p = r1.wrapping_add(REFCOUNT_OFF) as *mut u8;
            p.write(p.read().wrapping_sub(1));
            // Quirk, as in the original: nulls +0x14c again, not +0x150.
            (obj.wrapping_add(0x14c) as *mut u32).write_unaligned(0);
        }
        for off in [0x154u32, 0x158] {
            let s = (obj.wrapping_add(off) as *const u32).read_unaligned();
            if s != 0 {
                lf_checker_rt::callee_thiscall!(2, u32, s);
                lf_checker_rt::callee_cdecl!(3, u32, s);
                (obj.wrapping_add(off) as *mut u32).write_unaligned(0);
            }
        }
        lf_checker_rt::callee_thiscall!(4, u32, obj)
    }
});
