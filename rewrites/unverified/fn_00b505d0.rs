// original: 0x00b505d0 event_ctor (proposed)

/// Construct an event object in place.
///
/// Runs the base initialiser (callee 1, thiscall on `this` with a zero
/// argument), plants the event vtable pointer, and returns `this`. The
/// vtable address is relocated by the loader, so it is derived through
/// `relocated`, never hard-coded.
///
/// Original: 0x00b505d0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b505d0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eaeed4c;
        const BASE_CTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, 0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
