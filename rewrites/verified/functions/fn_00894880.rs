// original: 0x00894880 resolve_and_create_flagged
/// Resolve the first argument, then build an entry, choosing the resolver.
///
/// When the low byte of the second argument is zero the resolving helper is
/// the alternate one; otherwise it is the same helper as the unflagged
/// variant. The answer becomes the key passed to the five-argument
/// constructor with a zero marker and the last three arguments.
/// Returns the constructor's result.
lf_k2_rt::export!(thiscall, rs17_00894880(
    this: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
) -> u32 {
    let key = if (a2 & 0xFF) == 0 {
        lf_k2_rt::callee_thiscall!(3, u32, this, a1)
    } else {
        lf_k2_rt::callee_thiscall!(1, u32, this, a1)
    };
    lf_k2_rt::callee_thiscall!(2, u32, this, key, 0, a3, a4, a5)
});
