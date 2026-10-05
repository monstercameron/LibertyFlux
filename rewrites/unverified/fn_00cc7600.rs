// original: 0x00CC7600 euphoria_handler_ctor (proposed)

/// Constructor: run the base constructor, store the argument, plant the vtable.
///
/// Calls the base constructor on `this`, stores the caller's word at
/// `+0x50`, writes the vtable at `+0` and clears the tag at `+0xc`. Returns
/// `this`.
///
/// Original: 0x00CC7600 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cc7600(this: u32, arg: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const VTABLE: u32 = 0x00ED978C;
        const ARG_AT: u32 = 0x50;
        const TAG_AT: u32 = 0x0c;
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this);
        (this.wrapping_add(ARG_AT) as *mut u32).write_unaligned(arg);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(TAG_AT) as *mut u32).write_unaligned(0);
        this
    }
});
