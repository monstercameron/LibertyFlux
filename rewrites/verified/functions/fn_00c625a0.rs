// original: 0x00c625a0 anim_hooked_scale_product
/// Hooked scale product: the virtual hook's result times the bound duration.
///
/// Loads the duration at `+0xC` of the animation bound at `+0x40` (a null read
/// that faults unless the tag at `+0x44` is 1), calls the hook in the second
/// vtable slot with this player, and returns the hook's result times the
/// duration.
export!(thiscall, rw_00c625a0(this: u32) -> f32 {
    unsafe {
        let tag = *((this + 0x44) as *const u16);
        let span = if tag == 1 {
            let anim = *((this + 0x40) as *const u32);
            f32::from_bits(*((anim + 0xC) as *const u32))
        } else {
            core::ptr::read_volatile(0xC as *const f32)
        };
        let vtable = *((this + 0) as *const u32);
        let slot = *((vtable + 8) as *const u32);
        let hook: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute::<u32, extern "thiscall" fn(u32) -> f32>(slot);
        let got = hook(this);
        // mulss with the original's operand order; NaN selects explicitly (see vf2).
        if got.is_nan() {
            f32::from_bits(got.to_bits() | 0x0040_0000)
        } else if span.is_nan() {
            f32::from_bits(span.to_bits() | 0x0040_0000)
        } else {
            got * span
        }
    }
});
