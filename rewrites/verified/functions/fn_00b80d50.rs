// original: 0x00B80D50 ped_task_manager_first_occupied_slot

/// Return the index of the first non-null entry in a five-pointer task array,
/// or five when all five slots are null. The pointer is passed in ECX; the
/// function reads five consecutive 32-bit entries and writes no memory.
lf_checker_rt::export!(thiscall, rw_b80d50(this: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 5;
        let slots = this as *const u32;
        let mut index = 0u32;
        while index < SLOT_COUNT {
            let task = slots.add(index as usize).read_unaligned();
            if task != 0 {
                return index;
            }
            index += 1;
        }
        SLOT_COUNT
    }
});
