// original: 0x0063D560 rage::ptxModel::vf10

/// Forward four arguments to the model's three virtual parameter handlers
/// and return the third handler's result.
///
/// The object points to a vtable at offset `0`; the handlers are selected
/// from slots `0x2c`, `0x30`, and `0x34`. The first three arguments are
/// integer words. The fourth argument is a 32-bit float bit pattern passed
/// both in its stack position and in XMM0, matching the method ABI.
lf_checker_rt::export!(thiscall, rw_0063d560(object: u32, first: u32, second: u32, third: u32, blend_bits: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, object, first, second, third, blend_bits);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, object, first, second, third, blend_bits);
    lf_checker_rt::callee_thiscall!(3, u32, object, first, second, third, blend_bits)
});
