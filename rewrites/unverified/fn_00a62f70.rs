// original: 0x00A62F70 forward_planar_task_measurement

/// Obtain the task-dependent `f32` measurement through the first helper's
/// x87 return, then pass that exact bit pattern as both stack arguments to
/// the second helper. Return the second helper's EAX result. `this` is in
/// ECX and there are no stack arguments. Both calls are intercepted and
/// scripted by the contract.
lf_checker_rt::export!(thiscall, rw_00a62f70(this: u32) -> u32 {
    let measurement = lf_checker_rt::callee_thiscall!(1, f32, this);
    let bits = measurement.to_bits();
    lf_checker_rt::callee_cdecl!(2, u32, bits, bits)
});
