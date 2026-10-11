// original: 0x00E6E800 mainloop_timing_update_enabled_state

/// If the active bit is set, queries the timing object's third vtable slot,
/// writes its low-byte truth value to the shared state byte, calls the timing
/// helper twice when true, and clears the active bit. The return register is
/// unspecified by this void routine.
pub extern "cdecl" fn rw_00e6e800() {
    const ACTIVE_FLAGS: u32 = 0x018e_0104;
    const OBJECT_POINTER: u32 = 0x019f_2654;
    const ENABLED_STATE: u32 = 0x01bb_6818;

    let flags = unsafe { lf_checker_rt::global::<u8>(ACTIVE_FLAGS).read() };
    if flags & 1 == 0 {
        return;
    }

    let object = unsafe { lf_checker_rt::global::<u32>(OBJECT_POINTER).read() };
    let enabled = if object == 0 {
        false
    } else {
        let vtable = unsafe { (object as *const u32).read_unaligned() };
        let method = unsafe { ((vtable as *const u32).add(2)).read_unaligned() };
        let query: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(method as usize)
        };
        query(object) as u8 != 0
    };

    unsafe {
        lf_checker_rt::global::<u8>(ENABLED_STATE).write(u8::from(enabled));
        lf_checker_rt::global::<u8>(ACTIVE_FLAGS).write(flags & 0xfe);
    }
    if enabled {
        let helper: extern "cdecl" fn() -> u32 = unsafe {
            core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
        };
        let _ = helper();
        let _ = helper();
    }
}
