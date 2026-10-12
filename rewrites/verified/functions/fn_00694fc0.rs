// original: 0x00694FC0 rage::crWeightSet::~crWeightSet (proposed)

/// Tear down the weight-set fields at offsets `+4`, `+8`, `+0x0c`, `+0x10`,
/// and `+0x14`, releasing the optional resources at `+0x0c` and `+4` through
/// the thread-local memory manager. The original first installs the
/// destruction vtable and finishes by installing the base vtable; both are
/// relocated image pointers. The conditional release test after clearing
/// `+0x0c` is unreachable because AX was cleared immediately beforehand.
lf_checker_rt::export!(thiscall, rw_00694FC0(weight_set: u32) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00;
        const RESOURCE_AT_4: u32 = 0x04;
        const WORD_FIELD: u32 = 0x08;
        const RESOURCE_AT_C: u32 = 0x0C;
        const CLEARED_POINTER_10: u32 = 0x10;
        const CLEARED_POINTER_14: u32 = 0x14;
        const VTABLE_DESTRUCTION: u32 = 0x00FE3994;
        const VTABLE_BASE: u32 = 0x00E86AFC;
        const RELEASE_RESOURCE_ID: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        unsafe { write_u32(weight_set.wrapping_add(OBJECT_VTABLE), lf_checker_rt::relocated(VTABLE_DESTRUCTION)) };
        unsafe { write_u16(weight_set.wrapping_add(WORD_FIELD), 0) };

        let resource_at_c = unsafe { read_u32(weight_set.wrapping_add(RESOURCE_AT_C)) };
        if resource_at_c != 0 {
            let _ = unsafe {
                lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE_ID, u32, weight_set, resource_at_c)
            };
        }
        unsafe {
            write_u32(weight_set.wrapping_add(RESOURCE_AT_C), 0);
            write_u32(weight_set.wrapping_add(CLEARED_POINTER_10), 0);
            write_u32(weight_set.wrapping_add(CLEARED_POINTER_14), 0);
        }

        let resource_at_4 = unsafe { read_u32(weight_set.wrapping_add(RESOURCE_AT_4)) };
        if resource_at_4 != 0 {
            let _ = unsafe {
                lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE_ID, u32, weight_set, resource_at_4)
            };
        }
        unsafe { write_u32(weight_set.wrapping_add(OBJECT_VTABLE), lf_checker_rt::relocated(VTABLE_BASE)) };
        0
    }
});
