// original: 0x00CEAAC0 CBike::~CBike

/// Restore the bike vtable, destroy its owned subobjects, and tail-delegate.
///
/// The destructor writes the relocated bike vtable at offset zero, calls the
/// first subobject destructor at `0x1730`, destroys two elements at offsets
/// `0x15C0` and `0x1450` with the same cleanup helper, calls the subobject
/// destructor at `0x13D0`, then forwards `this` to the terminal base cleanup.
/// The two-element loop is fixed by the original's initial counter of one.
lf_checker_rt::export!(thiscall, rw_fn_00ceaac0(this: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00ed_d95c;
    const SUBOBJECT_DESTRUCTOR: u32 = 1;
    const ELEMENT_CLEANUP: u32 = 2;
    const TAIL_CLEANUP: u32 = 3;
    const FIRST_SUBOBJECT: u32 = 0x1730;
    const ELEMENT_STRIDE: u32 = 0x170;
    const ELEMENT_START: u32 = 0x1730;
    const SECOND_SUBOBJECT: u32 = 0x13d0;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
    }
    let _ = lf_checker_rt::callee_thiscall!(SUBOBJECT_DESTRUCTOR, u32, this.wrapping_add(FIRST_SUBOBJECT));
    for element_offset in [ELEMENT_START - ELEMENT_STRIDE, ELEMENT_START - 2 * ELEMENT_STRIDE] {
        let element = this.wrapping_add(element_offset);
        let _ = lf_checker_rt::callee_thiscall!(ELEMENT_CLEANUP, u32, element);
    }
    let _ = lf_checker_rt::callee_thiscall!(SUBOBJECT_DESTRUCTOR, u32, this.wrapping_add(SECOND_SUBOBJECT));
    lf_checker_rt::callee_thiscall!(TAIL_CLEANUP, u32, this)
});
