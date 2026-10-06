// original: 0x0094CC50 lookup_key_then_validate (proposed)

/// Resolve `arg` to a key through callee 1, then validate it via callee 2.
///
/// Calls callee 1 with the context word `CTX` in ECX and `arg` on the
/// stack; its answer is passed as the stack word to callee 2 (thiscall,
/// `this` in ECX), whose answer is returned.
///
/// Original: 0x0094CC50 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0094CC50(this: u32, arg: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x12E22A4;
        const LOOKUP: u32 = 1;
        const VALIDATE: u32 = 2;
        let ctx = (lf_checker_rt::global::<u32>(CTX) as *const u32).read();
        let key = lf_checker_rt::callee_thiscall!(LOOKUP, u32, ctx, arg);
        lf_checker_rt::callee_thiscall!(VALIDATE, u32, this, key)
    }
});
