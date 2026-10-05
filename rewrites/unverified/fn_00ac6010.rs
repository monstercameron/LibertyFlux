// original: 0x00AC6010 CCustomShaderEffectPedBoneDamageFX::vf1 (symbols)

/// Look up six parameter handles and store them on the effect.
///
/// The original calls the lookup callee six times with the object at
/// `[source + 8]`, a constant name and 1, storing each answer at
/// `this + 0x08 .. this + 0x1c` in order (thiscall, one stack pointer).
/// It returns 1 in the low byte.
lf_checker_rt::export!(thiscall, rw_00AC6010(this: u32, source: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const NAMES: [u32; 6] = [0x00EA5D48, 0x00EA5D60, 0x00EA5D78, 0x00EA5D84, 0x00EA5D94, 0x00EA5DA0];
        const FIRST: u32 = 0x08;
        let obj = (source.wrapping_add(8) as *const u32).read_unaligned();
        for i in 0..6u32 {
            let h = lf_checker_rt::callee_thiscall!(LOOKUP, u32, obj, NAMES[i as usize], 1u32);
            (this.wrapping_add(FIRST + i * 4) as *mut u32).write_unaligned(h);
        }
        1
    }
});
