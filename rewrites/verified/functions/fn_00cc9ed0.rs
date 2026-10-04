// original: 0x00cc9ed0 ARTFeedbackInterfaceGta::vf4
/// ART feedback slot 4: same shape as slot 2, notifying through slot 0x58.
///
/// See `rw_00cc9e60`: outer handle at +0x190, inner object at +0xC, gate at
/// +0x6C with flag byte at +0xE selecting the gate helper (id 1) or the
/// probe-plus-lookup path (ids 2, 3). A null target returns 0, else the
/// slot-0x58 virtual notification fires with (inner, this) and returns 1.
export!(thiscall, rw_00cc9ed0(this: *mut u8) -> u32 {
    unsafe {
        const OUTER_OFF: usize = 0x190;
        const INNER_OFF: usize = 0x0C;
        const GATE_OFF: usize = 0x6C;
        const FLAG_OFF: usize = 0x0E;
        const GATE_BLOCK: u32 = 0x808;
        const PROBE_OFF: usize = 0x224;
        const NOTIFY_SLOT: usize = 0x58;
        let outer = *(this.add(OUTER_OFF) as *const u32);
        if outer == 0 {
            return 0;
        }
        let inner = *((outer as *const u8).add(INNER_OFF) as *const u32);
        if inner == 0 {
            return 0;
        }
        let gate = *((inner as *const u8).add(GATE_OFF) as *const u32);
        let target = if gate != 0 && *(((gate as *const u8).add(FLAG_OFF))) != 0 {
            callee_thiscall!(1, u32, gate.wrapping_add(GATE_BLOCK))
        } else {
            let probe_input = *((inner as *const u8).add(PROBE_OFF) as *const u32);
            let probe: u32 = callee_thiscall!(2, u32, probe_input);
            callee_cdecl!(3, u32, probe, 0, relocated(0x1112778), relocated(0x11128A8), 0)
        };
        if target == 0 {
            return 0;
        }
        let vtable = *(target as *const u32);
        let slot = *((vtable as *const u8).add(NOTIFY_SLOT) as *const u32);
        let notify: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let _: u32 = notify(target, inner, this as u32);
        1
    }
});
