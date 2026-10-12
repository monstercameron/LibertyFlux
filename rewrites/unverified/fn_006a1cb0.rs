// original: 0x006A1CB0 gtaDrawable::vf13

/// Forwards six incoming stack words to the draw-data builder. The object
/// field at `this + 0x08` is its first stack argument, the builder's object
/// pointer is `this + 0x10`, and the remaining arguments follow the original
/// push order: `a0`, `a1`, `a2`, `a5`, `a3`, `a4`. The builder's 32-bit return
/// value is returned unchanged. The wrapper uses thiscall and cleans its six
/// incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_006a1cb0(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
) -> u32 {
    const BUILDER_ID: u32 = 1;
    const BUILDER_THIS_OFFSET: u32 = 0x10;
    const BUILDER_INPUT_OFFSET: u32 = 0x08;

    let builder_this = this.wrapping_add(BUILDER_THIS_OFFSET);
    let leading_input = unsafe {
        (this.wrapping_add(BUILDER_INPUT_OFFSET) as *const u32).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(
        BUILDER_ID,
        u32,
        builder_this,
        leading_input,
        a0,
        a1,
        a2,
        a5,
        a3,
        a4,
    )
});
