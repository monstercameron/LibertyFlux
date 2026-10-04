// original: 0x00dac850 flee_link_task_ctor
/// Construct a task holding one linked argument.
///
/// Runs the base constructor, installs the vtable, stores the argument and
/// zeroes the remaining fields, bumps the live-task counter, and links the
/// stored argument through the registrar when it is non-null. Returns the
/// task. The vtable word is an image address the loader relocates, so the
/// rewrite derives it with `relocated` like every other address.
export!(thiscall, rw_00dac850(this: u32, arg: u32) -> u32 {
    unsafe {
        /// Vtable installed by this constructor (file VA).
        const VTABLE: u32 = 0x00EF04AC;
        /// Live-task counter.
        const COUNT: u32 = 0x017A6538;
        /// Stored argument's slot.
        const ARG_SLOT: u32 = 0x14;
        let _: u32 = callee_thiscall!(1, u32, this);
        ((this) as *mut u32).write_unaligned(relocated(VTABLE));
        ((this.wrapping_add(ARG_SLOT)) as *mut u32).write_unaligned(arg);
        ((this.wrapping_add(0x18)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x1c)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x20)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x24)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x28)) as *mut u32).write_unaligned(0);
        (*((this.wrapping_add(0x30)) as *mut u8) = (0));
        let c = global::<u32>(COUNT);
        c.write(c.read().wrapping_add(1));
        if arg != 0 {
            let _: u32 = callee_thiscall!(2, u32, arg, this.wrapping_add(ARG_SLOT));
        }
        this
    }
});
