// original: 0x00dff6aa PMDtoOffset
/// MSVC RTTI helper: member-pointer displacement to offset.
///
/// Returns the base when the displacement is negative, otherwise the base
/// plus the displacement plus the table entry found through it.
export!(cdecl, rw_00dff6aa(base: u32, pm: u32) -> u32 {
    unsafe {
        let obj = *(pm as *const u32);
        let disp = *((pm.wrapping_add(4)) as *const u32) as i32;
        if disp < 0 {
            obj
        } else {
            let entry = *((disp as u32).wrapping_add(base) as *const u32);
            let off = *((pm.wrapping_add(8)) as *const u32);
            obj.wrapping_add(disp as u32)
                .wrapping_add(*((entry.wrapping_add(off)) as *const u32))
        }
    }
});
