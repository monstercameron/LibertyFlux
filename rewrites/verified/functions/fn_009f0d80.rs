// original: 0x009f0d80 ped_ratio_gate
/// Call the wide entry when the float argument beats a scaled tick count.
///
/// Converts the reporter's answer to float, scales it, and compares
/// argument 1 against the product; anything not strictly greater returns
/// the reporter's answer with its low byte cleared (the original's
/// `(an instruction of the original)` shape). Otherwise it calls the ten-argument entry on the
/// object at `0x570` with (arg0, arg2, arg3, 0, -1, 0, 0, 1.0, 0, 0) and
/// returns that answer with its low byte replaced by whether the byte was
/// nonzero (the `setne al` shape).
export!(thiscall, rw_009f0d80(
    this_ptr: u32,
    arg0: u32,
    arg1: f32,
    arg2: u32,
    arg3: u32,
) -> u32 {
    unsafe {
        let ticks: u32 = callee_cdecl!(2, u32,);
        let scaled = (ticks as i32 as f32) * f32::from_bits(*global::<u32>(0xfe8684));
        if !(arg1 > scaled) {
            return ticks & 0xffffff00;
        }
        let sub = this_ptr.wrapping_add(0x570);
        let ans: u32 = callee_thiscall!(
            3, u32, sub,
            arg0, arg2, arg3, 0, 0xffffffff, 0, 0, 1.0f32.to_bits(), 0, 0
        );
        (ans & 0xffffff00) | (((ans & 0xff) != 0) as u32)
    }
});
