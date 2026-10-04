// original: 0x00c5a7d0 task_init_and_register (proposed)

/// Initialise a task sub-object and register it, unless vetoed.
///
/// When `flag` is nonzero the function does nothing and returns 0 (the
/// original leaks the caller's entry `eax` in the upper return bytes; the
/// contract pins it to 0). Otherwise, when the state byte at `this+0x20` is
/// set, the sub-object is stamped with the current timer at `+0x18`, -1 at
/// `+0x1c` and 1 at `+0x20`, then callee 1 registers `obj` with the slot at
/// `[obj+0x224]+0x10c` and mode 0x10. Returns the callee answer with its low
/// byte forced to 1.
///
/// Original: 0x00c5a7d0 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00c5a7d0(this: u32, obj: u32, flag: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x0c;
        const STATE: u32 = 0x20;
        const TIMER_SLOT: u32 = 0x11735b4;
        const REGISTER: u32 = 1;
        if flag != 0 {
            return 0;
        }
        if ((this + STATE) as *const u8).read() != 0 {
            let timer = lf_checker_rt::global::<u32>(TIMER_SLOT).read();
            ((this + 0x18) as *mut u32).write_unaligned(timer);
            ((this + 0x1c) as *mut u32).write_unaligned(0xffff_ffff);
            ((this + STATE) as *mut u8).write(1);
        }
        let slot = ((obj + 0x224) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, this + SUB, obj, slot + 0x10c, 0x10);
        (r & 0xffff_ff00) | 1
    }
});

