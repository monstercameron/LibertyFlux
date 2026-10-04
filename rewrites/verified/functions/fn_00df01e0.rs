// original: 0x00df01e0 clip_viewer_refresh_gate
/// Refresh gate for a clip-viewer panel: polls two status words through a
/// shared gate object and runs follow-up actions when they match.
///
/// First asks the gate (id 1) for a status record. When its tag word is
/// READY_TAG and the neighbour check (id 2, the function just below this
/// one) agrees, it refreshes the gate (id 3), marks this panel ready, and
/// emits notification 0x18 (id 4). It then asks the gate for a second
/// status record (id 5): a mode of MODE_A or MODE_B polls the nested
/// sub-object's slot 0x124 (id 6) and, on a nonzero answer, applies the
/// mode's follow-up (id 7 for A, id 9 for B) before finishing through the
/// gate (id 8), whose answer is returned. Any other mode is returned
/// unchanged. Only the low byte of the polled answers is significant.
export!(thiscall, rw_00df01e0(this: u32) -> u32 {
    unsafe {
        const GATE_GLOBAL: u32 = 0x19D2E08;
        const FIRST_KEY: u32 = 0x20F00;
        const SECOND_KEY: u32 = 0xC000;
        const READY_TAG: u32 = 0x10;
        const MODE_A: u32 = 0x17;
        const MODE_B: u32 = 0x18;
        const NOTIFY_CODE: u32 = 0x18;
        const READY_FLAG_OFF: u32 = 0x1F0;
        const INNER_OFF: u32 = 0x1E0;
        const SUB_OFF: u32 = 0x1EC;
        const POLL_SLOT: u32 = 0x124;
        let fetch_first: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let neighbour_ok: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let refresh: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let notify: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(4) as usize);
        let fetch_second: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(5) as usize);
        let apply_a: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(7) as usize);
        let finish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(8) as usize);
        let apply_b: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(9) as usize);

        let gate = relocated(GATE_GLOBAL);
        let first = fetch_first(gate, FIRST_KEY, 1);
        if *((first) as *const u32) == READY_TAG && (neighbour_ok(this) as u8) != 0 {
            refresh(gate, 1);
            *((this.wrapping_add(READY_FLAG_OFF)) as *mut u32) = 1;
            notify(NOTIFY_CODE);
        }
        let second = fetch_second(gate, SECOND_KEY, 1);
        let mode = *((second) as *const u32);
        if mode == MODE_A || mode == MODE_B {
            let inner = *((this.wrapping_add(INNER_OFF)) as *const u32);
            let sub = *((inner.wrapping_add(SUB_OFF)) as *const u32);
            let vtable = *(sub as *const u32);
            let slot = *((vtable.wrapping_add(POLL_SLOT)) as *const u32);
            let poll: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            if (poll(sub) as u8) != 0 {
                if mode == MODE_A {
                    apply_a(sub, 1);
                } else {
                    apply_b(sub, 1);
                }
            }
            finish(gate, 1)
        } else {
            mode
        }
    }
});
