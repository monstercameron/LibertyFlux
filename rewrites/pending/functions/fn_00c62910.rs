// original: 0x00c62910 anim_apply_relocations
/// Relocation applier: rebases the two bound pointers through a range table.
///
/// The table holds a signed count at `+0x600` with parallel lower bounds,
/// upper bounds (`+0x200`) and deltas (`+0x400`). Each of the pointers at
/// `+0x40` and `+0x48` is increased by the delta of the first row whose
/// half-open range contains it (unsigned comparison). Returns the second
/// pointer's outcome: the rebased sum on a match, the count when no row
/// matches, or 0 for a non-positive count.
export!(thiscall, rw_00c62910(this: u32, table: u32) -> u32 {
    unsafe {
        let count = *((table + 0x600) as *const i32);
        if count > 0 {
            let mut current = *((this + 0x40) as *const u32);
            let mut i: i32 = 0;
            while i < count {
                let row = (i as u32) * 4;
                let lo = *((table + row) as *const u32);
                if current >= lo {
                    let hi = *((table + row + 0x200) as *const u32);
                    if current < hi {
                        let delta = *((table + row + 0x400) as *const u32);
                        current = current.wrapping_add(delta);
                        *((this + 0x40) as *mut u32) = current;
                        break;
                    }
                }
                i += 1;
            }
        }
        let count2 = *((table + 0x600) as *const i32);
        if count2 <= 0 {
            return 0;
        }
        let current = *((this + 0x48) as *const u32);
        let mut i: i32 = 0;
        while i < count2 {
            let row = (i as u32) * 4;
            let lo = *((table + row) as *const u32);
            if current >= lo {
                let hi = *((table + row + 0x200) as *const u32);
                if current < hi {
                    let delta = *((table + row + 0x400) as *const u32);
                    let sum = current.wrapping_add(delta);
                    *((this + 0x48) as *mut u32) = sum;
                    return sum;
                }
            }
            i += 1;
        }
        count2 as u32
    }
});
