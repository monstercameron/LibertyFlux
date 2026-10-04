// original: 0x00c62c70 anim_set_current
/// Current-value setter: stores the argument into the field at `+0x2C`.
///
/// Returns nothing meaningful (the entry EAX passes through untouched), so the
/// return channel is not compared.
export!(thiscall, rw_00c62c70(this: u32, value: u32) -> u32 {
    unsafe {
        *((this + 0x2C) as *mut u32) = value;
        0
    }
});
