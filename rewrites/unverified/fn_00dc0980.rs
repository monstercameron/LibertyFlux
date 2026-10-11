// original: 0x00DC0980 task_flag_compatibility

/// Test whether a task request is compatible with the object's flag byte.
///
/// The first stack argument points to the object and its flag byte is at
/// offset 0x14. The second points to a request: its type occupies bits
/// 13-14 of the first word, its secondary selector is bit 15, and the high
/// bit of the next word rejects the request. Types 1 and 2 require object
/// flag bit 1 to be clear; type 3 or a selected secondary mode requires bit
/// 2 to be clear; the secondary mode also requires bit 3 to be set. The
/// result is the low-byte boolean used by the original stdcall routine.
lf_checker_rt::export!(stdcall, rw_00DC0980(object: u32, request: u32) -> u8 {
    unsafe {
        const OBJECT_FLAGS: u32 = 0x14;
        const TYPE_SHIFT: u32 = 13;
        const TYPE_MASK: u32 = 0x03;
        const SECONDARY_SHIFT: u32 = 15;
        const HIGH_REJECT_SHIFT: u32 = 28;
        const REQUIRE_SECONDARY_FLAG: u8 = 0x08;
        const REJECT_SECONDARY_FLAG: u8 = 0x04;
        const REJECT_TYPE_FLAG: u8 = 0x02;

        let request_bits = (request as *const u32).read_unaligned();
        let request_type = (request_bits >> TYPE_SHIFT) & TYPE_MASK;
        if request_type == 0 {
            return 1;
        }

        let high_bits = (request.wrapping_add(4) as *const u32).read_unaligned();
        if (high_bits >> HIGH_REJECT_SHIFT) & 1 != 0 {
            return 0;
        }

        let secondary = (request_bits >> SECONDARY_SHIFT) & 1 != 0;
        let flags = (object.wrapping_add(OBJECT_FLAGS) as *const u8).read();
        if secondary && flags & REQUIRE_SECONDARY_FLAG == 0 {
            return 0;
        }
        if (request_type == 3 || secondary) && flags & REJECT_SECONDARY_FLAG != 0 {
            return 0;
        }
        if (request_type == 1 || request_type == 2) && flags & REJECT_TYPE_FLAG != 0 {
            return 0;
        }
        1
    }
});
