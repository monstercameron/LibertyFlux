// original: 0x00923140 release_resource_slots

/// Release one resource slot, or every populated slot for a negative index.
///
/// A nonnegative signed index selects one slot. A negative index scans the
/// four-slot table in order. For each nonzero count, the function reads that
/// slot's release code, calls the resource release helper, calls the object
/// helper with the record pointer and count, clears the 0x17e8-byte record,
/// then stores zero in the slot count. The final clear helper's EAX is returned
/// when a populated slot was processed; otherwise the return register is not
/// specified by this routine.
///
/// Calling convention: cdecl with one signed 32-bit index. Global file
/// addresses are relocated through the checker runtime before access.
lf_checker_rt::export!(cdecl, rw_00923140(index_bits: u32) -> u32 {
    const COUNT_TABLE_VA: u32 = 0x0119_5EA0;
    const RELEASE_CODE_TABLE_VA: u32 = 0x0119_56B8;
    const OBJECT_TABLE_VA: u32 = 0x0119_5F70;
    const RELEASE_CODE_STRIDE: u32 = 0x258;
    const OBJECT_STRIDE: u32 = 0x17E8;
    const OBJECT_BYTES: u32 = 0x17E8;
    const RELEASE_CODE_END_VA: u32 = 0x0119_6018;
    const ENTRY_COUNT: u32 = (RELEASE_CODE_END_VA - RELEASE_CODE_TABLE_VA)
        / RELEASE_CODE_STRIDE;
    const RELEASE_CALLEE: u32 = 1;
    const OBJECT_CALLEE: u32 = 2;
    const CLEAR_CALLEE: u32 = 3;

    let release_one = |slot: u32| -> Option<u32> {
        let count_address = COUNT_TABLE_VA.wrapping_add(slot.wrapping_mul(4));
        let count_pointer = lf_checker_rt::global::<u32>(count_address);
        let count = unsafe { count_pointer.read() };
        if count == 0 {
            return None;
        }

        let release_code_address = RELEASE_CODE_TABLE_VA
            .wrapping_add(slot.wrapping_mul(RELEASE_CODE_STRIDE));
        let release_code = unsafe { lf_checker_rt::global::<u32>(release_code_address).read() };
        let object_address = OBJECT_TABLE_VA.wrapping_add(slot.wrapping_mul(OBJECT_STRIDE));
        let object_pointer = lf_checker_rt::relocated(object_address);

        let _released = lf_checker_rt::callee_cdecl!(
            RELEASE_CALLEE,
            u32,
            release_code,
            1u32,
        );
        let _object_result = lf_checker_rt::callee_cdecl!(
            OBJECT_CALLEE,
            u32,
            object_pointer,
            count,
        );
        let clear_result = lf_checker_rt::callee_cdecl!(
            CLEAR_CALLEE,
            u32,
            object_pointer,
            0u32,
            OBJECT_BYTES,
        );
        unsafe { count_pointer.write(0) };
        Some(clear_result)
    };

    let signed_index = i32::from_ne_bytes(index_bits.to_ne_bytes());
    if signed_index < 0 {
        let mut last_clear_result = None;
        for slot in 0..ENTRY_COUNT {
            if let Some(result) = release_one(slot) {
                last_clear_result = Some(result);
            }
        }
        last_clear_result.unwrap_or(0)
    } else {
        release_one(signed_index as u32).unwrap_or(0)
    }
});
