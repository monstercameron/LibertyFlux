// original: 0x008F6A60 Input_Init

/// Initialise input: notify with `(arg0, 1)`, mark the mode register -1,
/// link the four slots to their state blocks and construct each, construct
/// the shared slot, initialise the four objects with 0, tear down the
/// default object, reset the device record, then set the ready flag and clear
/// the pending byte. Returns the device reset's answer. The slot loop bound
/// is a signed comparison. Convention: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008f6a60(arg0: u32) -> u32 {
    unsafe {
        const SLOT_FIRST: u32 = 0x0118D470;
        const SLOT_STRIDE: u32 = 0xBC;
        const NOTIFY: u32 = 1;
        const CONSTRUCT: u32 = 2;
        const INIT_OBJ: u32 = 3;
        const TEARDOWN: u32 = 4;
        const RESET_DEV: u32 = 5;
        const MODE_REG: u32 = 0x010330F8;
        const STATE_FIRST: u32 = 0x019F2388;
        const STATE_LAST: u32 = 0x019F2468;
        const STATE_STRIDE: u32 = 0x38;
        const SHARED_SLOT: u32 = 0x0118D3B0;
        const READY_FLAG: u32 = 0x018B7A54;
        const PENDING_BYTE: u32 = 0x0117E6D9;
        lf_checker_rt::callee_cdecl!(NOTIFY, u32, arg0, 1);
        lf_checker_rt::global::<u32>(MODE_REG).write_unaligned(0xFFFF_FFFF);
        let mut slot = lf_checker_rt::relocated(SLOT_FIRST);
        let mut state = lf_checker_rt::relocated(STATE_FIRST);
        let state_last = lf_checker_rt::relocated(STATE_LAST);
        loop {
            (slot as *mut u32).write_unaligned(state);
            let _: u32 = lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, slot);
            state = state.wrapping_add(STATE_STRIDE);
            slot = slot.wrapping_add(SLOT_STRIDE);
            if !((state as i32) < (state_last as i32)) {
                break;
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CONSTRUCT,
            u32,
            lf_checker_rt::relocated(SHARED_SLOT)
        );
        for obj in [0x0117E700_u32, 0x01182184, 0x01185C08, 0x0118968C] {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                INIT_OBJ,
                u32,
                lf_checker_rt::relocated(obj),
                0
            );
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            TEARDOWN,
            u32,
            lf_checker_rt::relocated(0x01185C08)
        );
        let r: u32 = lf_checker_rt::callee_thiscall!(
            RESET_DEV,
            u32,
            lf_checker_rt::relocated(0x0118D110)
        );
        lf_checker_rt::global::<u32>(READY_FLAG).write_unaligned(1);
        lf_checker_rt::global::<u8>(PENDING_BYTE).write(0);
        r
    }
});
