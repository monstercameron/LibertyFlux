// original: 0x00AC5F10 CCustomShaderEffectPedBoneDamageFX::vf0 (symbols)

/// Destroy the bone-damage effect, freeing it when the flag bit is set.
///
/// The original calls the destructor callee on `this`, then frees `this`
/// through the free callee only when bit 0 of the flag word is set
/// (thiscall, one stack word). It returns `this`.
lf_checker_rt::export!(thiscall, rw_00AC5F10(this: u32, flag: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const FREE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flag & 1 != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, this);
        }
        this
    }
});
