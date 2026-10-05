// original: 0x00a71c70 forward_float_with_tag (proposed)
/// Forwards the float argument and a tag to a five-word handler, returning
/// its answer.
///
/// `thiscall`: context in ECX, one stack word holding float bits, callee
/// pops 4. The handler takes the context as its object and (0x2EF3,
/// float_bits, 0, 0, 1) on the stack; the original's `(an instruction of the original)` only
/// reserves the float's slot (overwritten before the call), it is not an
/// argument. The float moves through a vector register in the original but
/// is a bit-exact transport, reproduced by passing the bits.
lf_checker_rt::export!(thiscall, rw_00a71c70(ctx: u32, fbits: u32) -> u32 {
    const TAG: u32 = 0x2ef3;
    lf_checker_rt::callee_thiscall!(1, u32, ctx, TAG, fbits, 0, 0, 1)
});
