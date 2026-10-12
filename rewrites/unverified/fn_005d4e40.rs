// original: 0x005d4e40 compute_entity_audio_flags
//
// Derive the low audio flags from the entity's type bits, a streaming-state
// query and, for selected state combinations, two virtual predicates in the
// entity vtable. The first predicate adds bit 0; the second adds bit 1. The
// base value contains bit 3 from entity flags and bit 0 from the streaming
// query. Only AL is the defined result. Convention: thiscall with `this` in
// ECX and no stack arguments.
lf_checker_rt::export!(thiscall, rw_005d4e40(this: u32) -> u32 {
    unsafe {
        const ID_STREAMING_QUERY: u32 = 1;
        const VTABLE_FIRST_SLOT_OFFSET: u32 = 0x24;
        const VTABLE_SECOND_SLOT_OFFSET: u32 = 0x28;
        const STATE_HAS_AUDIO_PREDICATE: u32 = 0x0100_0000;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn call_virtual_predicate(entity: u32, slot_offset: u32) -> u32 {
            let vtable = unsafe { read_u32(entity) };
            let target = unsafe { read_u32(vtable.wrapping_add(slot_offset)) };
            let predicate: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            predicate(entity)
        }

        let entity_flags = read_u32(this.wrapping_add(0x24));
        let state_flags = read_u32(this.wrapping_add(0x28));
        let streaming_flag: u32 =
            lf_checker_rt::callee_thiscall!(ID_STREAMING_QUERY, u32, this);
        let entity_flag_bit = ((entity_flags >> 27) & 1) << 3;
        let mut result = (entity_flag_bit | (streaming_flag & 1)) as u8;

        if state_flags & STATE_HAS_AUDIO_PREDICATE != 0 {
            if call_virtual_predicate(this, VTABLE_FIRST_SLOT_OFFSET) as u8 != 0 {
                result |= 1;
            } else if call_virtual_predicate(this, VTABLE_SECOND_SLOT_OFFSET) as u8 != 0 {
                result |= 2;
            }
        } else {
            let entity_type = (state_flags >> 6) & 0xf;
            match entity_type {
                3 | 6 => {
                    if call_virtual_predicate(this, VTABLE_FIRST_SLOT_OFFSET) as u8 != 0 {
                        result |= 1;
                    }
                }
                2 => {
                    if call_virtual_predicate(this, VTABLE_SECOND_SLOT_OFFSET) as u8 != 0 {
                        result |= 2;
                    }
                }
                _ => {}
            }
        }
        u32::from(result)
    }
});
