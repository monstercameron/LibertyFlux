// original: 0x00ccee70 task_die_anim_set
/// Drive the task's animation slot with a clamped float, then detach it.
///
/// If the slot at `this+0x1c` is non-null, the float argument is kept when
/// negative or NaN and replaced with the -32.0 constant otherwise (a
/// `comiss` against +0.0 whose jump is taken exactly when the argument is
/// not `>= 0.0`). Bit 0x8000 is set in the slot's flags word at `+4`, the
/// value goes to the animation setter, the detach helper runs only when the
/// slot's state word at `+0x24` is non-zero, and the slot is cleared.
/// Thiscall with one stack argument.
export!(thiscall, rw_00ccee70(this: u32, arg: u32) -> u32 {
    unsafe {
        const ANIM_OFF: u32 = 0x1c;
        const FLAGS_OFF: u32 = 0x04;
        const STATE_OFF: u32 = 0x24;
        const ZERO_G: u32 = 0x00fe8628;
        const CLAMP_G: u32 = 0x00fe8dd8;
        const ACTIVE_BIT: u32 = 0x8000;
        let anim = (this.wrapping_add(ANIM_OFF) as *const u32).read_unaligned();
        if anim == 0 {
            return 0;
        }
        let threshold = f32::from_bits(*global::<u32>(ZERO_G));
        let mut v = f32::from_bits(arg);
        if v >= threshold {
            v = f32::from_bits(*global::<u32>(CLAMP_G));
        }
        let flags = (anim.wrapping_add(FLAGS_OFF) as *const u32).read_unaligned();
        (anim.wrapping_add(FLAGS_OFF) as *mut u32).write_unaligned(flags | ACTIVE_BIT);
        let _: u32 = callee_thiscall!(1, u32, anim, v.to_bits());
        let anim2 = (this.wrapping_add(ANIM_OFF) as *const u32).read_unaligned();
        if (anim2.wrapping_add(STATE_OFF) as *const u32).read_unaligned() != 0 {
            let _: u32 = callee_thiscall!(2, u32, anim2, this);
        }
        (this.wrapping_add(ANIM_OFF) as *mut u32).write_unaligned(0);
        0
    }
});
