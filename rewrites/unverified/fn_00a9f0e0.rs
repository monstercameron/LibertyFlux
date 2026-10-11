// original: 0x00A9F0E0 loader_queue_resource_update
/// Finds a matching key in the first word of a 128-entry table. It asks the
/// loader for the matching resource, invokes its update hook with the two
/// remaining input words, then conditionally creates and initializes an
/// update record. The boolean result is returned in AL; all three stack
/// inputs are caller-provided 32-bit words, with the last interpreted as a
/// single-precision value when stored.
lf_checker_rt::export!(thiscall, rw_00A9F0E0(this: u32, key: u32, flags: u32, weight_bits: u32) -> u8 {
    unsafe {
        const KEY_TABLE_OFFSET: u32 = 0x38B4;
        const ENTRY_STRIDE: u32 = 0x10;
        const ENTRY_COUNT: u32 = 0x80;
        const LOOKUP_LINK_OFFSET: u32 = 0x0C;
        const RECORD_FLAGS_OFFSET: u32 = 0x00;
        const RECORD_INDEX_OFFSET: u32 = 0x04;
        const RECORD_KEY_OFFSET: u32 = 0x08;
        const RECORD_VALUE_OFFSET: u32 = 0x60;
        const RECORD_STATE_OFFSET: u32 = 0x5B;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut entry_index = 0u32;
        while entry_index < ENTRY_COUNT
            && read_u32(
                this.wrapping_add(KEY_TABLE_OFFSET)
                    .wrapping_add(entry_index.wrapping_mul(ENTRY_STRIDE)),
            ) != key
        {
            entry_index += 1;
        }
        if entry_index == ENTRY_COUNT {
            return 0;
        }

        let lookup = lf_checker_rt::callee_thiscall!(1, u32, this, key);
        if lookup == 0 {
            return 0;
        }
        let update_target = read_u32(lookup.wrapping_add(LOOKUP_LINK_OFFSET));
        let _ = lf_checker_rt::callee_thiscall!(2, u32, update_target, flags, weight_bits);

        let update_mode = lf_checker_rt::global::<u32>(0x011F7060).read_unaligned();
        let configuration = lf_checker_rt::global::<u32>(0x012088B4).read_unaligned();
        let active_configuration = lf_checker_rt::global::<u32>(0x00F1C040).read_unaligned();
        let resource_type = lf_checker_rt::global::<u32>(0x01037720).read_unaligned();
        if update_mode == 1
            || configuration != active_configuration
            || resource_type == 0x12
        {
            return 1;
        }

        let record = lf_checker_rt::callee_thiscall!(3, u32, this);
        if record == 0 {
            return 1;
        }

        write_u32(record.wrapping_add(RECORD_FLAGS_OFFSET), 1);
        write_u32(record.wrapping_add(RECORD_INDEX_OFFSET), entry_index);
        write_u32(record.wrapping_add(RECORD_KEY_OFFSET), key);
        let _ = lf_checker_rt::callee_cdecl!(4, u32, record.wrapping_add(0x50), flags, 0x0B);
        write_u8(record.wrapping_add(RECORD_STATE_OFFSET), 0);
        write_u32(record.wrapping_add(RECORD_VALUE_OFFSET), weight_bits);
        1
    }
});
