// original: 0x0087c220 rage::crmtNodeBlend::vf4
/// Forward a blend sample, refined through the child when one is attached.
///
/// Starts from the float at `+0x20`; when the child at `+0x2c` is non-null,
/// refines it through slot 0x0c of the child's table (sample passed as a
/// stack word, fresh float returned on the x87 stack, read here through the
/// stub's eax bit-mirror), then always forwards the sample and the word at
/// `+0x28` to slot 0x2c of the argument's table. Computes no return value.
/// The only comparison is a null check.
///
/// Original: thiscall/1, two indirect calls, x87 float answer channel.
export!(thiscall, rw_0087c220(this: u32, obj: u32) -> u32 {
    /// Sample float, read then possibly refined.
    const SAMPLE_OFF: u32 = 0x20;
    /// Child link, null-checked before the refine call.
    const CHILD_OFF: u32 = 0x2C;
    /// Refine slot in the child's table.
    const REFINE_SLOT: u32 = 0x0C;
    /// Forward slot in the argument's table.
    const FORWARD_SLOT: u32 = 0x2C;
    /// Extra word forwarded alongside the sample.
    const EXTRA_OFF: u32 = 0x28;
    unsafe {
        let mut x = ((this + SAMPLE_OFF) as *const u32).read_unaligned();
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if child != 0 {
            let vc = (child as *const u32).read_unaligned();
            let tc = ((vc + REFINE_SLOT) as *const u32).read_unaligned();
            // The stub answers f32st0 and mirrors the bits in eax, which is
            // how the rewrite reads the x87 answer without assembly.
            let fc: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tc as usize);
            x = fc(child, x);
        }
        let extra = ((this + EXTRA_OFF) as *const u32).read_unaligned();
        let vo = (obj as *const u32).read_unaligned();
        let to = ((vo + FORWARD_SLOT) as *const u32).read_unaligned();
        let fo: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(to as usize);
        fo(obj, x, extra);
        0
    }
});
