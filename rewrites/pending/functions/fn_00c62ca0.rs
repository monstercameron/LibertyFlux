// original: 0x00c62ca0 SetAnimClipTime
/// Clip-time setter: clamps the argument into [0, limit] and stores it twice.
///
/// Stores the clamped time into the fields at `+0x4C` and `+0x50`. NaN passes
/// through unclamped. Returns nothing meaningful (the entry EAX passes through
/// untouched), so the return channel is not compared.
export!(thiscall, rw_00c62ca0(this: u32, arg: u32) -> u32 {
    unsafe {
        let a = f32::from_bits(arg);
        let limit = f32::from_bits(*global::<u32>(0xFE88E8));
        let v = if 0.0 > a {
            0.0f32
        } else if a > limit {
            limit
        } else {
            a
        };
        *((this + 0x4C) as *mut u32) = v.to_bits();
        *((this + 0x50) as *mut u32) = v.to_bits();
        0
    }
});
