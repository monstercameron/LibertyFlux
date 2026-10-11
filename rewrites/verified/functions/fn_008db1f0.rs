// original: 0x008DB1F0 task_ctor_load_registry_record
/// Initialises the task header and fields from ten stack arguments, calls the
/// base and task helpers, then reads the selected registry record's word at
/// offset 0x12c. The scripted parser writes two local words and returns a
/// status; a zero status skips the range and manager calls, while a nonzero
/// status requests a range scaled by 64 and forwards the local record to the
/// second helper. The function uses a 32-bit thiscall convention.
lf_checker_rt::export!(thiscall, rw_008db1f0(
    this: u32,
    registry_index: u32,
    resource_id: u32,
    amount: u32,
    field_at_a0: u32,
    byte_a: u32,
    byte_b: u32,
    byte_c: u32,
    byte_d: u32,
    field_at_b0: u32,
    byte_e: u32,
) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e818cc;
        const COUNTER_FILE_VA: u32 = 0x010327a0;
        const REGISTRY_TABLE_FILE_VA: u32 = 0x01295cd8;
        const MANAGER_CONTEXT_FILE_VA: u32 = 0x011d4ec8;
        const RANGE_CONTEXT_FILE_VA: u32 = 0x01175c58;
        const STACK_FILL: u32 = 0xa5a5a5a5;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let counter = lf_checker_rt::global::<u32>(COUNTER_FILE_VA);
        let current_count = counter.read_unaligned();
        let old_cookie = read_word(this.wrapping_add(4));
        write_word(this, lf_checker_rt::relocated(COOKIE_MARKER));
        write_word(
            this.wrapping_add(4),
            old_cookie ^ ((old_cookie ^ current_count) & COOKIE_MASK),
        );
        counter.write_unaligned(current_count.wrapping_add(1));
        write_word(this, lf_checker_rt::relocated(OBJECT_VTABLE));

        let base_context = this.wrapping_add(0x10);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, base_context);
        (this.wrapping_add(0xa8) as *mut u16).write_unaligned(registry_index as u16);
        write_byte(this.wrapping_add(0xaa), byte_a as u8);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, base_context, amount);
        write_byte(this.wrapping_add(0xab), byte_b as u8);
        write_byte(this.wrapping_add(0xac), byte_c as u8);
        write_word(this.wrapping_add(0xa0), field_at_a0);
        write_byte(this.wrapping_add(0xae), byte_d as u8);
        write_word(this.wrapping_add(0xb0), field_at_b0);
        write_byte(this.wrapping_add(0xb4), byte_e as u8);

        let table_slot = lf_checker_rt::global::<u32>(
            REGISTRY_TABLE_FILE_VA.wrapping_add(registry_index.wrapping_mul(4)),
        );
        let registry_entry = table_slot.read_unaligned();
        let registry_value = read_word(registry_entry.wrapping_add(0x12c));
        let mut local_record = [STACK_FILL; 2];
        let local_record_ptr = local_record.as_mut_ptr() as u32;
        let parser_status = lf_checker_rt::callee_cdecl!(
            3, u32, local_record_ptr, registry_value, resource_id, amount,
        );
        write_byte(this.wrapping_add(0xad), parser_status as u8);

        if parser_status != 0 {
            let scaled_amount = parser_status.wrapping_shl(6);
            let range_context = lf_checker_rt::relocated(RANGE_CONTEXT_FILE_VA);
            let allocation = lf_checker_rt::callee_thiscall!(4, u32, range_context, 0u32, scaled_amount);
            let _ = lf_checker_rt::callee_cdecl!(
                5, u32, allocation, resource_id, local_record_ptr, amount,
            );
            let manager_context = lf_checker_rt::relocated(MANAGER_CONTEXT_FILE_VA);
            let _ = lf_checker_rt::callee_thiscall!(6, u32, manager_context, registry_index);
        }
        this
    }
});
