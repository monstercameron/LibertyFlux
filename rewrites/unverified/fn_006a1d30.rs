// original: 0x006A1D30 gtaDrawable::vf10

/// Loads the 32-bit field at `this + 0x08`, calls the drawable helper with
/// `this + 0x10` in ECX, and forwards the field, four stack arguments, then the
/// original `this` pointer. The helper's 32-bit return is returned unchanged. The
/// wrapper uses thiscall and cleans four incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_006a1d30(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
) -> u32 {
    const HELPER_ID: u32 = 1;
    const FIELD_OFFSET: u32 = 0x08;
    const SUBOBJECT_OFFSET: u32 = 0x10;
    let forwarded_field = unsafe {
        (this.wrapping_add(FIELD_OFFSET) as *const u32).read_unaligned()
    };
    let helper_this = this.wrapping_add(SUBOBJECT_OFFSET);
    lf_checker_rt::callee_thiscall!(HELPER_ID, u32, helper_this, forwarded_field, a0, a1, a2, a3, this)
});
