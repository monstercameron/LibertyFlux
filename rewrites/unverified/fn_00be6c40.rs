// original: 0x00BE6C40 CTaskSimplePathfindProblem::vf5
/// Initialise the abort fields of a pathfind-problem task when `gate` is 0.
///
/// When the second stack word `gate` is zero, copies the global at `SRC`
/// to `this+0x38`, writes -1 to `this+0x3c` and 1 to `this+0x40`. Returns 1
/// in AL; the high bytes of eax keep whatever the global load (taken path)
/// or the caller (skipped path) left there, so only AL is behaviour.
/// Original is thiscall with three stack words (only the second is read)
/// and pops 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6c40(this: u32, _a0: u32, gate: u32, _a2: u32) -> u32 {
    const SRC: u32 = 0x011735B4;
    const F_DST: u32 = 0x38;
    const F_NEG: u32 = 0x3C;
    const F_ONE: u32 = 0x40;
    if gate == 0 {
        unsafe {
            let v = lf_checker_rt::global::<u32>(SRC).read();
            (this.wrapping_add(F_DST) as *mut u32).write_unaligned(v);
            (this.wrapping_add(F_NEG) as *mut u32).write_unaligned(0xFFFFFFFF);
            (this.wrapping_add(F_ONE) as *mut u8).write(1);
        }
    }
    1
});
