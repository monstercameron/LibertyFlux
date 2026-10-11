// original: 0x00B25B20 CEntity_SetFrozen

/// Sets or clears the entity's freeze bits from two byte-valued options. The
/// second option chooses bit `0x10` versus bit `0x04`. With a physics record,
/// any nonzero first byte chooses set versus clear; with no record, only its
/// low bit chooses the flag value. With no physics record, only that bit update occurs. With a
/// record, setting calls the entity helper and returns; clearing may dispatch
/// an indexed service when the unsigned record index is not `0xffff`, bit
/// `0x04` is clear, and the runtime table entry has low bits equal to two. A
/// final helper runs only when the entity type mask is exactly `0xc0` and the
/// selected entity flags are clear.
lf_checker_rt::export!(thiscall, rw_b25b20(this: u32, set_frozen: u32, keep_collision: u32) -> () {
    const PHYSICS_RECORD: u32 = 0x38;
    const ENTITY_FLAGS: u32 = 0x24;
    const ENTITY_TYPE: u32 = 0x28;
    const RECORD_INDEX: u32 = 0x08;
    const MANAGER_TABLE: u32 = 0x70;
    const TYPE_MASK: u32 = 0x3C0;
    const TYPE_MATCH: u32 = 0xC0;
    const INDEXED_ENTRY_STRIDE: u32 = 8;
    const ENTRY_FLAGS: u32 = 4;
    const SERVICE_MANAGER: u32 = 0x12B9_C78;
    const SERVICE_RECEIVER: u32 = 0x12B9_C7C;

    const REFERENCE_HELPER: u32 = 1;
    const INDEXED_SERVICE: u32 = 2;
    const RECORD_SERVICE: u32 = 3;
    const CONTACT_HELPER: u32 = 4;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let record = read_u32(this.wrapping_add(PHYSICS_RECORD));
        let selected_bit = if keep_collision as u8 != 0 { 0x10 } else { 0x04 };
        let flags_address = this.wrapping_add(ENTITY_FLAGS);
        let mut flags = read_u32(flags_address);

        if record == 0 {
            flags = if set_frozen as u8 & 1 != 0 { flags | selected_bit } else { flags & !selected_bit };
            write_u32(flags_address, flags);
            return;
        }

        if set_frozen as u8 != 0 {
            flags |= selected_bit;
            write_u32(flags_address, flags);
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_HELPER, u32, this);
            return;
        }

        flags &= !selected_bit;
        write_u32(flags_address, flags);

        let index = read_u16(record.wrapping_add(RECORD_INDEX));
        if index != u16::MAX && flags & 0x04 == 0 {
            let manager = lf_checker_rt::global::<u32>(SERVICE_MANAGER).read_unaligned();
            let table = read_u32(manager.wrapping_add(MANAGER_TABLE));
            let entry = table
                .wrapping_add((index as u32).wrapping_mul(INDEXED_ENTRY_STRIDE))
                .wrapping_add(ENTRY_FLAGS);
            if read_u32(entry) & 3 == 2 {
                let service = lf_checker_rt::global::<u32>(SERVICE_RECEIVER).read_unaligned();
                let _ = lf_checker_rt::callee_thiscall!(INDEXED_SERVICE, u32, service, index as u32, 0);
                let _ = lf_checker_rt::callee_thiscall!(RECORD_SERVICE, u32, service, record, 0);
            }
        }

        let entity_type = read_u32(this.wrapping_add(ENTITY_TYPE)) & TYPE_MASK;
        if entity_type == TYPE_MATCH && read_u32(flags_address) & 0x1C == 0 {
            let _ = lf_checker_rt::callee_thiscall!(CONTACT_HELPER, u32, this);
        }
    }
});
