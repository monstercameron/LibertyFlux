// original: 0x005D2CA0 blip_slot_resolve_net_object

/// Resolve an empty blip slot's 16-bit network key and attach the returned
/// entity when its timing mode is active.
///
/// `this` is the slot. A nonempty slot is returned unchanged. An empty slot
/// with a zero key is also left unchanged. Otherwise the function asks the
/// mission entity registrar for the network object, calls virtual slot 5 on
/// that object to obtain its entity, checks the entity's timing-mode word at
/// `+0x12c`, and, when that word is `2`, stores the entity in the slot, clears
/// the key, and notifies the parent list. All pointers are 32-bit game-layout
/// addresses in this checker build; the registrar address is relocated from
/// the image base.
lf_checker_rt::export!(thiscall, rw_005d2ca0(this: u32) -> u32 {
    unsafe {
        const SLOT_ENTITY: usize = 0;
        const SLOT_KEY: usize = 0x40;
        const NET_OBJECT_VTABLE_SLOT5: usize = 0x14;
        const ENTITY_MODE_POINTER: usize = 0x21C;
        const ENTITY_MODE_VALUE: usize = 0x12C;
        const ACTIVE_MODE: u32 = 2;
        const MISSION_ENTITY_REGISTRAR: u32 = 0x01908EF0;

        let slot = this as *mut u8;
        let mut entity = (slot as *mut u32).read_unaligned();
        if entity == 0 {
            let key = (slot.add(SLOT_KEY) as *const u16).read_unaligned() as u32;
            if key != 0 {
                let registrar = lf_checker_rt::relocated(MISSION_ENTITY_REGISTRAR);
                let network_object = lf_checker_rt::callee_thiscall!(1, u32, registrar, key, 0);
                if network_object != 0 {
                    let vtable = (network_object as *const u32).read_unaligned();
                    let method = *((vtable + NET_OBJECT_VTABLE_SLOT5 as u32) as *const u32);
                    let resolve: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(method as usize);

                    if resolve(network_object) != 0 {
                        let mode_owner = resolve(network_object);
                        let mode_pointer = *((mode_owner + ENTITY_MODE_POINTER as u32) as *const u32);
                        let mode = *((mode_pointer + ENTITY_MODE_VALUE as u32) as *const u32);
                        if mode == ACTIVE_MODE {
                            entity = resolve(network_object);
                            (slot as *mut u32).write_unaligned(entity);
                            let _ = lf_checker_rt::callee_thiscall!(3, u32, entity, this);
                            (slot.add(SLOT_KEY) as *mut u16).write_unaligned(0);
                        }
                    }
                }
            }
        }
        entity
    }
});
