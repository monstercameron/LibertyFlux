// original: 0x00b77e40 lookup_state_is_ready (proposed)

/// Look an object up and report whether its state is ready.
///
/// Resolves `arg` through the lookup callee. A null result means not ready.
/// Otherwise the object's state field decides: 4, 5 or 6 count as ready
/// (the original reaches that test with a shared jump tail; the behaviour
/// is a plain membership test returning exactly 0 or 1).
///
/// Original: cdecl with one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b77e40(arg: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const STATE: u32 = 0x19C;
        let p: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg);
        if p == 0 {
            return 0;
        }
        match ((p + STATE) as *const u32).read_unaligned() {
            4 | 5 | 6 => 1,
            _ => 0,
        }
    }
});
