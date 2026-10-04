// original: 0x00c6e0a0 anim_reset_subobject
/// Reset the subobject at `this+0x10` through the shared helper. The stack
/// argument is ignored (but still popped by the callee). Returns `this`.
export!(thiscall, rw_00c6e0a0(this: u32, _ignored: u32) -> u32 {
    callee_cdecl!(1, u32, this.wrapping_add(0x10));
    this
});
