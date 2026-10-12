// original: 0x00b58510 dummy_object_init_tail (proposed)

/// Initialise a dummy object: install its virtual table, initialise the
/// sub-object at `+0x70` (scripted callee 1), then tail-call the base
/// teardown step (scripted callee 2) with `this` and return its result.
lf_checker_rt::export!(thiscall, rw_00b58510(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ea_f554;
        const SUBOBJ: u32 = 0x70;
        (this as *mut u32).write_unaligned(VTABLE);
        lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(SUBOBJ));
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
