// original: 0x00b59020 vehicle_factory_init (proposed)

/// Initialise a vehicle factory object: install its virtual table and run
/// the chained initialiser (a scripted callee) with `this` still in place.
/// Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b59020(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eb_0030;
        (this as *mut u32).write_unaligned(VTABLE);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        this
    }
});
