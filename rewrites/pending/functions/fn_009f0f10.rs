// original: 0x009f0f10 ped_clear_matching_slot
/// Find `target` in the four slots at field `0xd58` and clear it.
///
/// Scans the four words in order. Returns 4 when no slot holds `target`.
/// On the first match, releases the stored handle (unless it is null) with
/// the slot's own address, zeroes the slot, and returns the release call's
/// answer (or the slot index when the handle was already null).
export!(thiscall, rw_009f0f10(this_ptr: u32, target: u32) -> u32 {
    unsafe {
        let base = this_ptr + 0xd58;
        let mut index: u32 = 0;
        while index < 4 {
            if *((base + index * 4) as *const u32) == target {
                break;
            }
            index += 1;
        }
        if index == 4 {
            return 4;
        }
        let slot = (base + index * 4) as *mut u32;
        let handle = *slot;
        let mut answer = index;
        if handle != 0 {
            answer = callee_thiscall!(2, u32, handle, slot as u32);
        }
        *slot = 0;
        answer
    }
});
