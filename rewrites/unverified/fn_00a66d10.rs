// original: 0x00A66D10 phInstBehaviorExplosionGta::~phInstBehaviorExplosionGta

/// Restore the explosion behavior's vtable, or the vtable of each element in
/// the variable-length record when flag bit 1 is set. The element count is the
/// word immediately before `self`, and records have a 0x1c-byte stride. Flag
/// bit 0 requests the intercepted message-send helper with either `self` or
/// the count-header pointer. The helper is cdecl; the destructor is thiscall,
/// consumes one stack flag, and returns the selected pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00a66d10(instance: u32, flags: u32) -> u32 {
    const ARRAY_FLAG: u8 = 2;
    const SEND_MESSAGE_FLAG: u8 = 1;
    const COUNT_OFFSET: u32 = 4;
    const ELEMENT_STRIDE: u32 = 0x1c;
    const VTABLE_FILE_VA: u32 = 0x00E9DD6C;
    const SEND_MESSAGE_CALLEE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let vtable_value = lf_checker_rt::relocated(VTABLE_FILE_VA);
        let flags = flags as u8;
        let result_pointer = if flags & ARRAY_FLAG != 0 {
            let count_header = instance.wrapping_sub(COUNT_OFFSET);
            let signed_count = read_u32(count_header) as i32;
            let mut element_index = signed_count.wrapping_sub(1);
            while element_index >= 0 {
                let element = instance.wrapping_add((element_index as u32).wrapping_mul(ELEMENT_STRIDE));
                write_u32(element, vtable_value);
                element_index -= 1;
            }
            count_header
        } else {
            write_u32(instance, vtable_value);
            instance
        };

        if flags & SEND_MESSAGE_FLAG != 0 {
            let _ignored_result = lf_checker_rt::callee_cdecl!(
                SEND_MESSAGE_CALLEE,
                u32,
                result_pointer,
            );
        }
        result_pointer
    }
});
