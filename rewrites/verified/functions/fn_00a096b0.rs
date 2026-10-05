// original: 0x00a096b0 mission_cleanup_ctor (proposed)
/// Construct a mission-cleanup manager: plant its virtual table and run the
/// base constructor on `this`. Returns `this`. Thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00a096b0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00e99fe0;
        const BASE_CTOR: u32 = 0;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        this
    }
});
