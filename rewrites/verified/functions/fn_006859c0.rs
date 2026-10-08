// original: 0x006859C0 forward_to_00687000

/// Forwards two inputs and the owner through helper 1. The helper ECX points
/// to a local two-word context containing the owner and first input. The
/// contract omits the frame address with an empty call-register set and
/// snapshots both 32-bit pointee words at offsets 0 and 4. Stack arguments
/// remain compared in order: first input, second input, then owner.
///
/// Scope: one straight-line call; helper 1 returns scripted edge values, so
/// the native helper implementation is outside this proof. All strict stock-v8
/// checks run on the 1,000 generated inputs.
lf_checker_rt::export!(thiscall, rw_006859c0(this: u32, first: u32, second: u32) -> u32 {
    let context = [this, first];
    let context_ptr = context.as_ptr() as u32;
    lf_checker_rt::callee_thiscall!(1, u32, context_ptr, first, second, this)
});
