// original: 0x00c6e0c0 anim_clear_and_reset
/// Clear the two state words at `this+8`/`this+12`, then reset the subobject
/// at `this+0x10` through the shared helper. Returns `this`.
export!(thiscall, rw_00c6e0c0(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32).add(2) = 0;
        *(this as *mut u32).add(3) = 0;
    }
    callee_cdecl!(1, u32, this.wrapping_add(0x10));
    this
});
