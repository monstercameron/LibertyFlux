// original: 0x008DB760 task_ctor_route_from_registry_flags
/// Initialises the task header and scalar fields, reads a fabricated registry
/// entry selected by `registry_index`, follows its primary or fallback
/// pointer chain, and passes the resulting one-bit flag to the shared task
/// helper. It then notifies the shared manager and returns the destination.
/// Registry paths are tested for both zero and nonzero pointer branches.
lf_checker_rt::export!(thiscall, rw_008db760(
    this: u32,
    registry_index: u32,
    helper_argument: u32,
    field_at_10: u32,
    byte_a: u32,
    byte_b: u32,
    byte_c: u32,
    field_at_1c: u32,
) -> u32 {
    unsafe {
        const COOKIE_MASK: u32 = 0x3fff;
        const COOKIE_MARKER: u32 = 0x00e7e048;
        const OBJECT_VTABLE: u32 = 0x00e81d48;
        const COUNTER_FILE_VA: u32 = 0x010327a0;
        const REGISTRY_TABLE_FILE_VA: u32 = 0x01295cd8;
        const MANAGER_CONTEXT_FILE_VA: u32 = 0x011d4ec8;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
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

        write_byte(this.wrapping_add(0x0a), byte_a as u8);
        write_byte(this.wrapping_add(0x0b), byte_b as u8);
        write_byte(this.wrapping_add(0x0c), byte_c as u8);
        write_word(this.wrapping_add(0x10), field_at_10);
        write_word(this.wrapping_add(0x1c), field_at_1c);
        (this.wrapping_add(8) as *mut u16).write_unaligned(registry_index as u16);

        let table_slot = lf_checker_rt::global::<u32>(
            REGISTRY_TABLE_FILE_VA.wrapping_add(registry_index.wrapping_mul(4)),
        );
        let registry_entry = table_slot.read_unaligned();
        let primary = read_word(registry_entry.wrapping_add(8));
        let selected = if primary != 0 {
            read_word(primary.wrapping_add(0xb4))
        } else {
            let fallback = read_word(registry_entry.wrapping_add(0x0c));
            let fallback_value = read_word(fallback);
            fallback_value
        };
        let dispatch = read_word(selected.wrapping_add(0x40));
        let mut task_flag = 0u32;
        if dispatch != 0 {
            let first = read_word(dispatch);
            let second = read_word(first);
            task_flag = u32::from(read_byte(second.wrapping_add(0x15)) & 1 != 0);
        }

        let shared_helper = lf_checker_rt::callee_thiscall!(
            1, u32, task_flag, helper_argument,
            this.wrapping_add(0x14), this.wrapping_add(0x18),
            0u32, task_flag, 0u32, 0u32,
        );
        let manager_context = lf_checker_rt::relocated(MANAGER_CONTEXT_FILE_VA);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, manager_context, registry_index);
        let _ = shared_helper;
        this
    }
});
