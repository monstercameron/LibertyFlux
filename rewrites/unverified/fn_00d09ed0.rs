// original: 0x00D09ED0 CTaskComplexCombatFireSubtask::vf19

/// Run the three fire-subtask virtual steps in order.
///
/// Calls the virtual steps at slots `+0x5c`, `+0x60` and `+0x64` of the
/// task's table (planted stub callees): the first takes the incoming word,
/// the second takes the incoming word and the first answer, the third takes
/// the second answer and the incoming word. Returns the last answer.
///
/// Original: 0x00D09ED0 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d09ed0(this: u32, a1: u32) -> u32 {
    unsafe {
        let table = ((this) as *const u32).read_unaligned();
        let step1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((table + 0x5c) as *const u32).read_unaligned()) as usize);
        let r1 = step1(this, a1);
        let table = ((this) as *const u32).read_unaligned();
        let step2: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute((((table + 0x60) as *const u32).read_unaligned()) as usize);
        let r2 = step2(this, a1, r1);
        let table = ((this) as *const u32).read_unaligned();
        let step3: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute((((table + 0x64) as *const u32).read_unaligned()) as usize);
        step3(this, r2, a1)
    }
});
