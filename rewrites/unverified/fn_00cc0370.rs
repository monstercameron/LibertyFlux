// original: 0x00CC0370 CPedMoveBlendOnFoot::vf9
/// Return one in AL when either leading f32 component is nonzero or NaN, the
/// related task's virtual method accepts it, or the final f32 component is
/// not one. The related value comes from argument offset 0x78; the virtual
/// method is represented by a scripted thiscall stand-in.
lf_checker_rt::export!(thiscall, rw_00cc0370(this: *mut u8, related: *const u8) -> u32 {
    const COMPONENT_A_OFFSET: usize = 4;
    const COMPONENT_B_OFFSET: usize = 8;
    const FINAL_COMPONENT_OFFSET: usize = 0x54;
    const RELATED_VALUE_OFFSET: usize = 0x78;
    const ZERO_COMPONENT: f32 = 0.0;
    const RESTING_COMPONENT: f32 = 1.0;
    let first = unsafe { f32::from_bits(this.add(COMPONENT_A_OFFSET).cast::<u32>().read_unaligned()) };
    let second = unsafe { f32::from_bits(this.add(COMPONENT_B_OFFSET).cast::<u32>().read_unaligned()) };
    if first != ZERO_COMPONENT || first.is_nan() || second != ZERO_COMPONENT || second.is_nan() {
        return 1;
    }
    let related_value = unsafe { related.add(RELATED_VALUE_OFFSET).cast::<u32>().read_unaligned() };
    if lf_checker_rt::callee_thiscall!(1, u32, this as u32, related_value) & 0xFF != 0 {
        return 1;
    }
    let final_component = unsafe { f32::from_bits(this.add(FINAL_COMPONENT_OFFSET).cast::<u32>().read_unaligned()) };
    u32::from(final_component != RESTING_COMPONENT || final_component.is_nan())
});
