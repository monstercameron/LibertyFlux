// original: 0x0059dfa0 reinit_subsystems
// Re-initialize an object's subsystems: three one-shot calls, then a
// 3-slot sweep and an 11-slot sweep over consecutive object slots, then a
// tail call on the object itself whose result is returned.
//
// The two sweeps walk downward (each step subtracts 4 before calling).
export!(thiscall, rw_0059DFA0(this: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    callee_thiscall!(2, u32, this.wrapping_add(0x3D0));
    callee_thiscall!(3, u32, this.wrapping_add(0x60));
    let mut slot = this.wrapping_add(0x44);
    let mut i = 0;
    while i < 3 {
        slot = slot.wrapping_sub(4);
        callee_thiscall!(4, u32, slot);
        i += 1;
    }
    slot = this.wrapping_add(0x34);
    let mut j = 0;
    while j < 11 {
        slot = slot.wrapping_sub(4);
        callee_thiscall!(4, u32, slot);
        j += 1;
    }
    // Tail call in the original (jmp); a normal call here: same target,
    // same argument, same returned value, empty cleanup either way.
    callee_thiscall!(4, u32, this)
});
