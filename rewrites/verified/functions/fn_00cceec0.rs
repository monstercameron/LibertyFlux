// original: 0x00cceec0 task_anim_set_simple
/// Drive the task's animation slot with a float parameter, then detach it.
///
/// If the slot at `this+0x14` is non-null, the float argument's bits are
/// passed through to the animation setter (thiscall on the slot), the slot
/// is then handed to the detach helper (thiscall, with `this` as argument),
/// and the slot is cleared. Thiscall with one stack argument.
export!(thiscall, rw_00cceec0(this: u32, arg: u32) -> u32 {
    unsafe {
        const ANIM_OFF: u32 = 0x14;
        let anim = (this.wrapping_add(ANIM_OFF) as *const u32).read_unaligned();
        if anim == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(1, u32, anim, arg);
        let anim2 = (this.wrapping_add(ANIM_OFF) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(2, u32, anim2, this);
        (this.wrapping_add(ANIM_OFF) as *mut u32).write_unaligned(0);
        0
    }
});
