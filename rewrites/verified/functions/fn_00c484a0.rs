// original: 0x00c484a0 ccamcinematic_ctor (proposed)
/// Construct a cinematic camera: base object, vtable, then setup.
///
/// Calls the base constructor (callee 1) with `this`, plants the
/// `CCamCinematic` vtable, then calls the table-initialisation routine
/// (callee 2). Returns `this`.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c484a0(this: u32) -> u32 {
    const VTABLE_CCAMCINEMATIC: u32 = 0x00ec95a0;
    const BASE_CTOR: u32 = 1;
    const INIT_TABLES: u32 = 2;
    unsafe {
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_CCAMCINEMATIC));
        lf_checker_rt::callee_thiscall!(INIT_TABLES, u32, this);
    }
    this
});
