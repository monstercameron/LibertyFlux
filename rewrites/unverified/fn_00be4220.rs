// original: 0x00be4220 CTaskComplexUseEffect::vf18 (symbols)

/// Look up the effect handle and run its check when the lookup agrees.
///
/// `this + HANDLE_OFF` (0x1c) must hold a nonzero handle or this returns zero
/// at once. Otherwise the lookup callee (cdecl: it pops nothing itself)
/// runs over the incoming argument and the two parameter words at
/// `this + PAR0` (0x14) and `+ PAR1` (0x18); its answer becomes the object
/// for the resolve callee, which consumes the lookup's three leftover stack
/// words as its own arguments (thiscall, three words) and whose answer must
/// equal the handle. On agreement the check callee runs with the handle as
/// object and the incoming argument, and its answer is returned; on
/// disagreement this returns zero.
///
/// The rewrite passes the three words to both callees explicitly; the
/// original lets the second callee inherit them from the stack. The two call
/// logs match word for word.
///
/// Original: 0x00be4220 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be4220(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x1c;
        const PAR0: u32 = 0x14;
        const PAR1: u32 = 0x18;
        const LOOKUP: u32 = 1;
        const RESOLVE: u32 = 2;
        const CHECK: u32 = 3;
        let handle = (this.wrapping_add(HANDLE_OFF) as *const u32).read_unaligned();
        if handle == 0 {
            return 0;
        }
        let par0 = (this.wrapping_add(PAR0) as *const u32).read_unaligned();
        let par1 = (this.wrapping_add(PAR1) as *const u32).read_unaligned();
        let found = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg, par0, par1);
        let resolved = lf_checker_rt::callee_thiscall!(RESOLVE, u32, found, arg, par0, par1);
        if resolved != handle {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CHECK, u32, handle, arg)
    }
});
