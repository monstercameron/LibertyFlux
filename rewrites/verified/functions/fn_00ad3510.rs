// original: 0x00ad3510 mgr_forward_float_1f8
/// Forward one float plus a variant word to a shared manager method.
///
/// The float's bits are passed through unchanged alongside the variant word
/// from its global; the shared callee is intercepted by the checker and its
/// answer returned.
export!(cdecl, rw_00ad3510(arg: f32) -> u32 {
    unsafe {
        let this = *global::<u32>(0x0154E190);
        let variant = *global::<u32>(0x0154E1F8);
        callee_thiscall!(1, u32, this, variant, arg.to_bits())
    }
});
