// original: 0x00BE6B70 CTaskSimpleSetCharDecisionMaker::vf5
/// Forward an abort-check to the task at virtual slot 0x44.
///
/// Loads the vtable from `[this]`, calls slot `0x44` as a thiscall with
/// `this` in ECX and the first stack word as its argument, and returns that
/// call's high three bytes with low byte forced to 1. Original is thiscall
/// with three stack words (only the first is read) and pops 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6b70(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    const SLOT: u32 = 0x44;
    unsafe {
        let vt = (this as *const u32).read_unaligned();
        let target = ((vt + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
        let r = f(this, a0);
        (r & 0xFFFFFF00) | 1
    }
});
