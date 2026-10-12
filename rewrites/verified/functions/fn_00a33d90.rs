// original: 0x00A33D90 CEntity::set_ui_listener

/// Set or clear the entity's UI-listener bit at `+0x5e`. Any nonzero low byte
/// of the stack argument enables the listener; zero disables it. The helper
/// is called only when the requested state differs from the current bit, and
/// the updated 16-bit flags are stored before the call. Both callbacks receive
/// this entity as one cdecl stack argument. The function uses thiscall and
/// consumes its one stack word.
lf_checker_rt::export!(thiscall, rw_00a33d90(this: u32, enable: u32) -> () {
    const LISTENER_FLAGS: u32 = 0x5e;
    const LISTENER_ENABLED: u16 = 0x0002;
    const REGISTER_LISTENER: u32 = 1;
    const RELEASE_TIMER: u32 = 2;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    unsafe {
        let flags_address = this.wrapping_add(LISTENER_FLAGS);
        let flags = read_u16(flags_address);
        if enable & 0xff != 0 {
            if flags & LISTENER_ENABLED != 0 {
                return;
            }
            write_u16(flags_address, flags | LISTENER_ENABLED);
            let _ = lf_checker_rt::callee_cdecl!(REGISTER_LISTENER, u32, this);
        } else {
            if flags & LISTENER_ENABLED == 0 {
                return;
            }
            write_u16(flags_address, flags & !LISTENER_ENABLED);
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_TIMER, u32, this);
        }
    }
});
