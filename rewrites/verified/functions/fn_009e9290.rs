// original: 0x009e9290 CPed::vf60
/// Publishes a level value: forwards its bits to the shared level
/// sink, then records three threshold flags (50.0 and 100.0 from the
/// shared constants, plus non-positive) at `+0x212/+0x211/+0x210` and
/// the value itself at `+0x214`. (thiscall, 1 float arg.)
lf_checker_rt::export!(thiscall, rw_009e9290(this_ptr: u32, level_arg: u32) -> u32 {
    unsafe {
        const LO_TH: u32 = 0xFE8B68;
        const HI_TH: u32 = 0xFE8BB0;
        const FLAG2_OFF: u32 = 0x212;
        const FLAG1_OFF: u32 = 0x211;
        const FLAG0_OFF: u32 = 0x210;
        const VALUE_OFF: u32 = 0x214;
        let ans: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr, level_arg);
        let x = f32::from_bits(level_arg);
        let c1 = f32::from_bits(lf_checker_rt::global::<u32>(LO_TH).read_unaligned());
        let c2 = f32::from_bits(lf_checker_rt::global::<u32>(HI_TH).read_unaligned());
        (this_ptr.wrapping_add(FLAG2_OFF) as *mut u8).write(if c1 > x { 1 } else { 0 });
        (this_ptr.wrapping_add(FLAG1_OFF) as *mut u8).write(if c2 > x { 1 } else { 0 });
        let b0 = if x <= 0.0 { 1u32 } else { 0u32 };
        (this_ptr.wrapping_add(FLAG0_OFF) as *mut u8).write(b0 as u8);
        (this_ptr.wrapping_add(VALUE_OFF) as *mut u32).write_unaligned(level_arg);
        (ans & 0xFFFFFF00) | b0
    }
});
