// original: 0x00cc9e60 ARTFeedbackInterfaceGta::vf2
/// ART feedback slot 2: resolve the live feedback target, then notify it.
///
/// Reads the outer handle at +0x190 and its inner object at +0xC. When the
/// inner gate at +0x6C is set and its flag byte at +0xE is set, the target
/// comes from the gate helper (thiscall/0, id 1) on the gate block +0x808;
/// otherwise a probe (thiscall/0, id 2) on the field at +0x224 is wrapped by
/// the feedback lookup (cdecl/5, id 3). A null target returns 0, else the
/// slot-0x50 virtual notification fires with (inner, this) and the function
/// returns 1.
export!(thiscall, rw_00cc9e60(this: *mut u8) -> u32 {
    unsafe {
        const OUTER_OFF: usize = 0x190;
        const INNER_OFF: usize = 0x0C;
        const GATE_OFF: usize = 0x6C;
        const FLAG_OFF: usize = 0x0E;
        const GATE_BLOCK: u32 = 0x808;
        const PROBE_OFF: usize = 0x224;
        const NOTIFY_SLOT: usize = 0x50;
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
