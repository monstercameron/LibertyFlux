// original: 0x00988CC0 audEntity_ctor (proposed)

/// Audio entity constructor: chains the base constructor, installs the
/// vtable and zeroes the first fields.
///
/// Runs the base constructor (callee id 1) on `this`, writes the vtable
/// pointer `VTABLE` at `+0x00`, zeroes the words at `+0x08`, `+0x0c` and
/// `+0x10`, and returns `this`.
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00988CC0(this: u32) -> u32 {
    const VTABLE: u32 = 0xe8ec90;
    const BASE_CTOR: u32 = 1;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this + 0x00) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x08) as *mut u32).write_unaligned(0);
        ((this + 0x0c) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
    }
    this
});
