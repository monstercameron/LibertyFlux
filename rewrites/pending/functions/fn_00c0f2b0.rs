// original: 0x00c0f2b0 UIFrame::vf119
/// Remove the resolved slot value from this frame's slot list.
///
/// Resolves the incoming key through the engine helper (one intercepted
/// call), then scans the dword array at field `0x1d4` for the first entry
/// equal to the answer, searching the `u16` entry count at field `0x1d8`.
/// A hit is closed by shifting the tail down one slot. The count is then
/// decremented unconditionally, even when the value was absent or the list
/// was already empty (wrapping from 0 to 0xffff), matching the original.
/// Always returns 0xffff.
export!(thiscall, rw_00c0f2b0(this: *mut u8, key: u32) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, key);
        let slots = *(this.add(0x1d4) as *const u32) as *mut u32;
        let count_cell = this.add(0x1d8) as *mut u16;
        let count = *count_cell as u32;
        let mut index = 0xffffu32;
        if count > 0 {
            let mut i = 0u32;
            loop {
                if *slots.add(i as usize) == answer {
                    index = i;
                    break;
                }
                i += 1;
                if i >= count {
                    break;
                }
            }
        }
        // The original guards the shift with a signed `index < count - 1`
        // compare; `index + 1 < count` is the same test for u16 counts.
        if index + 1 < count {
            let mut j = index;
            while j + 1 < count {
                *slots.add(j as usize) = *slots.add(j as usize + 1);
                j += 1;
            }
        }
        *count_cell = count.wrapping_sub(1) as u16;
        0xffff
    }
});
