// original: 0x00cd0d00 CTaskComplexMelee::vf19
/// Initialise the melee task's timing fields and register it.
///
/// Copies two global words into `this+0xdc` and `this+0xe0`, sets the ready
/// byte at `this+0xe4`, then calls the target setup (thiscall, the stack
/// argument) and the registration helper (thiscall with the argument, the
/// constant 0x1b1 and 1). Thiscall with one stack argument.
export!(thiscall, rw_00cd0d00(this: u32, arg: u32) -> u32 {
    unsafe {
        const FIRST_G: u32 = 0x0105194c;
        const SECOND_G: u32 = 0x011735b4;
        const SECOND_OFF: u32 = 0xdc;
        const FIRST_OFF: u32 = 0xe0;
        const READY_OFF: u32 = 0xe4;
        const KIND: u32 = 0x1b1;
        let first = *global::<u32>(FIRST_G);
        let second = *global::<u32>(SECOND_G);
        (this.wrapping_add(SECOND_OFF) as *mut u32).write_unaligned(second);
        (this.wrapping_add(FIRST_OFF) as *mut u32).write_unaligned(first);
        (this.wrapping_add(READY_OFF) as *mut u8).write(1);
        let _: u32 = callee_thiscall!(1, u32, this, arg);
        let _: u32 = callee_thiscall!(2, u32, this, arg, KIND, 1);
        0
    }
});
