// original: 0x00dac8b0 flee_full_task_ctor
/// Construct a task from an argument block and a source record.
///
/// Runs the base constructor, installs the vtable, copies the four source
/// words and the scalar arguments into their fields, registers the stored
/// first argument unconditionally, and stamps the trailing slot with -1.0.
/// Returns the task. The vtable word is a relocated image address like the
/// sibling constructor's.
export!(thiscall, rw_00dac8b0(
    this: u32,
    arg0: u32,
    src: u32,
    arg2: u32,
    arg3: u32,
    arg4: f32,
    arg5: f32,
    arg6: u32,
) -> u32 {
    unsafe {
        /// Vtable installed by this constructor (file VA).
        const VTABLE: u32 = 0x00EF0454;
        /// Stored first argument's slot (also registered).
        const ARG_SLOT: u32 = 0x14;
        let _: u32 = callee_thiscall!(1, u32, this);
        ((this) as *mut u32).write_unaligned(relocated(VTABLE));
        ((this.wrapping_add(0x44)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x48)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x4c)) as *mut u16).write_unaligned(0);
        ((this.wrapping_add(ARG_SLOT)) as *mut u32).write_unaligned(arg0);
        ((this.wrapping_add(0x20)) as *mut u32).write_unaligned(((src) as *const u32).read_unaligned());
        ((this.wrapping_add(0x24)) as *mut u32).write_unaligned((f32::from_bits(((src.wrapping_add(4)) as *const u32).read_unaligned())).to_bits());
        ((this.wrapping_add(0x28)) as *mut u32).write_unaligned((f32::from_bits(((src.wrapping_add(8)) as *const u32).read_unaligned())).to_bits());
        ((this.wrapping_add(0x2c)) as *mut u32).write_unaligned(((src.wrapping_add(0x0c)) as *const u32).read_unaligned());
        (*((this.wrapping_add(0x30)) as *mut u8) = (arg2 as u8));
        ((this.wrapping_add(0x34)) as *mut u32).write_unaligned(arg3);
        ((this.wrapping_add(0x38)) as *mut u32).write_unaligned((arg4).to_bits());
        ((this.wrapping_add(0x3c)) as *mut u32).write_unaligned((arg5).to_bits());
        (*((this.wrapping_add(0x40)) as *mut u8) = (arg6 as u8));
        let _: u32 = callee_thiscall!(2, u32, arg0, this.wrapping_add(ARG_SLOT));
        ((this.wrapping_add(0x50)) as *mut u32).write_unaligned((-1.0f32).to_bits());
        this
    }
});
