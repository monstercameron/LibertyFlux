// original: 0x00602DA0 rm_ptfx_manager_refresh_slots
/// Clears two selected words in every particle-manager slot, then walks each
/// slot's linked payloads. Payloads with either dispatch flag set invoke the
/// node dispatcher; every payload invokes the slot updater, and payloads with
/// a callback object receive the incoming XMM1 single-precision bits. The
/// manager's 16-bit slot count is at `+0x270`, its slot-array pointer is at
/// `+0x26c`, and slots are `0x30` bytes apart. The relocated settings global
/// points to the byte selector at `+1`. Returns the final zero-extended count.
lf_checker_rt::export!(thiscall, rm_ptfx_manager_refresh_slots(manager: u32) -> u32 {
    const SLOT_ARRAY: u32 = 0x26c;
    const SLOT_COUNT: u32 = 0x270;
    const SLOT_STRIDE: u32 = 0x30;
    const FIRST_CLEAR: u32 = 0x04;
    const SECOND_CLEAR: u32 = 0x14;
    const NODE_NEXT: u32 = 0x00;
    const NODE_PAYLOAD: u32 = 0x10;
    const PAYLOAD_FLAG_A: u32 = 0x00;
    const PAYLOAD_FLAG_B: u32 = 0x08;
    const PAYLOAD_DISPATCH: u32 = 0x0c;
    const PAYLOAD_CALLBACK: u32 = 0x78;
    const SETTINGS_GLOBAL: u32 = 0x01bb_6678;
    const SELECTED_INDEX: u32 = 1;
    const SLOT_UPDATER: u32 = 1;
    const NODE_DISPATCHER: u32 = 2;
    const PAYLOAD_CALLBACK_METHOD: u32 = 3;

    let read_u8 = |address: u32| unsafe {
        core::ptr::read_volatile(address as *const u8)
    };
    let read_u16 = |address: u32| unsafe {
        core::ptr::read_volatile(address as *const u16)
    };
    let read_u32 = |address: u32| unsafe {
        core::ptr::read_volatile(address as *const u32)
    };
    let write_u32 = |address: u32, value: u32| unsafe {
        core::ptr::write_volatile(address as *mut u32, value)
    };

    let blend_bits = lf_checker_rt::xmm_word(1, 0);
    let mut slot_index = 0u32;
    let mut slot_count = u32::from(read_u16(manager.wrapping_add(SLOT_COUNT)));
    while slot_index < slot_count {
        let settings = unsafe {
            core::ptr::read_volatile(lf_checker_rt::global::<u32>(SETTINGS_GLOBAL))
        };
        let slots = read_u32(manager.wrapping_add(SLOT_ARRAY));
        let slot = slots.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
        let selected = u32::from(read_u8(settings.wrapping_add(SELECTED_INDEX)));
        let clear_index = selected.wrapping_mul(4);
        write_u32(slot.wrapping_add(clear_index).wrapping_add(FIRST_CLEAR), 0);
        write_u32(slot.wrapping_add(clear_index).wrapping_add(SECOND_CLEAR), 0);

        let mut node = read_u32(slot);
        while node != 0 {
            let next_node = read_u32(node.wrapping_add(NODE_NEXT));
            let payload = node.wrapping_add(NODE_PAYLOAD);
            if read_u32(payload.wrapping_add(PAYLOAD_FLAG_A)) != 0
                || read_u32(payload.wrapping_add(PAYLOAD_FLAG_B)) != 0
            {
                let _dispatcher = read_u32(payload.wrapping_add(PAYLOAD_DISPATCH));
                let _ = lf_checker_rt::callee_cdecl!(
                    NODE_DISPATCHER,
                    u32,
                    payload,
                    payload
                );
            }
            let _ = lf_checker_rt::callee_thiscall!(SLOT_UPDATER, u32, payload);
            let callback = read_u32(payload.wrapping_add(PAYLOAD_CALLBACK));
            if callback != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    PAYLOAD_CALLBACK_METHOD,
                    u32,
                    callback,
                    payload,
                    blend_bits
                );
            }
            node = next_node;
        }

        slot_index = slot_index.wrapping_add(1);
        slot_count = u32::from(read_u16(manager.wrapping_add(SLOT_COUNT)));
    }
    slot_count
});
