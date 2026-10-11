// original: 0x00a29e40 CPlayerPed::vf62

/// Add the stack-supplied single-precision amount to the ped's field at `+0xe9c`,
/// store the rounded result, then forward that exact bit pattern to the virtual
/// helper. The method uses ECX for `this` and callee-cleans its one stack word.
lf_checker_rt::export!(thiscall, rw_a29e40(this: u32, amount: f32) -> u32 {
    const MOTION_VALUE: u32 = 0x0e9c;
    let field = unsafe { (this + MOTION_VALUE) as *mut f32 };
    let current = unsafe { field.read_unaligned() };
    let updated = core::hint::black_box(amount) + core::hint::black_box(current);
    unsafe { field.write_unaligned(updated) };
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, this, updated.to_bits()) }
});
