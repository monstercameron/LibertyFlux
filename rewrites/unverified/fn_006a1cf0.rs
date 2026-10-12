// original: 0x006A1CF0 rage::fragDrawable::~fragDrawable__deleting_2

/// Forwards five incoming stack arguments and the original object pointer
/// to the fragment drawable destructor helper. The helper receives `this + 0x10`
/// in ECX and the stack words in order: `a0`, `a1`, `a2`, `a3`, `a4`, original
/// `this`. Its 32-bit return is forwarded unchanged. The wrapper uses thiscall and
/// cleans its five incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_006a1cf0(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
) -> u32 {
    const HELPER_ID: u32 = 1;
    const SUBOBJECT_OFFSET: u32 = 0x10;
    let helper_this = this.wrapping_add(SUBOBJECT_OFFSET);
    lf_checker_rt::callee_thiscall!(HELPER_ID, u32, helper_this, a0, a1, a2, a3, a4, this)
});
