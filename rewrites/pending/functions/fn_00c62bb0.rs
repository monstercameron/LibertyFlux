// original: 0x00c62bb0 anim_set_blend_limit
/// Blend-limit setter: stores the argument into the limit field at `+0x58`.
///
/// Returns nothing meaningful (the entry EAX passes through untouched), so the
/// return channel is not compared.
export!(thiscall, rw_00c62bb0(this: u32, limit: u32) -> u32 {
    unsafe {
        *((this + 0x58) as *mut u32) = limit;
        0
    }
});
