// original: 0x00D254D0 CCustomShaderEffectVehicleFX::vf6
/// Clears the local draw-command field and, when a command object is supplied, replaces the
/// incoming pointer argument with 1 before dispatching through the object's first virtual slot as
/// a tail call. The method is thiscall with one stack argument; the stack comparison is narrowed
/// because safe Rust does not expose the original incoming argument slot, while the outgoing call
/// log observes the value passed to the virtual method.
lf_checker_rt::export!(thiscall, rw_00d254d0(this: u32, draw_command: u32) -> () {
    const DRAW_COMMAND_SLOT: u32 = 0xdc;
    const CALL_ARGUMENT: u32 = 1;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        write_u32(this, DRAW_COMMAND_SLOT, 0);
        if draw_command != 0 {
            let vtable = read_u32(draw_command, 0);
            let target = read_u32(vtable, 0);
            let method: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
            let _ = method(draw_command, CALL_ARGUMENT);
        }
    }
});
