// original: 0x00a13600 scaled_max_select (proposed)
/// Return the larger of a possibly scaled value and a floor.
///
/// When the byte at `this + 0x22c` is 1 and `sel` is 0 or 1, `v` is first
/// multiplied by the constant 0x3f266666. The result is the larger of that
/// value and `floor`, compared with SSE `comiss` semantics (an unordered
/// NaN comparison keeps the first value). Returned on the x87 stack.
/// Thiscall, three stack arguments.
export!(thiscall, rw_00a13600(this: u32, v: u32, sel: u32, floor: u32) -> f32 {
    unsafe {
        const MODE_OFF: u32 = 0x22c;
        const SCALE_ADDR: u32 = 0x00fe8864;
        let mut a = f32::from_bits(v);
        if ((this + MODE_OFF) as *const u8).read() == 1 && (sel == 0 || sel == 1) {
            let k = f32::from_bits(*global::<u32>(SCALE_ADDR));
            a = core::hint::black_box(a) * core::hint::black_box(k);
        }
        let b = f32::from_bits(floor);
        if b > a {
            b
        } else {
            a
        }
    }
});
