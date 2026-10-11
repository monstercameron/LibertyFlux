// original: 0x00cab200 event_pair_matches_policy

/// Checks a candidate event against the event and optional linked objects
/// held at `this + 4`, `this + 8` and `this + 0x0c`. It compares the event
/// type values through a cdecl helper, then applies the event's virtual
/// decision and gate methods on the original branches. The second argument's
/// low byte controls the direct linked-object check. Boolean results are
/// returned in AL; all object pointers use the 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00cab200(this: u32, event: u32, gate_flags: u32) -> u32 {
    const RELATED_OBJECT: u32 = 4;
    const OPTIONAL_OBJECT: u32 = 8;
    const FINAL_OBJECT: u32 = 0x0c;
    const EVENT_TYPE_SLOT: u32 = 4;
    const EVENT_GATE_SLOT: u32 = 0x4c;
    const EVENT_DECISION_SLOT: u32 = 0x50;
    const TYPE_EQUAL_CALLEE: u32 = 2;
    const OBJECT_ACCEPT_CALLEE: u32 = 5;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn virtual_type(object: u32, slot: u32) -> u32 {
        let table = unsafe { read_u32(object) };
        let target = unsafe { read_u32(table + slot) };
        let method: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        method(object)
    }

    #[inline(always)]
    unsafe fn virtual_flag(object: u32, slot: u32) -> u8 {
        let table = unsafe { read_u32(object) };
        let target = unsafe { read_u32(table + slot) };
        let method: extern "thiscall" fn(u32) -> u8 =
            unsafe { core::mem::transmute(target as usize) };
        method(object)
    }

    #[inline(always)]
    unsafe fn virtual_decision(object: u32, slot: u32, linked: u32) -> u8 {
        let table = unsafe { read_u32(object) };
        let target = unsafe { read_u32(table + slot) };
        let method: extern "thiscall" fn(u32, u32) -> u8 =
            unsafe { core::mem::transmute(target as usize) };
        method(object, linked)
    }

    unsafe {
        let related_object = read_u32(this + RELATED_OBJECT);
        let optional_object = read_u32(this + OPTIONAL_OBJECT);
        let final_object = read_u32(this + FINAL_OBJECT);

        if related_object != 0 {
            let related_type = virtual_type(related_object, EVENT_TYPE_SLOT);
            let event_type = virtual_type(event, EVENT_TYPE_SLOT);
            let equal = lf_checker_rt::callee_cdecl!(
                TYPE_EQUAL_CALLEE,
                u8,
                event_type,
                related_type
            );
            if equal != 0 {
                return 1;
            }
            return u32::from(virtual_decision(event, EVENT_DECISION_SLOT, related_object));
        }

        if optional_object == 0 {
            return 1;
        }

        let event_gate = virtual_flag(event, EVENT_GATE_SLOT);
        if event_gate != 0 && (gate_flags as u8) == 0 {
            let accepted = lf_checker_rt::callee_thiscall!(
                OBJECT_ACCEPT_CALLEE,
                u8,
                event,
                optional_object
            );
            return u32::from(accepted);
        }

        let optional_type = virtual_type(optional_object, EVENT_TYPE_SLOT);
        let event_type = virtual_type(event, EVENT_TYPE_SLOT);
        let equal = lf_checker_rt::callee_cdecl!(
            TYPE_EQUAL_CALLEE,
            u8,
            event_type,
            optional_type
        );
        if equal == 0 && virtual_decision(event, EVENT_DECISION_SLOT, optional_object) == 0 {
            return 0;
        }

        if final_object == 0 {
            return 1;
        }
        u32::from(lf_checker_rt::callee_thiscall!(
            OBJECT_ACCEPT_CALLEE,
            u8,
            event,
            final_object
        ))
    }
});
