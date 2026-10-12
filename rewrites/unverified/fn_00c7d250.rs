// original: 0x00C7D250 CTaskComplexWanderingScenario::vf20
/// Flag the inner object of `arg` and return the child task. `arg` points to an object whose word at +0x224 points to a second object; that object's byte at +0x2dc is set to 1. The function returns the word at `this` + 8. It makes no calls and reads no globals. Calling convention: thiscall with one stack argument; returns a 32-bit word.
lf_checker_rt::export!(thiscall, rw_00c7d250(this: u32, arg: u32) -> u32 {
    const INNER_OFFSET: u32 = 0x224;
    const FLAG_OFFSET: u32 = 0x2DC;
    const CHILD_OFFSET: u32 = 0x08;
    unsafe {
        let inner = ((arg.wrapping_add(INNER_OFFSET)) as *const u32).read_unaligned();
        ((inner.wrapping_add(FLAG_OFFSET)) as *mut u8).write(1);
        ((this.wrapping_add(CHILD_OFFSET)) as *const u32).read_unaligned()
    }
});
