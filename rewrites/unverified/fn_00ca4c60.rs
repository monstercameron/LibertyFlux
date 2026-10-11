// original: 0x00ca4c60 CEventDamage::vf20

/// Compares an event with this damage event through their virtual interfaces.
/// It returns true for the special event types accepted when the flag is set
/// and the disable byte is nonzero, for an unequal related-event result or
/// when that same flag/disable condition holds,
/// or when the related-event helper accepts the event. Otherwise it returns
/// whether this event's interface priority is at least the other priority,
/// comparing both values as signed 32-bit integers. The event and any
/// linked entity are read-only; virtual calls are made through their table
/// entries so the checker can plant the same stand-ins in both executions.
///
/// The boolean result occupies AL. Object offsets and pointers use the
/// original 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00ca4c60(this: u32, event: u32) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0;
        const EVENT_TYPE_SLOT: u32 = 4;
        const EVENT_PRIORITY_SLOT: u32 = 8;
        const EVENT_MATCH_SLOT: u32 = 0x34;
        const EVENT_FLAGS: u32 = 0x28;
        const EVENT_DISABLED: u32 = 0x0c;
        const LINKED_ENTITY: u32 = 0x18;
        const ENTITY_FLAGS: u32 = 0x28;
        const ENTITY_STATE: u32 = 0x219;
        const LOOKUP_VALUE: u32 = 0x34;
        const LOOKUP_CALLEE: u32 = 4;
        const EVENT_TYPE_CALLEE: u32 = 1;
        const EVENT_MATCH_CALLEE: u32 = 2;
        const EVENT_PRIORITY_CALLEE: u32 = 3;
        const FLAG_MASK: u8 = 4;
        const ENTITY_MASK: u32 = 0x3c0;
        const ENTITY_REQUIRED: u32 = 0xc0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn event_method(object: u32, slot: u32) -> u32 {
            let table = unsafe { read_u32(object + EVENT_VTABLE) };
            let target = unsafe { read_u32(table + slot) };
            let method: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(object)
        }

        let event_flag_is_set = unsafe { read_u8(this + EVENT_FLAGS) & FLAG_MASK != 0 };
        let disabled_byte = unsafe { read_u8(this + EVENT_DISABLED) };
        let special_type_is_enabled = event_flag_is_set && disabled_byte != 0;
        let accepted = 'decision: {
            let first_type = unsafe { event_method(event, EVENT_TYPE_SLOT) };
            if first_type == 0x42 && special_type_is_enabled {
                break 'decision 1;
            }

            let second_type = unsafe { event_method(event, EVENT_TYPE_SLOT) };
            if second_type == 0x3a && special_type_is_enabled {
                break 'decision 1;
            }

            let linked_entity = unsafe { read_u32(this + LINKED_ENTITY) };
            let entity_matches = linked_entity != 0
                && (unsafe { read_u32(linked_entity + ENTITY_FLAGS) } & ENTITY_MASK)
                    == ENTITY_REQUIRED
                && unsafe { read_u8(linked_entity + ENTITY_STATE) != 0 };
            if entity_matches {
                let linked_event_type = unsafe { event_method(event, EVENT_TYPE_SLOT) };
                if linked_event_type == 9 {
                    let _ = unsafe { event_method(event, EVENT_MATCH_SLOT) };
                    break 'decision 1;
                }
            }

            let related_type = unsafe { event_method(event, EVENT_TYPE_SLOT) };
            if related_type == 9 {
                let event_match = unsafe { event_method(event, EVENT_MATCH_SLOT) };
                let this_match = unsafe { event_method(this, EVENT_MATCH_SLOT) };
                if event_match != this_match || special_type_is_enabled {
                    break 'decision 1;
                }
            }

            let keyed_type = unsafe { event_method(event, EVENT_TYPE_SLOT) };
            if keyed_type == 0x82 {
                let lookup_value = unsafe { read_u32(this + LOOKUP_VALUE) };
                if lookup_value != 0 {
                    let lookup_result = lf_checker_rt::callee_cdecl!(
                        LOOKUP_CALLEE,
                        u32,
                        lookup_value,
                        0,
                        lf_checker_rt::relocated(0x0111_2778),
                        lf_checker_rt::relocated(0x0111_28a8),
                        0
                    );
                    if lookup_result != 0 {
                        break 'decision 1;
                    }
                }
            }

            let event_priority = unsafe { event_method(event, EVENT_PRIORITY_SLOT) };
            let this_priority = unsafe { event_method(this, EVENT_PRIORITY_SLOT) };
            u32::from((this_priority as i32) >= (event_priority as i32))
        };
        accepted
    }
});
