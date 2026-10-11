// original: 0x00AF64B0 scale_object_value_by_flag

/// Scale the object's float value according to bit 20 in its flags. The
/// value is read at byte offset 0x90, the flag word at 0x20, and the two
/// scale factors come from relocated read-only game data. The result is a
/// bit-exact f32 returned on the x87 stack.
lf_checker_rt::export!(thiscall, rw_00AF64B0(object: u32) -> f32 {
    const FLAGS_OFFSET: u32 = 0x20;
    const VALUE_OFFSET: u32 = 0x90;
    const SCALE_ORDINARY_VA: u32 = 0x0103_FC80;
    const SCALE_SPECIAL_VA: u32 = 0x0103_FC84;
    const SPECIAL_FLAG: u32 = 1 << 20;

    unsafe {
        let flags = (object.wrapping_add(FLAGS_OFFSET) as *const u32).read_volatile();
        let value_bits = (object.wrapping_add(VALUE_OFFSET) as *const u32).read_volatile();
        let scale_va = if flags & SPECIAL_FLAG != 0 {
            SCALE_SPECIAL_VA
        } else {
            SCALE_ORDINARY_VA
        };
        let scale_bits = lf_checker_rt::global::<u32>(scale_va).read_volatile();
        let value = core::hint::black_box(f32::from_bits(value_bits));
        let scale = core::hint::black_box(f32::from_bits(scale_bits));
        value * scale
    }
});
