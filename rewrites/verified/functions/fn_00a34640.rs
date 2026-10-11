// original: 0x00A34640 find_entity_world_entry

/// Searches two fixed world-entry tables for an entity key and reports
/// whether either table contains it. The first table has 90 records spaced
/// 0x2c bytes apart; the second has 90 records spaced 0x24 bytes apart. Only
/// each record's first dword is compared. On a match the zero-based record
/// index is sent to that table's helper. The original constructs and cleans
/// up two-dword stack-local iterator objects around the scans; their contents are
/// observed at each helper boundary by the proof contract. This is a cdecl
/// function with one 32-bit entity key argument and a boolean result in EAX.
lf_checker_rt::export!(cdecl, rw_00a34640(entity_key: u32) -> u32 {
    const FIRST_TABLE_VA: u32 = 0x012d_e150;
    const SECOND_TABLE_VA: u32 = 0x012d_f0c8;
    const FIRST_DESCRIPTOR_VA: u32 = 0x012d_fd80;
    const SECOND_DESCRIPTOR_VA: u32 = 0x012e_1ec0;
    const FIRST_STRIDE: u32 = 0x2c;
    const SECOND_STRIDE: u32 = 0x24;
    const RECORD_COUNT: u32 = 90;

    unsafe {
        let mut first_iterator = [0u32; 2];
        let mut second_iterator = [0u32; 2];
        let first_iterator_ptr = first_iterator.as_mut_ptr() as u32;
        let second_iterator_ptr = second_iterator.as_mut_ptr() as u32;

        let first_descriptor = lf_checker_rt::global::<u8>(FIRST_DESCRIPTOR_VA) as u32;
        lf_checker_rt::callee_thiscall!(1, u32, first_iterator_ptr, first_descriptor);

        let mut first_slot = lf_checker_rt::global::<u8>(FIRST_TABLE_VA) as u32;
        let mut first_index = 0u32;
        while first_index < RECORD_COUNT {
            let value = (first_slot as *const u32).read_unaligned();
            if value == entity_key {
                lf_checker_rt::callee_cdecl!(5, u32, first_index);
                lf_checker_rt::callee_thiscall!(2, u32, first_iterator_ptr);
                lf_checker_rt::callee_thiscall!(3, u32, first_iterator_ptr);
                return 1;
            }
            first_slot = first_slot.wrapping_add(FIRST_STRIDE);
            first_index += 1;
        }

        lf_checker_rt::callee_thiscall!(2, u32, first_iterator_ptr);
        let second_descriptor = lf_checker_rt::global::<u8>(SECOND_DESCRIPTOR_VA) as u32;
        lf_checker_rt::callee_thiscall!(1, u32, second_iterator_ptr, second_descriptor);

        let mut second_slot = lf_checker_rt::global::<u8>(SECOND_TABLE_VA) as u32;
        let mut second_index = 0u32;
        while second_index < RECORD_COUNT {
            let value = (second_slot as *const u32).read_unaligned();
            if value == entity_key {
                lf_checker_rt::callee_cdecl!(4, u32, second_index);
                lf_checker_rt::callee_thiscall!(2, u32, second_iterator_ptr);
                lf_checker_rt::callee_thiscall!(3, u32, second_iterator_ptr);
                lf_checker_rt::callee_thiscall!(3, u32, first_iterator_ptr);
                return 1;
            }
            second_slot = second_slot.wrapping_add(SECOND_STRIDE);
            second_index += 1;
        }

        lf_checker_rt::callee_thiscall!(2, u32, second_iterator_ptr);
        lf_checker_rt::callee_thiscall!(3, u32, second_iterator_ptr);
        lf_checker_rt::callee_thiscall!(3, u32, first_iterator_ptr);
        0
    }
});
