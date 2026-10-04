// original: 0x00E6FE90 timer_poll_gate
/// Poll the timer provider once when the poll gate is open.
///
/// When the gate flag is set, asks the installed provider whether a tick is
/// pending, records the answer in the state byte, fires the two tick hooks
/// on a hit, and closes the gate.
export!(cdecl, rw_00e6fe90() -> u32 {
    unsafe {
        const GATE: u32 = 0x018E0106;
        const OBJ_SLOT: u32 = 0x019F2654;
        const STATE: u32 = 0x01BB6818;
        const POLL_OFF: u32 = 8;
        if *global::<u8>(GATE) & 1 == 0 {
            return 0;
        }
        let obj = *global::<u32>(OBJ_SLOT);
        let mut hit = false;
        if obj != 0 {
            let vtable = *(obj as *const u32);
            let poll: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((vtable + POLL_OFF) as *const u32) as usize);
            if poll(obj, obj) & 0xFF != 0 {
                hit = true;
            }
        }
        *global::<u8>(STATE) = hit as u8;
        if hit {
            callee_cdecl!(2, u32,);
            callee_cdecl!(2, u32,);
        }
        *global::<u8>(GATE) &= 0xFE;
        0
    }
});
