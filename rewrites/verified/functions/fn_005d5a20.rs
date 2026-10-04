// original: 0x005d5a20 html_text_format_apply_scale
/// Scale the base metric by the factor at `+0x14` and forward the result
/// with two of the incoming arguments to the layout routine.
///
/// (The original also multiplies the value at `+0x10` by the fifth
/// argument and drops the result; that computation has no observable
/// effect and is omitted.) Returns the layout answer with its low byte
/// forced to 1.
export!(thiscall, rw_005d5a20(
    this_ptr: u32,
    _a1: u32,
    _a2: u32,
    a3: u32,
    a4: u32,
    _a5: u32,
) -> u32 {
    let base: f32 = callee_thiscall!(0, f32, this_ptr);
    let factor = unsafe { ((this_ptr + 0x14) as *const f32).read() };
    let scaled = base * factor;
    let ans: u32 = callee_thiscall!(1, u32, this_ptr, a3, a4, scaled.to_bits());
    (ans & 0xFFFF_FF00) | 1
});
